// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;
use crate::{SleepMode, frame};

pub struct SleepCommand;

impl SleepCommand {
    const RECEIVER: u8 = 0x02;
    const COMMAND: u16 = 0xa0c2;

    pub fn build(mode: SleepMode, sequence_nr: u16) -> Result<[u8; 60], T4lError> {
        let value: u32 = match mode {
            SleepMode::Awake => 0,
            SleepMode::Sleep => 1,
            SleepMode::Unknown => return Err(T4lError::InvalidSetting),
        };

        Ok(frame()
            .receiver(Self::RECEIVER)
            .command(Self::COMMAND)
            .data(&value.to_le_bytes())
            .sequence_nr(sequence_nr)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{SleepCommand, SleepMode};
    use assertables::assert_err;
    use test_case::test_case;

    // Frames captured from the vendor software. Building with the captured sequence number
    // has to reproduce them byte for byte, which proves both checksums are computed correctly.
    #[test_case(SleepMode::Awake, 0x00a5, "aa25a5000c005fef0a02c2a00400be0700000000"; "awake command")]
    #[test_case(SleepMode::Sleep, 0x0042, "aa2542000c00ea630a02c2a00400bffb01000000"; "sleep command")]
    fn sleep_mode(mode: SleepMode, sequence_nr: u16, captured: &str) {
        let captured = hex::decode(captured).unwrap();

        let command = SleepCommand::build(mode, sequence_nr).unwrap();

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }

    #[test]
    fn unknown_sleep_mode() {
        assert_err!(SleepCommand::build(SleepMode::Unknown, 0));
    }
}
