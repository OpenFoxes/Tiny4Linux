// SPDX-License-Identifier: EUPL-1.2

use crate::SleepBackground;
use crate::libs::errors::T4lError;

/// Short command (unit 2, selector 6) for what the camera shows while asleep:
/// `0e 02 00 <0 black | 1 built-in image>`.
pub struct SleepBackgroundCommand;

impl SleepBackgroundCommand {
    pub fn build(background: SleepBackground) -> Result<[u8; 4], T4lError> {
        let value = match background {
            SleepBackground::Black => 0x00,
            SleepBackground::Image => 0x01,
            SleepBackground::Unknown => return Err(T4lError::InvalidSetting),
        };
        Ok([0x0e, 0x02, 0x00, value])
    }
}

/// Short command (unit 2, selector 6) that mirrors the sleep background:
/// `0e 02 02 <0 off | 1 on>`.
pub struct SleepBackgroundMirrorCommand;

impl SleepBackgroundMirrorCommand {
    pub fn build(mirrored: bool) -> [u8; 4] {
        [0x0e, 0x02, 0x02, mirrored as u8]
    }
}

#[cfg(test)]
mod tests {
    use crate::{SleepBackground, SleepBackgroundCommand, SleepBackgroundMirrorCommand};
    use assertables::assert_err;
    use test_case::test_case;

    #[test_case(SleepBackground::Black, [0x0e, 0x02, 0x00, 0x00]; "black")]
    #[test_case(SleepBackground::Image, [0x0e, 0x02, 0x00, 0x01]; "built-in image")]
    fn sleep_background(background: SleepBackground, expected: [u8; 4]) {
        assert_eq!(SleepBackgroundCommand::build(background).unwrap(), expected);
    }

    #[test]
    fn unknown_sleep_background() {
        assert_err!(SleepBackgroundCommand::build(SleepBackground::Unknown));
    }

    #[test_case(true, [0x0e, 0x02, 0x02, 0x01]; "mirrored")]
    #[test_case(false, [0x0e, 0x02, 0x02, 0x00]; "not mirrored")]
    fn sleep_background_mirror(mirrored: bool, expected: [u8; 4]) {
        assert_eq!(SleepBackgroundMirrorCommand::build(mirrored), expected);
    }
}
