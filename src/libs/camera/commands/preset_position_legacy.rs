// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::command_legacy::command_legacy;
use crate::libs::errors::T4lError;

/// A preset position as the OBSBOT Tiny 4K stores it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PresetPosition {
    pub slot: u8,
    /// Pitch in hundredths of a degree.
    pub pitch: i16,
    /// Yaw in hundredths of a degree.
    pub yaw: i16,
    /// Zoom factor in hundredths, so `100` is 1.00x.
    pub zoom: u16,
}

/// Preset positions for the OBSBOT Tiny 4K (#72).
///
/// The Tiny 4K has no "recall preset" command. It stores the positions itself and
/// hands the whole table out in one read on extension unit 2, selector 7:
///
/// ```text
/// <count> then <count> records of  <slot> 00 00 <int16 pitch> <int16 yaw> <u16 zoom>
/// ```
///
/// A recall is then two frames: release the tracked target, because the AI would
/// otherwise immediately steer the gimbal back, and move to the stored angles.
pub struct LegacyPresetPositionCommand;

impl LegacyPresetPositionCommand {
    const ROUTE_AI: u8 = 0xe3;
    const ROUTE_GIMBAL: u8 = 0xe2;
    const COMMAND_TARGET_DESELECT: [u8; 2] = [0x30, 0x66];
    const COMMAND_SET_SPEED_POSITION: [u8; 2] = [0x20, 0x06];

    const RECORD_LENGTH: usize = 9;

    /// Speed the gimbal travels with, in hundredths of a degree per second.
    ///
    /// The vendor app varies this per recall; a fixed moderate speed reproduces the
    /// same movement.
    const SPEED: i16 = 2000;

    /// Reads the preset table out of a selector 7 response.
    pub fn parse(buffer: &[u8]) -> Vec<PresetPosition> {
        let count = buffer.first().copied().unwrap_or(0) as usize;
        let available = buffer.len().saturating_sub(1) / Self::RECORD_LENGTH;

        (0..count.min(available))
            .map(|index| {
                let record = &buffer[1 + index * Self::RECORD_LENGTH..][..Self::RECORD_LENGTH];

                PresetPosition {
                    slot: record[0],
                    pitch: i16::from_le_bytes([record[3], record[4]]),
                    yaw: i16::from_le_bytes([record[5], record[6]]),
                    zoom: u16::from_le_bytes([record[7], record[8]]),
                }
            })
            .collect()
    }

    /// Picks the preset stored in the given slot.
    pub fn find(buffer: &[u8], slot: u8) -> Result<PresetPosition, T4lError> {
        Self::parse(buffer)
            .into_iter()
            .find(|preset| preset.slot == slot)
            .ok_or(T4lError::UnsupportedIntValue(
                "preset position".to_string(),
                slot as i32,
            ))
    }

    /// Builds the frames that move the gimbal to a stored preset, to be sent in order.
    pub fn build(preset: &PresetPosition) -> Vec<[u8; 60]> {
        let mut payload = Vec::with_capacity(12);
        for value in [0, 0, Self::SPEED, 0, preset.pitch, preset.yaw] {
            payload.extend_from_slice(&value.to_le_bytes());
        }

        vec![
            command_legacy(Self::ROUTE_AI, Self::COMMAND_TARGET_DESELECT, 6, &[]),
            command_legacy(
                Self::ROUTE_GIMBAL,
                Self::COMMAND_SET_SPEED_POSITION,
                7,
                &payload,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{LegacyPresetPositionCommand, PresetPosition};

    /// Read from a real Tiny 4K: slot 0 was stored from the vendor app, slots 1 and 2
    /// are untouched and therefore identical.
    const TABLE: [u8; 28] = [
        0x03, 0x01, 0x00, 0x00, 0xf7, 0xff, 0xfc, 0xff, 0x64, 0x00, 0x02, 0x00, 0x00, 0xf7, 0xff,
        0xfc, 0xff, 0x64, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xfe, 0x44, 0x08, 0x64, 0x00,
    ];

    #[test]
    fn parses_every_slot() {
        let presets = LegacyPresetPositionCommand::parse(&TABLE);

        assert_eq!(
            presets.len(),
            3,
            "the leading byte is the number of presets"
        );
        assert_eq!(
            presets[2],
            PresetPosition {
                slot: 0,
                pitch: -318,
                yaw: 2116,
                zoom: 100
            },
            "slot 0 keeps the position stored from the vendor app"
        );
        assert_eq!(presets[0].slot, 1);
        assert_eq!(presets[1].slot, 2);
        assert_eq!(
            (presets[0].pitch, presets[0].yaw, presets[0].zoom),
            (presets[1].pitch, presets[1].yaw, presets[1].zoom),
            "untouched slots hold the same default position, only the slot number differs"
        );
    }

    #[test]
    fn finds_a_slot_by_number() {
        let preset = LegacyPresetPositionCommand::find(&TABLE, 0).unwrap();

        assert_eq!(preset.yaw, 2116);
        assert!(
            LegacyPresetPositionCommand::find(&TABLE, 7).is_err(),
            "a slot the camera does not have is an error, not a silent no-op"
        );
    }

    #[test]
    fn truncated_or_empty_tables_do_not_panic() {
        assert!(LegacyPresetPositionCommand::parse(&[]).is_empty());
        assert!(LegacyPresetPositionCommand::parse(&[0x00]).is_empty());
        assert_eq!(
            LegacyPresetPositionCommand::parse(&[0x09, 0x01, 0x00, 0x00]).len(),
            0,
            "a count larger than the buffer is clamped"
        );
    }

    #[test]
    fn recall_releases_the_target_before_moving() {
        let preset = LegacyPresetPositionCommand::find(&TABLE, 0).unwrap();
        let frames = LegacyPresetPositionCommand::build(&preset);

        assert_eq!(frames.len(), 2);
        assert_eq!(
            frames[0][9..12],
            [0xe3, 0x30, 0x66],
            "the AI would steer the gimbal straight back otherwise"
        );
        assert_eq!(frames[1][9..12], [0xe2, 0x20, 0x06], "absolute move");
        assert_eq!(
            frames[1][12..24],
            [
                0x00, 0x00, 0x00, 0x00, 0xd0, 0x07, 0x00, 0x00, 0xc2, 0xfe, 0x44, 0x08
            ],
            "three speeds then three angles, little endian"
        );
    }
}
