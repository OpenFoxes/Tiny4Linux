// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;
use crate::{TrackingSpeed, command02};

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

        Ok(command02()
            .function_group(FUNCTION_GROUP_TRACKING_SPEED)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .appendix(appendix)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{TrackingSpeed, TrackingSpeedCommand};
    use assertables::assert_ok;
    use test_case::test_case;

    #[test_case(TrackingSpeed::Standard, [0x20, 0x00], [0xab, 0xcb], [0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00]; "standard speed")]
    #[test_case(TrackingSpeed::Sport, [0x21, 0x00], [0xfa, 0x0e], [0x67, 0xfe, 0x02, 0x00, 0x00, 0x00]; "sport speed")]
    fn speed_mode(mode: TrackingSpeed, sequence_nr: [u8; 2], checksum: [u8; 2], command: [u8; 6]) {
        let speed_command_option = TrackingSpeedCommand::build(mode);

        assert_ok!(
            &speed_command_option,
            "tracking speed command should be built"
        );

        let speed_command = speed_command_option.unwrap();
        assert_eq!(
            speed_command[8..14],
            [0x0a, 0x04, 0xc4, 0x0c, 0x01, 0x00],
            "Function group should be set for sleep mode"
        );
        assert_eq!(
            speed_command[2..4],
            sequence_nr,
            "Sequence number should be set"
        );
        assert_eq!(speed_command[6..8], checksum, "Checksum should be set");
        assert_eq!(speed_command[14..20], command, "Command should be set");
    }
}
