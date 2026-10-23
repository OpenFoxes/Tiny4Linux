// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::enums::{
    AIMode, AutoSleepTime, FieldOfView, NoiseReduction, PickupDistance, SleepBackground, SleepMode,
    TrackingSpeed,
};

/// Decoded status block (unit 2, selector 6, GET_CUR).
///
/// The offsets of the fields below `hdr_on` were measured on an OBSBOT Tiny 2 Lite by
/// changing one setting at a time and diffing the block. Other models (e.g. the Tiny 2)
/// may place them differently.
pub struct CameraStatus {
    pub awake: SleepMode,
    pub ai_mode: AIMode,
    pub speed: TrackingSpeed,
    pub hdr_on: bool,
    pub auto_sleep: AutoSleepTime,
    pub sleep_background: SleepBackground,
    pub sleep_background_mirror: bool,
    pub field_of_view: FieldOfView,
    pub mirror: bool,
    /// Status LED, 0 off, 1-3 brightness. On the Tiny 2 Lite this is byte 0x21.
    pub led_brightness: u8,
    pub mic_in_sleep: bool,
    pub noise_reduction: NoiseReduction,
    pub auto_gain: bool,
    pub pickup_distance: PickupDistance,
    /// Gesture "target lock". The other gesture switches are not in the status block.
    pub gesture_target_lock: bool,
}

impl CameraStatus {
    pub fn decode(bytes: &[u8]) -> Self {
        CameraStatus {
            awake: Self::decode_sleep_mode(bytes),
            ai_mode: Self::decode_ai_mode(bytes),
            speed: Self::decode_tracking_speed(bytes),
            hdr_on: Self::decode_hdr_on(bytes),
            auto_sleep: Self::decode_auto_sleep(bytes),
            sleep_background: Self::decode_sleep_background(bytes),
            sleep_background_mirror: bytes[0x1d] != 0,
            field_of_view: Self::decode_field_of_view(bytes),
            mirror: bytes[0x13] != 0,
            led_brightness: bytes[0x21],
            mic_in_sleep: bytes[0x10] != 0,
            noise_reduction: Self::decode_noise_reduction(bytes),
            auto_gain: bytes[0x19] != 0,
            pickup_distance: Self::decode_pickup_distance(bytes),
            gesture_target_lock: bytes[0x01] != 0,
        }
    }

    fn decode_sleep_mode(bytes: &[u8]) -> SleepMode {
        match bytes[0x02] {
            0 => SleepMode::Awake,
            1 => SleepMode::Sleep,
            _ => SleepMode::Unknown,
        }
    }

    fn decode_ai_mode(bytes: &[u8]) -> AIMode {
        let m = bytes[0x18];
        let n = bytes[0x1c];

        match (m, n) {
            (0, 0) => AIMode::NoTracking,
            (2, 0) => AIMode::NormalTracking,
            (2, 1) => AIMode::UpperBody,
            (2, 2) => AIMode::CloseUp,
            (2, 3) => AIMode::Headless,
            (2, 4) => AIMode::LowerBody,
            (5, 0) => AIMode::DeskMode,
            (4, 0) => AIMode::Whiteboard,
            (6, 0) => AIMode::Hand,
            (1, 0) => AIMode::Group,
            (_, _) => AIMode::Unknown,
        }
    }

    fn decode_tracking_speed(bytes: &[u8]) -> TrackingSpeed {
        match bytes[0x21] {
            0 => TrackingSpeed::Standard,
            2 => TrackingSpeed::Sport,
            _ => TrackingSpeed::Standard,
        }
    }

    fn decode_hdr_on(bytes: &[u8]) -> bool {
        bytes[0x6] != 0
    }

    fn decode_auto_sleep(bytes: &[u8]) -> AutoSleepTime {
        AutoSleepTime(i16::from_le_bytes([bytes[0x0a], bytes[0x0b]]))
    }

    fn decode_field_of_view(bytes: &[u8]) -> FieldOfView {
        match bytes[0x11] {
            0 => FieldOfView::Wide,
            1 => FieldOfView::Medium,
            2 => FieldOfView::Narrow,
            _ => FieldOfView::Unknown,
        }
    }

    fn decode_noise_reduction(bytes: &[u8]) -> NoiseReduction {
        match bytes[0x08] {
            0 => NoiseReduction::Off,
            1 => NoiseReduction::Weak,
            2 => NoiseReduction::Medium,
            3 => NoiseReduction::Strong,
            _ => NoiseReduction::Unknown,
        }
    }

    /// The pickup distance sits in the low bits of byte 0x22, on top of a base of 0x10.
    fn decode_pickup_distance(bytes: &[u8]) -> PickupDistance {
        match bytes[0x22] & 0x0f {
            0 => PickupDistance::Near,
            1 => PickupDistance::Standard,
            2 => PickupDistance::Far,
            _ => PickupDistance::Unknown,
        }
    }

