// SPDX-License-Identifier: EUPL-1.2

//! Framed commands (unit 2, selector 2, receiver 4) for the gesture control.
//!
//! Only "target lock" is reported back in the status block; the camera does not report
//! the other switches or the zoom ratio.

use crate::libs::errors::T4lError;
use crate::{Gesture, frame};

const RECEIVER: u8 = 0x04;

/// Turns a gesture on or off, data `u8` 0/1.
pub struct GestureCommand;

impl GestureCommand {
    pub fn build(gesture: Gesture, enabled: bool, sequence_nr: u16) -> [u8; 60] {
        let command = match gesture {
            Gesture::TargetLock => 0x30c4,
            Gesture::Zoom => 0x3144,
            Gesture::DynamicZoom => 0x3344,
            Gesture::DirectionChange => 0x33c4,
        };

        frame()
            .receiver(RECEIVER)
            .command(command)
            .data(&[enabled as u8])
            .sequence_nr(sequence_nr)
            .build()
    }
}

/// Sets how far the zoom gesture zooms in, data `f32` little endian.
pub struct GestureZoomRatioCommand;

impl GestureZoomRatioCommand {
    pub const MIN_RATIO: f32 = 1.0;
    pub const MAX_RATIO: f32 = 4.0;
    const COMMAND: u16 = 0x3244;

    pub fn build(ratio: f32, sequence_nr: u16) -> Result<[u8; 60], T4lError> {
        if !(Self::MIN_RATIO..=Self::MAX_RATIO).contains(&ratio) {
            return Err(T4lError::InvalidSetting);
        }

        Ok(frame()
            .receiver(RECEIVER)
            .command(Self::COMMAND)
            .data(&ratio.to_le_bytes())
            .sequence_nr(sequence_nr)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Gesture, GestureCommand, GestureZoomRatioCommand};
    use assertables::assert_err;
    use test_case::test_case;

    // Frames captured from OBSBOT Center (capture 04 of the protocol reference),
    // reproduced byte for byte when given the captured sequence number.
    #[test_case(Gesture::TargetLock, true, 0x004d, "aa254d000c00399e0a04c430010027ff01"; "target lock on")]
    #[test_case(Gesture::TargetLock, false, 0x0023, "aa2523000c005bd50a04c4300100e63f00"; "target lock off")]
    #[test_case(Gesture::Zoom, true, 0x004c, "aa254c000c00c85b0a044431010027ff01"; "zoom on")]
    #[test_case(Gesture::Zoom, false, 0x0024, "aa2524000c004a0f0a0444310100e63f00"; "zoom off")]
    #[test_case(Gesture::DynamicZoom, true, 0x004b, "aa254b000c00f8400a044433010027ff01"; "dynamic zoom on")]
    #[test_case(Gesture::DynamicZoom, false, 0x0048, "aa2548000c00084f0a0444330100e63f00"; "dynamic zoom off")]
    #[test_case(Gesture::DirectionChange, true, 0x0049, "aa2549000c00384a0a04c433010027ff01"; "direction change on")]
    #[test_case(Gesture::DirectionChange, false, 0x004a, "aa254a000c00c8450a04c4330100e63f00"; "direction change off")]
    fn gesture(gesture: Gesture, enabled: bool, sequence_nr: u16, captured: &str) {
        let captured = hex::decode(captured).unwrap();

        let command = GestureCommand::build(gesture, enabled, sequence_nr);

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }

    #[test]
    fn zoom_ratio_matches_capture() {
        let captured = hex::decode("aa2556000c00a9ef0a0444320400bff700000040").unwrap();

        let command = GestureZoomRatioCommand::build(2.0, 0x0056).unwrap();

        assert_eq!(command[..captured.len()], captured[..]);
        assert!(command[captured.len()..].iter().all(|b| *b == 0));
    }

    #[test_case(1.0, [0x00, 0x00, 0x80, 0x3f]; "minimum")]
    #[test_case(1.5, [0x00, 0x00, 0xc0, 0x3f]; "fraction")]
    #[test_case(4.0, [0x00, 0x00, 0x80, 0x40]; "maximum")]
    fn zoom_ratio_is_f32_little_endian(ratio: f32, expected: [u8; 4]) {
        let command = GestureZoomRatioCommand::build(ratio, 0).unwrap();

        assert_eq!(command[12..14], [0x04, 0x00], "four data bytes");
        assert_eq!(command[16..20], expected);
    }

    #[test_case(0.99; "below minimum")]
    #[test_case(4.01; "above maximum")]
    #[test_case(f32::NAN; "not a number")]
    fn invalid_zoom_ratio(ratio: f32) {
        assert_err!(GestureZoomRatioCommand::build(ratio, 0));
    }
}
