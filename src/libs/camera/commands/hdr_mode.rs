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

#[cfg(test)]
mod tests {
    use crate::HdrModeCommand;
    use test_case::test_case;

    #[test_case(true, [0x01, 0x01, 0x01]; "HDR on")]
    #[test_case(false, [0x01, 0x01, 0x00]; "HDR off")]
    fn hdr_mode(mode: bool, expected: [u8; 3]) {
        assert_eq!(HdrModeCommand::build(mode), expected)
    }
}
