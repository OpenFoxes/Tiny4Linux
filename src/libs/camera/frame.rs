// SPDX-License-Identifier: EUPL-1.2

use bon::builder;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Size of the buffer the camera expects on unit 2, selector 2.
pub const FRAME_SIZE: usize = 60;
const HEADER_SIZE: usize = 12;
const DATA_HEADER_SIZE: usize = 4;
/// Maximum payload that fits into one frame after both headers.
pub const MAX_FRAME_DATA: usize = FRAME_SIZE - HEADER_SIZE - DATA_HEADER_SIZE;

/// CRC-16/USB: reflected polynomial `0xA001`, init `0xFFFF`, final XOR `0xFFFF`.
///
/// The camera checks both the header and the data block of a frame with it.
/// First described in <https://github.com/OpenFoxes/Tiny4Linux/issues/171>.
pub fn crc16_usb(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xffff;
    for byte in data {
        crc ^= *byte as u16;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xa001
            } else {
                crc >> 1
            };
        }
    }
    crc ^ 0xffff
}

/// Builds a framed command for unit 2, selector 2.
///
/// Layout (all multi-byte fields little endian):
/// `aa 25 | seq:u16 | 0c 00 | header crc:u16 | 0a | receiver:u8 | command:u16 | data len:u16 | data crc:u16 | data`
///
/// The header CRC covers bytes 0..12 with its own field zeroed, the data CRC covers the
/// data length, the CRC field (zeroed) and the data. The rest of the buffer is zero padded.
///
/// # Panics
/// If `data` is longer than [`MAX_FRAME_DATA`] bytes.
#[builder(finish_fn = build)]
pub fn frame(receiver: u8, command: u16, data: &[u8], sequence_nr: u16) -> [u8; FRAME_SIZE] {
    assert!(
        data.len() <= MAX_FRAME_DATA,
        "frame data must not exceed {MAX_FRAME_DATA} bytes"
    );

    let mut buffer = [0u8; FRAME_SIZE];

    buffer[0..2].copy_from_slice(&[0xaa, 0x25]);
    buffer[2..4].copy_from_slice(&sequence_nr.to_le_bytes());
    buffer[4..6].copy_from_slice(&(HEADER_SIZE as u16).to_le_bytes());
    buffer[8] = 0x0a;
    buffer[9] = receiver;
    buffer[10..12].copy_from_slice(&command.to_le_bytes());
    let header_crc = crc16_usb(&buffer[0..HEADER_SIZE]);
    buffer[6..8].copy_from_slice(&header_crc.to_le_bytes());

    let data_end = HEADER_SIZE + DATA_HEADER_SIZE + data.len();
    buffer[12..14].copy_from_slice(&(data.len() as u16).to_le_bytes());
    buffer[16..data_end].copy_from_slice(data);
    let data_crc = crc16_usb(&buffer[HEADER_SIZE..data_end]);
    buffer[14..16].copy_from_slice(&data_crc.to_le_bytes());

    buffer
}

/// Sequence numbers for framed commands.
///
/// The camera drops a frame that repeats the sequence number of the previous one,
/// so every frame needs a fresh number. Seeding from the clock keeps two short-lived
/// processes (e.g. two CLI calls in a row) from starting with the same number.
pub struct FrameSequence(AtomicU16);

impl FrameSequence {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u16)
            .unwrap_or(0);
        Self(AtomicU16::new(seed))
    }

    pub fn next(&self) -> u16 {
        self.0.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
    }
}

impl Default for FrameSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    mod unit {
        use crate::libs::camera::frame::{FrameSequence, MAX_FRAME_DATA, crc16_usb, frame};

        #[test]
        fn crc16_usb_check_value() {
            // Standard check value of CRC-16/USB for the ASCII string "123456789".
            assert_eq!(crc16_usb(b"123456789"), 0xb4c8);
        }

        #[test]
        fn layout() {
            let command = frame()
                .receiver(0x04)
                .command(0x3244)
                .data(&[0x11, 0x22, 0x33])
                .sequence_nr(0x1234)
                .build();

            assert_eq!(command[0..2], [0xaa, 0x25], "frame id");
            assert_eq!(command[2..4], [0x34, 0x12], "sequence number little endian");
            assert_eq!(command[4..6], [0x0c, 0x00], "header length");
            assert_eq!(command[8], 0x0a, "fixed header byte");
            assert_eq!(command[9], 0x04, "receiver");
            assert_eq!(
                command[10..12],
                [0x44, 0x32],
                "full 16 bit command id little endian"
            );
            assert_eq!(command[12..14], [0x03, 0x00], "data length");
            assert_eq!(command[16..19], [0x11, 0x22, 0x33], "data");
            assert!(command[19..].iter().all(|b| *b == 0), "zero padding");
        }

        #[test]
        fn checksums_cover_header_and_data() {
            let command = frame()
                .receiver(0x04)
                .command(0x30c4)
                .data(&[0x01])
                .sequence_nr(7)
                .build();

            let mut header = command[0..12].to_vec();
            header[6..8].fill(0);
            assert_eq!(command[6..8], crc16_usb(&header).to_le_bytes());

            let mut data_block = command[12..17].to_vec();
            data_block[2..4].fill(0);
            assert_eq!(command[14..16], crc16_usb(&data_block).to_le_bytes());
        }

        #[test]
        fn sequence_changes_header_checksum_only() {
            let a = frame()
                .receiver(0x04)
                .command(0x30c4)
                .data(&[0x01])
                .sequence_nr(1)
                .build();
            let b = frame()
                .receiver(0x04)
                .command(0x30c4)
                .data(&[0x01])
                .sequence_nr(2)
                .build();

            assert_ne!(a[6..8], b[6..8], "header checksum depends on sequence");
            assert_eq!(a[14..16], b[14..16], "data checksum does not");
        }

        #[test]
        fn max_data_fits() {
            let data = [0xffu8; MAX_FRAME_DATA];
            let command = frame()
                .receiver(0x04)
                .command(0x0001)
                .data(&data)
                .sequence_nr(0)
                .build();
            assert_eq!(command[16..], data);
        }

        #[test]
        #[should_panic]
        fn too_much_data_panics() {
            let data = [0u8; MAX_FRAME_DATA + 1];
            frame()
                .receiver(0x04)
                .command(0x0001)
                .data(&data)
                .sequence_nr(0)
                .build();
        }

        #[test]
        fn sequence_never_repeats_consecutively() {
            let sequence = FrameSequence::new();
            let mut previous = sequence.next();
            for _ in 0..70_000 {
                let current = sequence.next();
                assert_ne!(current, previous);
                assert_eq!(current, previous.wrapping_add(1));
                previous = current;
            }
        }
    }
}
