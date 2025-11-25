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
