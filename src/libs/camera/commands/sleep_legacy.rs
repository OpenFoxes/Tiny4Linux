// SPDX-License-Identifier: EUPL-1.2

use crate::SleepMode;
use crate::libs::camera::command_legacy::command_legacy;
use crate::libs::errors::T4lError;

/// Sleep and wake for the OBSBOT Tiny 4K (#72).
///
/// The Tiny 4K accepts the Tiny 2 sleep frame on USB but ignores it, so it needs
/// its own command built from the legacy frame. `0x13c2` is the device run status;
/// value `1` runs the camera and `3` sends it to sleep.
pub struct LegacySleepCommand;

impl LegacySleepCommand {
    const ROUTE_CAMERA: u8 = 0xe1;
    const COMMAND_DEV_RUN_STATUS: [u8; 2] = [0x13, 0xc2];

    /// Sequence numbers of the frames this was reverse engineered from.
    /// The camera echoes them but does not validate them.
    const SEQUENCE_NR_AWAKE: u16 = 0x35;
    const SEQUENCE_NR_SLEEP: u16 = 0x34;

    /// Fixed first payload byte of a device run status frame.
    const DEV_RUN_STATUS_PAYLOAD_PREFIX: u8 = 0x01;

    const DEV_RUN_STATUS_RUN: u8 = 0x01;
    const DEV_RUN_STATUS_SLEEP: u8 = 0x03;

    pub fn build(mode: SleepMode) -> Result<[u8; 60], T4lError> {
        let (sequence_nr, dev_run_status) = match mode {
            SleepMode::Awake => (Self::SEQUENCE_NR_AWAKE, Self::DEV_RUN_STATUS_RUN),
            SleepMode::Sleep => (Self::SEQUENCE_NR_SLEEP, Self::DEV_RUN_STATUS_SLEEP),
            SleepMode::Unknown => return Err(T4lError::InvalidSetting),
        };

        Ok(command_legacy(
            Self::ROUTE_CAMERA,
            Self::COMMAND_DEV_RUN_STATUS,
            sequence_nr,
            &[Self::DEV_RUN_STATUS_PAYLOAD_PREFIX, dev_run_status],
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{LegacySleepCommand, SleepMode};
    use assertables::assert_ok;
    use test_case::test_case;

    #[test_case(SleepMode::Awake, 0x01; "awake sets the run status to 1")]
    #[test_case(SleepMode::Sleep, 0x03; "sleep sets the run status to 3")]
    fn sleep_mode(mode: SleepMode, expected_dev_run_status: u8) {
        let command_option = LegacySleepCommand::build(mode);

        assert_ok!(&command_option, "sleep mode command should be built");

        let command = command_option.unwrap();
        assert_eq!(
            command[8..14],
            [0x00, 0xe1, 0x13, 0xc2, 0x01, expected_dev_run_status],
            "Frame is routed to the camera and sets the device run status"
        );
    }

    #[test]
    fn unknown_sleep_mode() {
        assert!(
            LegacySleepCommand::build(SleepMode::Unknown).is_err(),
            "an unknown sleep mode is not a valid setting"
        );
    }
}
