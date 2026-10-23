// SPDX-License-Identifier: EUPL-1.2

use crate::frame;

/// Framed command (unit 2, selector 2, receiver 2) that resets the camera to its factory
/// settings. The camera restarts afterwards.
pub struct FactoryResetCommand;

impl FactoryResetCommand {
    const RECEIVER: u8 = 0x02;
    const COMMAND: u16 = 0xa802;

    pub fn build(sequence_nr: u16) -> [u8; 60] {
        frame()
            .receiver(Self::RECEIVER)
            .command(Self::COMMAND)
            .data(&[0x01])
            .sequence_nr(sequence_nr)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::FactoryResetCommand;

    #[test]
    fn factory_reset_matches_capture() {
        // Frame captured from OBSBOT Center (capture 16 of the protocol reference),
        // reproduced byte for byte when given the captured sequence number.
        let captured = hex::decode("aa250b000c00695e0a0202a8010027ff01").unwrap();

        let command = FactoryResetCommand::build(0x000b);

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }
}
