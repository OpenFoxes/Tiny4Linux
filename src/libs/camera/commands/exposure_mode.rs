// SPDX-License-Identifier: EUPL-1.2

use crate::ExposureMode;

pub struct ExposureModeCommand;

impl ExposureModeCommand {
    pub fn build(mode: ExposureMode) -> Option<[u8; 3]> {
        match mode {
            ExposureMode::Manual => None,
            ExposureMode::Global => Some([0x03, 0x01, 0x00]),
            ExposureMode::Face => Some([0x03, 0x01, 0x01]),
        }
    }
}

#[cfg(test)]
mod tests {
    mod unit {
        use crate::{ExposureMode, ExposureModeCommand};
        use assertables::{assert_none, assert_some};
        use test_case::test_case;

        #[test]
        fn manual_exposure_mode() {
            let exposure_command = ExposureModeCommand::build(ExposureMode::Manual);

            assert_none!(
                exposure_command,
                "manual exposure mode should not set a exposure mode command"
            )
        }

        #[test_case(ExposureMode::Global, [0x03, 0x01, 0x00]; "global automatic exposure should be set")]
        #[test_case(ExposureMode::Face, [0x03, 0x01, 0x01]; "face automatic exposure should be set")]
        fn automatic_exposure_modes(mode: ExposureMode, expected: [u8; 3]) {
            let exposure_command = ExposureModeCommand::build(mode);

            assert_some!(
                exposure_command,
                "automatic exposure mode should set a exposure mode command"
            );
            assert_eq!(
                exposure_command.unwrap(),
                expected,
                "correct command is expected"
            )
        }
    }
}
