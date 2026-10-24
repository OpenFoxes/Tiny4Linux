// SPDX-License-Identifier: EUPL-1.2

use crate::{ExposureMode, frame};

pub struct ExposureModeTypeCommand;

impl ExposureModeTypeCommand {
    const RECEIVER: u8 = 0x02;
    const COMMAND: u16 = 0x2982;

    pub fn build(mode: ExposureMode, sequence_nr: u16) -> [u8; 60] {
        let data: [u8; 5] = if mode == ExposureMode::Manual {
            [0x02, 0x04, 0x00, 0x00, 0x00]
        } else {
            [0x01, 0x32, 0x00, 0x00, 0x00]
        };

        frame()
            .receiver(Self::RECEIVER)
            .command(Self::COMMAND)
            .data(&data)
            .sequence_nr(sequence_nr)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExposureMode, ExposureModeTypeCommand};
    use test_case::test_case;

    // Captured frames, reproduced byte for byte when given the captured sequence number.
    #[test_case(ExposureMode::Manual, 0x0016, "aa2516000c0058910a0282290500b2af0204000000"; "manual exposure")]
    #[test_case(ExposureMode::Global, 0x0015, "aa2515000c00a89e0a0282290500f9270132000000"; "global automatic exposure")]
    #[test_case(ExposureMode::Face, 0x0015, "aa2515000c00a89e0a0282290500f9270132000000"; "face automatic exposure")]
    fn exposure_mode_type(mode: ExposureMode, sequence_nr: u16, captured: &str) {
        let captured = hex::decode(captured).unwrap();

        let command = ExposureModeTypeCommand::build(mode, sequence_nr);

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }
}
