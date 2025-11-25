// SPDX-License-Identifier: EUPL-1.2

use crate::ExposureMode;

pub struct ExposureModeCommand;

impl ExposureModeCommand {
    pub fn build(mode: ExposureMode) -> Option<[u8; 3]> {
        match mode {
            ExposureMode::Manual => None,
            ExposureMode::Global => Some([0x03, 0x01, 0x00]),
            ExposureMode::Face => Some([0x03, 0x01, 0x01]),
        }
    }
}
