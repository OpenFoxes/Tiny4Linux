// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;

/// Short command (unit 2, selector 6) for the status LED: `1a 01 <0 off | 1..3 brightness>`.
pub struct LedCommand;

impl LedCommand {
    pub const MAX_BRIGHTNESS: u8 = 3;

    pub fn build(brightness: u8) -> Result<[u8; 3], T4lError> {
        if brightness > Self::MAX_BRIGHTNESS {
            return Err(T4lError::InvalidSetting);
        }
        Ok([0x1a, 0x01, brightness])
    }
}

#[cfg(test)]
mod tests {
    use crate::LedCommand;
    use assertables::assert_err;
    use test_case::test_case;

    #[test_case(0, [0x1a, 0x01, 0x00]; "off")]
    #[test_case(1, [0x1a, 0x01, 0x01]; "dim")]
    #[test_case(2, [0x1a, 0x01, 0x02]; "medium")]
    #[test_case(3, [0x1a, 0x01, 0x03]; "bright")]
    fn led(brightness: u8, expected: [u8; 3]) {
        assert_eq!(LedCommand::build(brightness).unwrap(), expected);
    }

    #[test]
    fn brightness_above_three_is_rejected() {
        assert_err!(LedCommand::build(4));
    }
}
