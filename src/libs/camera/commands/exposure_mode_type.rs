// SPDX-License-Identifier: EUPL-1.2

use crate::Command02;
use crate::ExposureMode;

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

        Command02::new()
            .function_group(FUNCTION_GROUP_EXPOSURE_MODE_TYPE)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .build()
    }
}
