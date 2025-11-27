// SPDX-License-Identifier: EUPL-1.2

use crate::command02;
use crate::libs::errors::T4lError;

pub struct GotoPresetPositionCommand;

impl GotoPresetPositionCommand {
    pub fn build(preset_nr: i8) -> Result<[u8; 36], T4lError> {
        if preset_nr < 0 || preset_nr > 3 {
            return Err(T4lError::InvalidSetting);
        }

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
