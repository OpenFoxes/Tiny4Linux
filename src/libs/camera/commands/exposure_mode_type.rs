// SPDX-License-Identifier: EUPL-1.2

use crate::{ExposureMode, command02};

pub struct ExposureModeTypeCommand;

impl ExposureModeTypeCommand {
    pub fn build(mode: ExposureMode) -> [u8; 36] {
        const FUNCTION_GROUP_EXPOSURE_MODE_TYPE: [u8; 6] = [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00];

        let (sequence_nr, checksum, command) = if mode == ExposureMode::Manual {
            (
                [0x16, 0x00],
                [0x58, 0x91],
                [0xb2, 0xaf, 0x02, 0x04, 0x00, 0x00],
            )
        } else {
            (
                [0x15, 0x00],
                [0xa8, 0x9e],
                [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            )
        };

        command02()
            .function_group(FUNCTION_GROUP_EXPOSURE_MODE_TYPE)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExposureMode, ExposureModeTypeCommand};
    use test_case::test_case;

    #[test]
    fn manual_exposure_mode() {
        let exposure_command = ExposureModeTypeCommand::build(ExposureMode::Manual);

        assert_eq!(
            exposure_command[8..14],
            [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00],
            "Function group should be set for exposure mode type"
        );
        assert_eq!(
            exposure_command[2..4],
            [0x16, 0x00],
            "Sequence number should be set"
        );
        assert_eq!(
            exposure_command[6..8],
            [0x58, 0x91],
            "Checksum should be set"
        );
        assert_eq!(
            exposure_command[14..20],
            [0xb2, 0xaf, 0x02, 0x04, 0x00, 0x00],
            "Command should be set"
        );
    }

    #[test_case(ExposureMode::Global; "global automatic exposure should be set")]
    #[test_case(ExposureMode::Face; "face automatic exposure should be set")]
    fn automatic_exposure_modes(mode: ExposureMode) {
        let exposure_command = ExposureModeTypeCommand::build(mode);

        assert_eq!(
            exposure_command[8..14],
            [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00],
            "Function group should be set for exposure mode type"
        );
        assert_eq!(
            exposure_command[2..4],
            [0x15, 0x00],
            "Sequence number should be set"
        );
        assert_eq!(
            exposure_command[6..8],
            [0xa8, 0x9e],
            "Checksum should be set"
        );
        assert_eq!(
            exposure_command[14..20],
            [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            "Command should be set"
        );
    }
}
