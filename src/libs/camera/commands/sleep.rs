// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;
use crate::{SleepMode, command02};

pub struct SleepCommand;

impl SleepCommand {
    pub fn build(mode: SleepMode) -> Result<[u8; 36], T4lError> {
        if mode == SleepMode::Unknown {
            return Err(T4lError::InvalidSetting);
        }

        const FUNCTION_GROUP_SLEEP: [u8; 6] = [0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00];

        let (sequence_nr, checksum, command) = match mode {
            SleepMode::Awake => (
                [0xa5, 0x00],
                [0x5f, 0xef],
                [0xbe, 0x07, 0x00, 0x00, 0x00, 0x00],
            ),
            SleepMode::Sleep => (
                [0x42, 0x00],
                [0xea, 0x63],
                [0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00],
            ),
            SleepMode::Unknown => panic!(),
        };

        Ok(command02()
            .function_group(FUNCTION_GROUP_SLEEP)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{SleepCommand, SleepMode};
    use assertables::assert_ok;
    use test_case::test_case;

    #[test_case(SleepMode::Awake, [0xa5, 0x00], [0x5f, 0xef], [0xbe, 0x07, 0x00, 0x00, 0x00, 0x00]; "awake command")]
    #[test_case(SleepMode::Sleep, [0x42, 0x00], [0xea, 0x63], [0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00]; "sleep command")]
    fn sleep_mode(mode: SleepMode, sequence_nr: [u8; 2], checksum: [u8; 2], command: [u8; 6]) {
        let sleep_command_option = SleepCommand::build(mode);

        assert_ok!(&sleep_command_option, "sleep mode command should be built");

        let sleep_command = sleep_command_option.unwrap();
        assert_eq!(
            sleep_command[8..14],
            [0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00],
            "Function group should be set for sleep mode"
        );
        assert_eq!(
            sleep_command[2..4],
            sequence_nr,
            "Sequence number should be set"
        );
        assert_eq!(sleep_command[6..8], checksum, "Checksum should be set");
        assert_eq!(sleep_command[14..20], command, "Command should be set");
    }

    #[test]
    #[should_panic]
    fn unknown_sleep_mode() {
        SleepCommand::build(SleepMode::Unknown)
            .expect("Should panic because of unknown sleep mode");
    }
}
