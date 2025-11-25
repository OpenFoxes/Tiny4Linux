// SPDX-License-Identifier: EUPL-1.2

pub struct HdrModeCommand;

impl HdrModeCommand {
    pub fn build(mode: bool) -> [u8; 3] {
        if mode {
            [0x01, 0x01, 0x01]
        } else {
            [0x01, 0x01, 0x00]
        }
    }
}
