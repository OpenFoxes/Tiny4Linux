// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::enums::{AIMode, SleepMode, TrackingSpeed};

pub struct CameraStatus {
    pub awake: SleepMode,
    pub ai_mode: AIMode,
    pub speed: TrackingSpeed,
    pub hdr_on: bool,
}

impl CameraStatus {
    pub fn decode(bytes: &[u8]) -> Self {
        CameraStatus {
            awake: Self::decode_sleep_mode(bytes),
            ai_mode: Self::decode_ai_mode(bytes),
            speed: Self::decode_tracking_speed(bytes),
            hdr_on: Self::decode_hdr_on(bytes),
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

    pub fn default() -> Self {
        CameraStatus {
            awake: SleepMode::Unknown,
            ai_mode: AIMode::Unknown,
            speed: TrackingSpeed::Standard,
            hdr_on: false,
        }
    }
}

#[cfg(test)]
mod tests {
    mod unit {
        mod decode {
            use crate::{AIMode, CameraStatus, SleepMode, TrackingSpeed};
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
            use crate::{AIMode, CameraStatus, SleepMode, TrackingSpeed};

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
        }
    }
}
