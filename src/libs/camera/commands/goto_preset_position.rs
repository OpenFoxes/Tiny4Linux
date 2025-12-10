// SPDX-License-Identifier: EUPL-1.2

use crate::command02;
use crate::libs::errors::T4lError;

pub struct GotoPresetPositionCommand;

impl GotoPresetPositionCommand {
    pub fn build(preset_nr: i8) -> Result<[u8; 36], T4lError> {
        const FUNCTION_GROUP_PRESETS: [u8; 6] = [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00];

        let (sequence_nr, checksum, command) = match preset_nr {
            0 => (
                [0x20, 0x00],
                [0x6b, 0xdc],
                [0xd6, 0xfb, 0x00, 0x00, 0x00, 0x00],
            ),
            1 => (
                [0x1a, 0x00],
                [0x4b, 0x03],
                [0xeb, 0x2a, 0x01, 0x00, 0x00, 0x00],
            ),
            2 => (
                [0x26, 0x00],
                [0x8b, 0xc3],
                [0xaf, 0x19, 0x02, 0x00, 0x00, 0x00],
            ),
            _ => {
                println!("Invalid preset nr {}", preset_nr + 1);
                return Err(T4lError::InvalidSetting);
            }
        };

        Ok(command02()
            .function_group(FUNCTION_GROUP_PRESETS)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .appendix({
                let mut arr = [0u8; 16];
                for i in 0..4 {
                    arr[i * 4..(i + 1) * 4].copy_from_slice(&[0x00, 0x00, 0x80, 0x3f]);
                }
                arr
            })
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::GotoPresetPositionCommand;
    use assertables::{assert_err, assert_ok};
    use test_case::test_case;

    #[test_case(0, [0x20, 0x00], [0x6b, 0xdc], [0xd6, 0xfb, 0x00, 0x00, 0x00, 0x00]; "preset position 1 (id 0)")]
    #[test_case(1, [0x1a, 0x00], [0x4b, 0x03], [0xeb, 0x2a, 0x01, 0x00, 0x00, 0x00]; "preset position 2 (id 1)")]
    #[test_case(2, [0x26, 0x00], [0x8b, 0xc3], [0xaf, 0x19, 0x02, 0x00, 0x00, 0x00]; "preset position 3 (id 2)")]
    fn valid_preset_position(
        position: i8,
        sequence_nr: [u8; 2],
        checksum: [u8; 2],
        command: [u8; 6],
    ) {
        let position_command_option = GotoPresetPositionCommand::build(position);

        assert_ok!(
            &position_command_option,
            "preset position command should be built"
        );

        let position_command = position_command_option.unwrap();
        assert_eq!(
            position_command[8..14],
            [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00],
            "Function group should be set for preset position"
        );
        assert_eq!(
            position_command[2..4],
            sequence_nr,
            "Sequence number should be set"
        );
        assert_eq!(position_command[6..8], checksum, "Checksum should be set");
        assert_eq!(position_command[14..20], command, "Command should be set");
    }

    #[test_case(-1; "invalid preset position (lower than 0)")]
    #[test_case(3; "invalid preset position (greater than 2)")]
    #[test_case(i8::MIN; "invalid preset position (minimal number)")]
    #[test_case(i8::MAX - 1; "invalid preset position (maximal number - 1)")]
    fn invalid_preset_position(invalid_position: i8) {
        let position_command_option = GotoPresetPositionCommand::build(invalid_position);

        assert_err!(
            position_command_option,
            "invalid position should return an error"
        );
    }
}
