// SPDX-License-Identifier: EUPL-1.2

use crate::Command02;
use crate::TrackingSpeed;
use crate::libs::errors::T4lError;

pub struct TrackingSpeedCommand;

impl TrackingSpeedCommand {
    pub fn build(speed: TrackingSpeed) -> Result<[u8; 36], T4lError> {
        const FUNCTION_GROUP_TRACKING_SPEED: [u8; 6] = [0x0a, 0x04, 0xc4, 0x0c, 0x01, 0x00];

        let appendix: [u8; 16] = {
            let mut a = [0x00; 16];
            a[..4].fill(0x00);
            a
        };

        let (sequence_nr, checksum, command) = match speed {
            TrackingSpeed::Standard => (
                [0x20, 0x00],
                [0xab, 0xcb],
                [0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00],
            ),
            TrackingSpeed::Sport => (
                [0x21, 0x00],
                [0xfa, 0x0e],
                [0x67, 0xfe, 0x02, 0x00, 0x00, 0x00],
            ),
        };

        Ok(Command02::new()
            .function_group(FUNCTION_GROUP_TRACKING_SPEED)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .appendix(appendix)
            .build())
    }
}
