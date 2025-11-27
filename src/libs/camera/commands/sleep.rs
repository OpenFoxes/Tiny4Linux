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
