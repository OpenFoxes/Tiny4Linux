// SPDX-License-Identifier: EUPL-1.2

/// Builder for the *legacy* control frame used by the OBSBOT Tiny 4K (#72).
///
/// The Tiny 4K is architecturally the original OBSBOT Tiny, not a cut-down Tiny 2,
/// and it does not understand the Tiny 2 frames built by [`crate::command02`].
/// It accepts this older frame instead:
///
/// ```text
/// aa 00 <len> <type> <seq u16 BE> <crc u16 BE> 00 <route> <command u16 BE> <payload>
/// ```
///
/// - `len` is the frame length including the 12 byte header
/// - `type` is `0x10` for a SET (the camera answers with `0x11` / `0x13`)
/// - `crc` is CRC-16/USB over `frame[0..len]` with the checksum bytes themselves
///   zeroed, stored big-endian
/// - `route` addresses a subsystem: `0xe1` camera, `0xe2` gimbal, `0xe3` AI
///
/// The frame is zero-padded to the 60 byte extension unit buffer.
/// The camera does not validate the sequence number.
const FRAME_HEADER_LENGTH: u8 = 12;
const FRAME_TYPE_SET: u8 = 0x10;

/// CRC-16/USB: reflected polynomial 0xA001, init 0xFFFF, final xor 0xFFFF.
fn crc16_usb(bytes: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for byte in bytes {
        crc ^= *byte as u16;

        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xA001
            } else {
                crc >> 1
            };
        }
    }

    crc ^ 0xFFFF
}

/// Builds a legacy frame for the given route, command and payload.
///
/// # Parameters
/// - `route`: the addressed subsystem, e.g. `0xe1` for the camera itself
/// - `command`: the two byte command id, big-endian
/// - `sequence_nr`: echoed by the camera; it is not validated, so any value works
/// - `payload`: command arguments, may be empty and at most 48 bytes, since the
///   frame is padded to the 60 byte extension unit buffer
pub fn command_legacy(route: u8, command: [u8; 2], sequence_nr: u16, payload: &[u8]) -> [u8; 60] {
    debug_assert!(
        payload.len() <= 60 - FRAME_HEADER_LENGTH as usize,
        "payload does not fit into the extension unit buffer"
    );

    let length = FRAME_HEADER_LENGTH + payload.len() as u8;
    let mut frame = [0u8; 60];

    frame[0] = 0xaa;
    frame[1] = 0x00;
    frame[2] = length;
    frame[3] = FRAME_TYPE_SET;
    frame[4..6].copy_from_slice(&sequence_nr.to_be_bytes());
    // frame[6..8] stays zeroed while the checksum is calculated over it
    frame[8] = 0x00;
    frame[9] = route;
    frame[10..12].copy_from_slice(&command);
    frame[FRAME_HEADER_LENGTH as usize..length as usize].copy_from_slice(payload);

    let checksum = crc16_usb(&frame[..length as usize]);
    frame[6..8].copy_from_slice(&checksum.to_be_bytes());

    frame
}

#[cfg(test)]
mod tests {
    mod unit {
        use crate::libs::camera::command_legacy::{command_legacy, crc16_usb};

        #[test]
        fn crc_of_the_check_string() {
            // The CRC-16/USB check value for "123456789" is 0xB4C8.
            assert_eq!(crc16_usb(b"123456789"), 0xB4C8);
        }

        /// Both frames were captured from the vendor app driving a Tiny 4K and
        /// replayed successfully against the real camera.
        #[test]
        fn reproduces_the_captured_wake_frame() {
            let frame = command_legacy(0xe1, [0x13, 0xc2], 0x35, &[0x01, 0x01]);

            assert_eq!(
                frame[..14],
                [
                    0xaa, 0x00, 0x0e, 0x10, 0x00, 0x35, 0x2c, 0xf1, 0x00, 0xe1, 0x13, 0xc2, 0x01,
                    0x01
                ]
            );
        }

        #[test]
        fn reproduces_the_captured_sleep_frame() {
            let frame = command_legacy(0xe1, [0x13, 0xc2], 0x34, &[0x01, 0x03]);

            assert_eq!(
                frame[..14],
                [
                    0xaa, 0x00, 0x0e, 0x10, 0x00, 0x34, 0x7d, 0x7d, 0x00, 0xe1, 0x13, 0xc2, 0x01,
                    0x03
                ]
            );
        }

        #[test]
        fn pads_the_frame_to_the_full_buffer() {
            let frame = command_legacy(0xe3, [0x30, 0x3a], 0x01, &[]);

            assert_eq!(frame.len(), 60, "frame fills the extension unit buffer");
            assert_eq!(frame[2], 0x0c, "a frame without payload is 12 bytes long");
            assert!(
                frame[12..].iter().all(|byte| *byte == 0),
                "everything after the frame is zero padding"
            );
        }
    }
}
