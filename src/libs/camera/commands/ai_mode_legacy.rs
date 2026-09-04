// SPDX-License-Identifier: EUPL-1.2

use crate::AIMode;
use crate::libs::camera::command_legacy::command_legacy;
use crate::libs::errors::T4lError;

/// AI tracking for the OBSBOT Tiny 4K (#72).
///
/// The Tiny 4K does not understand the Tiny 2 AI-mode setting on selector 6. It
/// splits the same functionality over three legacy frames on the AI route `0xe3`:
///
/// - `0x3051` enables or disables the AI engine
/// - `0x3091` selects the framing mode
/// - `0x3067` selects a target, `0x3066` releases it
///
/// Enabling only the AI engine is not enough — the camera follows a person once a
/// target has been selected, which is what the vendor app's "lock" button does.
pub struct LegacyAiModeCommand;

impl LegacyAiModeCommand {
    const ROUTE_AI: u8 = 0xe3;
    const COMMAND_AI_ENABLED: [u8; 2] = [0x30, 0x51];
    const COMMAND_TRACKING_MODE: [u8; 2] = [0x30, 0x91];
    const COMMAND_TARGET_DESELECT: [u8; 2] = [0x30, 0x66];
    const COMMAND_TARGET_SELECT: [u8; 2] = [0x30, 0x67];

    /// Sub index of the AI enable flag inside the `0x3051` payload.
    const AI_ENABLED_SUB_INDEX: u8 = 0x00;
    /// The `0x3067` payload is a fixed byte, not a boolean.
    const TARGET_SELECT_PAYLOAD: u8 = 0x00;

    const TRACKING_MODE_STANDARD: u8 = 0x00;
    const TRACKING_MODE_HEADROOM: u8 = 0x01;

    /// Whether the Tiny 4K offers the given mode at all.
    ///
    /// The 4K knows standard, headroom and motion framing. It has none of the
    /// Tiny 2 modes such as desk, whiteboard or hand, and there is no motion
    /// mode in [`AIMode`] to map its third value to.
    pub fn supports(mode: AIMode) -> bool {
        matches!(
            mode,
            AIMode::NoTracking | AIMode::NormalTracking | AIMode::UpperBody
        )
    }

    /// Builds the frames for the given mode, to be sent in order.
    pub fn build(mode: AIMode) -> Result<Vec<[u8; 60]>, T4lError> {
        let tracking_mode = match mode {
            AIMode::NoTracking => {
                return Ok(vec![
                    command_legacy(Self::ROUTE_AI, Self::COMMAND_TARGET_DESELECT, 1, &[]),
                    command_legacy(
                        Self::ROUTE_AI,
                        Self::COMMAND_AI_ENABLED,
                        2,
                        &[Self::AI_ENABLED_SUB_INDEX, 0x00],
                    ),
                ]);
            }
            AIMode::NormalTracking => Self::TRACKING_MODE_STANDARD,
            AIMode::UpperBody => Self::TRACKING_MODE_HEADROOM,
            _ => return Err(T4lError::InvalidSetting),
        };

        Ok(vec![
            command_legacy(
                Self::ROUTE_AI,
                Self::COMMAND_AI_ENABLED,
                3,
                &[Self::AI_ENABLED_SUB_INDEX, 0x01],
            ),
            command_legacy(
                Self::ROUTE_AI,
                Self::COMMAND_TRACKING_MODE,
                4,
                &[tracking_mode],
            ),
            command_legacy(
                Self::ROUTE_AI,
                Self::COMMAND_TARGET_SELECT,
                5,
                &[Self::TARGET_SELECT_PAYLOAD],
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::{AIMode, LegacyAiModeCommand};
    use test_case::test_case;

    #[test]
    fn tracking_off_releases_the_target_and_disables_the_ai() {
        let frames = LegacyAiModeCommand::build(AIMode::NoTracking).unwrap();

        assert_eq!(
            frames.len(),
            2,
            "target is released before the AI is turned off"
        );
        assert_eq!(
            frames[0][9..12],
            [0xe3, 0x30, 0x66],
            "first frame releases the target"
        );
        assert_eq!(
            frames[1][9..14],
            [0xe3, 0x30, 0x51, 0x00, 0x00],
            "second frame disables the AI engine"
        );
    }

    #[test_case(AIMode::NormalTracking, 0x00; "normal tracking uses the standard framing")]
    #[test_case(AIMode::UpperBody, 0x01; "upper body uses the headroom framing")]
    fn tracking_on(mode: AIMode, expected_tracking_mode: u8) {
        let frames = LegacyAiModeCommand::build(mode).unwrap();

        assert_eq!(frames.len(), 3, "enable, mode and target selection");
        assert_eq!(
            frames[0][9..14],
            [0xe3, 0x30, 0x51, 0x00, 0x01],
            "first frame enables the AI engine"
        );
        assert_eq!(
            frames[1][9..13],
            [0xe3, 0x30, 0x91, expected_tracking_mode],
            "second frame selects the framing mode"
        );
        assert_eq!(
            frames[2][9..13],
            [0xe3, 0x30, 0x67, 0x00],
            "third frame selects a target so the camera starts following"
        );
    }

    #[test_case(AIMode::CloseUp; "close up")]
    #[test_case(AIMode::DeskMode; "desk mode")]
    #[test_case(AIMode::Whiteboard; "whiteboard")]
    #[test_case(AIMode::Unknown; "unknown")]
    fn modes_the_tiny_4k_does_not_have(mode: AIMode) {
        assert!(!LegacyAiModeCommand::supports(mode));
        assert!(LegacyAiModeCommand::build(mode).is_err());
    }

    #[test_case(AIMode::NoTracking; "no tracking")]
    #[test_case(AIMode::NormalTracking; "normal tracking")]
    #[test_case(AIMode::UpperBody; "upper body")]
    fn supported_modes_build(mode: AIMode) {
        assert!(LegacyAiModeCommand::supports(mode));
        assert!(LegacyAiModeCommand::build(mode).is_ok());
    }
}
