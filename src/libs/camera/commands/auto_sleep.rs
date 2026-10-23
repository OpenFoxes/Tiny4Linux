// SPDX-License-Identifier: EUPL-1.2

use crate::AutoSleepTime;

/// Short command (unit 2, selector 6) that sets the auto-sleep time.
///
/// `0b 02 <i16 seconds, little endian>`; a negative time turns auto-sleep off.
pub struct AutoSleepCommand;

impl AutoSleepCommand {
    pub fn build(time: AutoSleepTime) -> [u8; 4] {
        let [low, high] = time.0.to_le_bytes();
        [0x0b, 0x02, low, high]
    }
}

#[cfg(test)]
mod tests {
    use crate::{AutoSleepCommand, AutoSleepTime};
    use test_case::test_case;

    #[test_case(30, [0x0b, 0x02, 0x1e, 0x00]; "30 seconds")]
    #[test_case(120, [0x0b, 0x02, 0x78, 0x00]; "2 minutes")]
    #[test_case(600, [0x0b, 0x02, 0x58, 0x02]; "10 minutes, little endian")]
    #[test_case(-120, [0x0b, 0x02, 0x88, 0xff]; "off after 2 minutes, as OBSBOT Center sends it")]
    fn auto_sleep(seconds: i16, expected: [u8; 4]) {
        assert_eq!(AutoSleepCommand::build(AutoSleepTime(seconds)), expected);
    }
}
