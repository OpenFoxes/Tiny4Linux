// SPDX-License-Identifier: EUPL-1.2

use crate::AIMode;
use crate::libs::errors::T4lError;

pub struct AIModeCommand;

impl AIModeCommand {
    pub fn build(mode: AIMode) -> Result<[u8; 4], T4lError> {
        Ok(match mode {
            AIMode::NoTracking => [0x16, 0x02, 0x00, 0x00],
            AIMode::NormalTracking => [0x16, 0x02, 0x02, 0x00],
            AIMode::UpperBody => [0x16, 0x02, 0x02, 0x01],
            AIMode::DeskMode => [0x16, 0x02, 0x05, 0x00],
            AIMode::Whiteboard => [0x16, 0x02, 0x04, 0x00],
            AIMode::Group => [0x16, 0x02, 0x01, 0x00],
            AIMode::Hand => [0x16, 0x02, 0x03, 0x00],
            AIMode::CloseUp => [0x16, 0x02, 0x02, 0x02],
            AIMode::Headless => [0x16, 0x02, 0x02, 0x03],
            AIMode::LowerBody => [0x16, 0x02, 0x02, 0x04],
            AIMode::Unknown => [0x16, 0x02, 0x00, 0x00],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{AIMode, AIModeCommand};
    use test_case::test_case;

    #[test_case(AIMode::NoTracking, [0x16, 0x02, 0x00, 0x00]; "no tracking")]
    #[test_case(AIMode::NormalTracking, [0x16, 0x02, 0x02, 0x00]; "normal tracking")]
    #[test_case(AIMode::UpperBody, [0x16, 0x02, 0x02, 0x01]; "upper body")]
    #[test_case(AIMode::DeskMode, [0x16, 0x02, 0x05, 0x00]; "desk mode")]
    #[test_case(AIMode::Whiteboard, [0x16, 0x02, 0x04, 0x00]; "whiteboard")]
    #[test_case(AIMode::Group, [0x16, 0x02, 0x01, 0x00]; "group")]
    #[test_case(AIMode::Hand, [0x16, 0x02, 0x03, 0x00]; "hand")]
    #[test_case(AIMode::CloseUp, [0x16, 0x02, 0x02, 0x02]; "close up")]
    #[test_case(AIMode::Headless, [0x16, 0x02, 0x02, 0x03]; "headless")]
    #[test_case(AIMode::LowerBody, [0x16, 0x02, 0x02, 0x04]; "lower body")]
    #[test_case(AIMode::Unknown, [0x16, 0x02, 0x00, 0x00]; "unknown")]
    fn ai_mode(mode: AIMode, expected: [u8; 4]) {
        assert_eq!(AIModeCommand::build(mode).unwrap(), expected)
    }
}
