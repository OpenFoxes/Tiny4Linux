// SPDX-License-Identifier: EUPL-1.2

//! Short commands (unit 2, selector 6) for the microphone, `<cmd> 01 <value>`.

use crate::libs::errors::T4lError;
use crate::{NoiseReduction, PickupDistance};

/// Keeps the microphone on while the camera is asleep: `13 01 <0 off | 1 on>`.
pub struct MicInSleepCommand;

impl MicInSleepCommand {
    pub fn build(enabled: bool) -> [u8; 3] {
        [0x13, 0x01, enabled as u8]
    }
}

/// Microphone noise reduction: `0a 01 <0 off | 1 weak | 2 medium | 3 strong>`.
pub struct NoiseReductionCommand;

impl NoiseReductionCommand {
    pub fn build(level: NoiseReduction) -> Result<[u8; 3], T4lError> {
        let value = match level {
            NoiseReduction::Off => 0x00,
            NoiseReduction::Weak => 0x01,
            NoiseReduction::Medium => 0x02,
            NoiseReduction::Strong => 0x03,
            NoiseReduction::Unknown => return Err(T4lError::InvalidSetting),
        };
        Ok([0x0a, 0x01, value])
    }
}

/// Automatic gain of the microphone: `17 01 <0 off | 1 on>`.
pub struct AutoGainCommand;

impl AutoGainCommand {
    pub fn build(enabled: bool) -> [u8; 3] {
        [0x17, 0x01, enabled as u8]
    }
}

/// Audio pickup distance: `1b 01 <0 near | 1 standard | 2 far>`.
pub struct PickupDistanceCommand;

impl PickupDistanceCommand {
    pub fn build(distance: PickupDistance) -> Result<[u8; 3], T4lError> {
        let value = match distance {
            PickupDistance::Near => 0x00,
            PickupDistance::Standard => 0x01,
            PickupDistance::Far => 0x02,
            PickupDistance::Unknown => return Err(T4lError::InvalidSetting),
        };
        Ok([0x1b, 0x01, value])
    }
}

/// Enables or disables the microphone: `1c 01 <0 off | 1 on>`.
///
/// Disabling it makes the camera restart (it re-enumerates on USB, about 18 s).
pub struct MicrophoneCommand;

impl MicrophoneCommand {
    pub fn build(enabled: bool) -> [u8; 3] {
        [0x1c, 0x01, enabled as u8]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AutoGainCommand, MicInSleepCommand, MicrophoneCommand, NoiseReduction,
        NoiseReductionCommand, PickupDistance, PickupDistanceCommand,
    };
    use assertables::assert_err;
    use test_case::test_case;

    #[test_case(true, [0x13, 0x01, 0x01]; "on")]
    #[test_case(false, [0x13, 0x01, 0x00]; "off")]
    fn mic_in_sleep(enabled: bool, expected: [u8; 3]) {
        assert_eq!(MicInSleepCommand::build(enabled), expected);
    }

    #[test_case(NoiseReduction::Off, [0x0a, 0x01, 0x00]; "off")]
    #[test_case(NoiseReduction::Weak, [0x0a, 0x01, 0x01]; "weak")]
    #[test_case(NoiseReduction::Medium, [0x0a, 0x01, 0x02]; "medium")]
    #[test_case(NoiseReduction::Strong, [0x0a, 0x01, 0x03]; "strong")]
    fn noise_reduction(level: NoiseReduction, expected: [u8; 3]) {
        assert_eq!(NoiseReductionCommand::build(level).unwrap(), expected);
    }

    #[test]
    fn unknown_noise_reduction() {
        assert_err!(NoiseReductionCommand::build(NoiseReduction::Unknown));
    }

    #[test_case(true, [0x17, 0x01, 0x01]; "on")]
    #[test_case(false, [0x17, 0x01, 0x00]; "off")]
    fn auto_gain(enabled: bool, expected: [u8; 3]) {
        assert_eq!(AutoGainCommand::build(enabled), expected);
    }

    #[test_case(PickupDistance::Near, [0x1b, 0x01, 0x00]; "near")]
    #[test_case(PickupDistance::Standard, [0x1b, 0x01, 0x01]; "standard")]
    #[test_case(PickupDistance::Far, [0x1b, 0x01, 0x02]; "far")]
    fn pickup_distance(distance: PickupDistance, expected: [u8; 3]) {
        assert_eq!(PickupDistanceCommand::build(distance).unwrap(), expected);
    }

    #[test]
    fn unknown_pickup_distance() {
        assert_err!(PickupDistanceCommand::build(PickupDistance::Unknown));
    }

    #[test_case(true, [0x1c, 0x01, 0x01]; "on")]
    #[test_case(false, [0x1c, 0x01, 0x00]; "off restarts the camera")]
    fn microphone(enabled: bool, expected: [u8; 3]) {
        assert_eq!(MicrophoneCommand::build(enabled), expected);
    }
}
