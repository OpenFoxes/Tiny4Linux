// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;
use crate::{TrackingSpeed, frame};

pub struct TrackingSpeedCommand;

impl TrackingSpeedCommand {
    const RECEIVER: u8 = 0x04;
    const COMMAND: u16 = 0x0cc4;

    pub fn build(speed: TrackingSpeed, sequence_nr: u16) -> Result<[u8; 60], T4lError> {
        let value: u8 = match speed {
            TrackingSpeed::Standard => 0,
            TrackingSpeed::Sport => 2,
        };

        Ok(frame()
            .receiver(Self::RECEIVER)
            .command(Self::COMMAND)
            .data(&[value])
            .sequence_nr(sequence_nr)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{TrackingSpeed, TrackingSpeedCommand};
    use test_case::test_case;

    // Captured frames, reproduced byte for byte when given the captured sequence number.
    #[test_case(TrackingSpeed::Standard, 0x0020, "aa2520000c00abcb0a04c40c0100e63f00"; "standard speed")]
    #[test_case(TrackingSpeed::Sport, 0x0021, "aa2521000c00fa0e0a04c40c010067fe02"; "sport speed")]
    fn speed_mode(speed: TrackingSpeed, sequence_nr: u16, captured: &str) {
        let captured = hex::decode(captured).unwrap();

        let command = TrackingSpeedCommand::build(speed, sequence_nr).unwrap();

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }
}