    fn decode_sleep_background(bytes: &[u8]) -> SleepBackground {
        match bytes[0x1b] {
            0 => SleepBackground::Black,
            1 => SleepBackground::Image,
            _ => SleepBackground::Unknown,
        }
    }

    pub fn default() -> Self {
        CameraStatus {
            awake: SleepMode::Unknown,
            ai_mode: AIMode::Unknown,
            speed: TrackingSpeed::Standard,
            hdr_on: false,
            auto_sleep: AutoSleepTime(0),
            sleep_background: SleepBackground::Unknown,
            sleep_background_mirror: false,
            field_of_view: FieldOfView::Unknown,
            mirror: false,
            led_brightness: 0,
            mic_in_sleep: false,
            noise_reduction: NoiseReduction::Unknown,
            auto_gain: false,
            pickup_distance: PickupDistance::Unknown,
            gesture_target_lock: false,
        }
    }
}

#[cfg(test)]
mod tests {
    mod unit {
        mod decode {
            use crate::{
                AIMode, AutoSleepTime, CameraStatus, FieldOfView, NoiseReduction, PickupDistance,
                SleepBackground, SleepMode, TrackingSpeed,
            };
            use test_case::test_case;

            const RAW_TEST_STRING: [u8; 57] = [0; 57];

            #[test_case(0x00, SleepMode::Awake; "awake")]
            #[test_case(0x01, SleepMode::Sleep; "sleep")]
            #[test_case(0x02, SleepMode::Unknown; "unknown")]
            fn sleep_mode(hex_value: u8, expected: SleepMode) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x02] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.awake);
            }

            #[test_case(0x00, 0x00, AIMode::NoTracking; "none")]
            #[test_case(0x02, 0x00, AIMode::NormalTracking; "normal")]
            #[test_case(0x02, 0x01, AIMode::UpperBody; "upper body")]
            #[test_case(0x02, 0x02, AIMode::CloseUp; "close up")]
            #[test_case(0x02, 0x03, AIMode::Headless; "headless")]
            #[test_case(0x02, 0x04, AIMode::LowerBody; "lower body")]
            #[test_case(0x05, 0x00, AIMode::DeskMode; "desk")]
            #[test_case(0x04, 0x00, AIMode::Whiteboard; "whiteboard")]
            #[test_case(0x06, 0x00, AIMode::Hand; "hand")]
            #[test_case(0x01, 0x00, AIMode::Group; "group")]
            #[test_case(0x17, 0x42, AIMode::Unknown; "unknown")]
            fn ai_mode(hex_value1: u8, hex_value2: u8, expected: AIMode) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x18] = hex_value1;
                test_string[0x1c] = hex_value2;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.ai_mode);
            }

            #[test_case(0x00, TrackingSpeed::Standard; "standard")]
            #[test_case(0x02, TrackingSpeed::Sport; "sport")]
            #[test_case(0x01, TrackingSpeed::Standard; "unknown defaults to standard")]
            fn tracking_speed(hex_value: u8, expected: TrackingSpeed) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x21] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.speed);
            }

            #[test_case(0x01, true; "on")]
            #[test_case(0x00, false; "off")]
            #[test_case(0x02, true; "unknown defaults to on")]
            fn hdr(hex_value: u8, expected: bool) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x06] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.hdr_on);
            }

            #[test_case([0x78, 0x00], 120; "2 minutes")]
            #[test_case([0x58, 0x02], 600; "10 minutes, little endian")]
            #[test_case([0x88, 0xff], -120; "off, the time is kept negated")]
            fn auto_sleep(hex_values: [u8; 2], expected: i16) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x0a..0x0c].copy_from_slice(&hex_values);

                let status = CameraStatus::decode(&test_string);

                assert_eq!(AutoSleepTime(expected), status.auto_sleep);
            }

            #[test_case(0x00, SleepBackground::Black; "black")]
            #[test_case(0x01, SleepBackground::Image; "image")]
            #[test_case(0x07, SleepBackground::Unknown; "unknown")]
            fn sleep_background(hex_value: u8, expected: SleepBackground) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x1b] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.sleep_background);
            }

            #[test_case(0x01, true; "mirrored")]
            #[test_case(0x00, false; "not mirrored")]
            fn sleep_background_mirror(hex_value: u8, expected: bool) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x1d] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.sleep_background_mirror);
            }

            #[test_case(0x00, FieldOfView::Wide; "wide")]
            #[test_case(0x01, FieldOfView::Medium; "medium")]
            #[test_case(0x02, FieldOfView::Narrow; "narrow")]
            #[test_case(0x03, FieldOfView::Unknown; "unknown")]
            fn field_of_view(hex_value: u8, expected: FieldOfView) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x11] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.field_of_view);
            }

            #[test_case(0x01, true; "mirrored")]
            #[test_case(0x00, false; "not mirrored")]
            fn mirror(hex_value: u8, expected: bool) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x13] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.mirror);
            }

            #[test_case(0x00; "off")]
            #[test_case(0x02; "medium")]
            #[test_case(0x03; "bright")]
            fn led_brightness(hex_value: u8) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x21] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(hex_value, status.led_brightness);
            }

            #[test_case(0x01, true; "on")]
            #[test_case(0x00, false; "off")]
            fn mic_in_sleep(hex_value: u8, expected: bool) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x10] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.mic_in_sleep);
            }

            #[test_case(0x00, NoiseReduction::Off; "off")]
            #[test_case(0x01, NoiseReduction::Weak; "weak")]
            #[test_case(0x02, NoiseReduction::Medium; "medium")]
            #[test_case(0x03, NoiseReduction::Strong; "strong")]
            #[test_case(0x04, NoiseReduction::Unknown; "unknown")]
            fn noise_reduction(hex_value: u8, expected: NoiseReduction) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x08] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.noise_reduction);
            }

            #[test_case(0x01, true; "on")]
            #[test_case(0x00, false; "off")]
            fn auto_gain(hex_value: u8, expected: bool) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x19] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.auto_gain);
            }

            // The camera reports the distance on top of a base of 0x10 (near = 0x10).
            #[test_case(0x10, PickupDistance::Near; "near")]
            #[test_case(0x11, PickupDistance::Standard; "standard")]
            #[test_case(0x12, PickupDistance::Far; "far")]
            #[test_case(0x13, PickupDistance::Unknown; "unknown")]
            fn pickup_distance(hex_value: u8, expected: PickupDistance) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x22] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.pickup_distance);
            }

            #[test_case(0x01, true; "on")]
            #[test_case(0x00, false; "off")]
            fn gesture_target_lock(hex_value: u8, expected: bool) {
                let mut test_string = RAW_TEST_STRING;
                test_string[0x01] = hex_value;

                let status = CameraStatus::decode(&test_string);

                assert_eq!(expected, status.gesture_target_lock);
            }

            #[test]
            fn status_defaults() {
                let default_status = CameraStatus::default();

                assert_eq!(default_status.awake, SleepMode::Unknown, "sleep is unknown");
                assert_eq!(
                    default_status.ai_mode,
                    AIMode::Unknown,
                    "tracking is unknown"
                );
                assert_eq!(
                    default_status.speed,
                    TrackingSpeed::Standard,
                    "speed is standard"
                );
                assert_eq!(default_status.hdr_on, false, "hdr is off");
            }
        }
    }

    mod integration {
        mod camera_status {
            use crate::{
                AIMode, AutoSleepTime, CameraStatus, FieldOfView, NoiseReduction, PickupDistance,
                SleepBackground, SleepMode, TrackingSpeed,
            };

            #[test]
            fn decode_status() {
                let data = [
                    0x27, 0x00, 0x00, 0x01, 0x42, 0x00, 0x01, 0x01, 0x01, 0x01, 0x88, 0xff, 0x00,
                    0x00, 0x01, 0x00, 0x00, 0x03, 0x00, 0x00, 0x01, 0x00, 0x21, 0x00, 0x02, 0x01,
                    0x03, 0x00, 0x01, 0x00, 0x00, 0x1e, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00,
                ];
                let status = CameraStatus::decode(&data);
                assert_eq!(status.awake, SleepMode::Awake);
                assert_eq!(status.hdr_on, true);
                assert_eq!(status.ai_mode, AIMode::UpperBody);
                assert_eq!(status.speed, TrackingSpeed::Sport);
            }

            /// Status block read from a Tiny 2 Lite in its factory defaults
            /// (auto-sleep 120 s, LED 3, wide FOV, weak noise reduction, auto gain on,
            /// pickup distance near, gesture target lock on).
            const TINY2_LITE_DEFAULTS: [u8; 60] = [
                0x27, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x78, 0x00, 0x00, 0x01,
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x21, 0x00, 0x00, 0x01, 0x03, 0x00,
                0x00, 0x00, 0x00, 0x1e, 0x00, 0x03, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ];

            #[test]
            fn decode_tiny2_lite_status() {
                let status = CameraStatus::decode(&TINY2_LITE_DEFAULTS);
                assert_eq!(status.awake, SleepMode::Awake);
                assert_eq!(status.hdr_on, false);
                assert_eq!(status.ai_mode, AIMode::NoTracking);
                assert_eq!(status.auto_sleep, AutoSleepTime(120));
                assert_eq!(status.sleep_background, SleepBackground::Black);
                assert_eq!(status.sleep_background_mirror, false);
                assert_eq!(status.field_of_view, FieldOfView::Wide);
                assert_eq!(status.mirror, false);
                assert_eq!(status.led_brightness, 3);
                assert_eq!(status.mic_in_sleep, false);
                assert_eq!(status.noise_reduction, NoiseReduction::Weak);
                assert_eq!(status.auto_gain, true);
                assert_eq!(status.pickup_distance, PickupDistance::Near);
                assert_eq!(status.gesture_target_lock, true);
            }
        }
    }
}
