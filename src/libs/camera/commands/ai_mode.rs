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
