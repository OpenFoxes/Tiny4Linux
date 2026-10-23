// SPDX-License-Identifier: EUPL-1.2

use crate::frame;
use crate::libs::errors::T4lError;

pub struct GotoPresetPositionCommand;

impl GotoPresetPositionCommand {
    const RECEIVER: u8 = 0x04;
    const COMMAND: u16 = 0x39c4;

    pub fn build(preset_nr: i8, sequence_nr: u16) -> Result<[u8; 60], T4lError> {
        if !(0..=2).contains(&preset_nr) {
            println!("Invalid preset nr {}", preset_nr as i16 + 1);
            return Err(T4lError::InvalidSetting);
        }

        // preset index as u32, followed by four f32 1.0 values as sent by the vendor software
        let mut data = [0u8; 20];
        data[0..4].copy_from_slice(&(preset_nr as u32).to_le_bytes());
        for i in 1..5 {
            data[i * 4..(i + 1) * 4].copy_from_slice(&1.0f32.to_le_bytes());
        }

        Ok(frame()
            .receiver(Self::RECEIVER)
            .command(Self::COMMAND)
            .data(&data)
            .sequence_nr(sequence_nr)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::GotoPresetPositionCommand;
    use assertables::assert_err;
    use test_case::test_case;

    // Captured frames, reproduced byte for byte when given the captured sequence number.
    #[test_case(0, 0x0020, "aa2520000c006bdc0a04c4391400d6fb000000000000803f0000803f0000803f0000803f"; "preset position 1 (id 0)")]
    #[test_case(1, 0x001a, "aa251a000c004b030a04c4391400eb2a010000000000803f0000803f0000803f0000803f"; "preset position 2 (id 1)")]
    #[test_case(2, 0x0026, "aa2526000c008bc30a04c4391400af19020000000000803f0000803f0000803f0000803f"; "preset position 3 (id 2)")]
    fn valid_preset_position(position: i8, sequence_nr: u16, captured: &str) {
        let captured = hex::decode(captured).unwrap();

        let command = GotoPresetPositionCommand::build(position, sequence_nr).unwrap();

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }

    #[test_case(-1; "invalid preset position (lower than 0)")]
    #[test_case(3; "invalid preset position (greater than 2)")]
    #[test_case(i8::MIN; "invalid preset position (minimal number)")]
    #[test_case(i8::MAX - 1; "invalid preset position (maximal number - 1)")]
    fn invalid_preset_position(invalid_position: i8) {
        assert_err!(
            GotoPresetPositionCommand::build(invalid_position, 0),
            "invalid position should return an error"
        );
    }
}
