// SPDX-License-Identifier: EUPL-1.2

use crate::frame;

/// Framed command (unit 2, selector 2) that turns the view and the gimbal upside down,
/// for a camera mounted upside down. The camera does not report this setting back.
pub struct InvertCommand;

impl InvertCommand {
    const RECEIVER: u8 = 0x04;
    const COMMAND: u16 = 0x3b84;

    pub fn build(inverted: bool, sequence_nr: u16) -> [u8; 60] {
        frame()
            .receiver(Self::RECEIVER)
            .command(Self::COMMAND)
            .data(&[inverted as u8])
            .sequence_nr(sequence_nr)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::InvertCommand;
    use test_case::test_case;

    // Frames captured from OBSBOT Center (capture 05 of the protocol reference),
    // reproduced byte for byte when given the captured sequence number.
    #[test_case(true, 0x005b, "aa255b000c00a8130a04843b010027ff01"; "inverted")]
    #[test_case(false, 0x005c, "aa255c000c0019c90a04843b0100e63f00"; "not inverted")]
    fn invert(inverted: bool, sequence_nr: u16, captured: &str) {
        let captured = hex::decode(captured).unwrap();

        let command = InvertCommand::build(inverted, sequence_nr);

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }
}
