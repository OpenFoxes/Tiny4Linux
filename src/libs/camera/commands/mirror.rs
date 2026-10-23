// SPDX-License-Identifier: EUPL-1.2

/// Short command (unit 2, selector 6) that mirrors the image: `14 01 <0 off | 1 on>`.
pub struct MirrorCommand;

impl MirrorCommand {
    pub fn build(mirrored: bool) -> [u8; 3] {
        [0x14, 0x01, mirrored as u8]
    }
}

#[cfg(test)]
mod tests {
    use crate::MirrorCommand;
    use test_case::test_case;

    #[test_case(true, [0x14, 0x01, 0x01]; "mirrored")]
    #[test_case(false, [0x14, 0x01, 0x00]; "not mirrored")]
    fn mirror(mirrored: bool, expected: [u8; 3]) {
        assert_eq!(MirrorCommand::build(mirrored), expected);
    }
}
