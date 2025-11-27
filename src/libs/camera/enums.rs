// SPDX-License-Identifier: EUPL-1.2

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SleepMode {
    Awake,
    Sleep,
    Unknown,
}

impl Display for SleepMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SleepMode::Awake => write!(f, "Awake"),
            SleepMode::Sleep => write!(f, "Sleeping"),
            SleepMode::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIMode {
    NoTracking,
    NormalTracking,
    UpperBody,
    CloseUp,
    Headless,
    LowerBody,
    DeskMode,
    Whiteboard,
    Hand,
    Group,
    Unknown,
}

impl Display for AIMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIMode::NoTracking => write!(f, "Static"),
            AIMode::NormalTracking => write!(f, "Normal Tracking"),
            AIMode::UpperBody => write!(f, "Upper Body"),
            AIMode::CloseUp => write!(f, "Close-up"),
            AIMode::Headless => write!(f, "Headless"),
            AIMode::LowerBody => write!(f, "Lower Body"),
            AIMode::DeskMode => write!(f, "Desk Mode"),
            AIMode::Whiteboard => write!(f, "Whiteboard"),
            AIMode::Hand => write!(f, "Hand"),
            AIMode::Group => write!(f, "Group"),
            AIMode::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrackingSpeed {
    Standard,
    Sport,
}

impl Display for TrackingSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackingSpeed::Standard => write!(f, "Standard"),
            TrackingSpeed::Sport => write!(f, "Sport"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExposureMode {
    Manual,
    Global,
    Face,
}

impl Display for ExposureMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExposureMode::Manual => write!(f, "Manual"),
            ExposureMode::Global => write!(f, "Global"),
            ExposureMode::Face => write!(f, "Face"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExposureModeType {
    Auto,
    Manual,
}

#[cfg(test)]
mod tests {
    mod unit {
        mod display {
            mod sleep_mode {
                use crate::SleepMode;
                use test_case::test_case;

                #[test_case(SleepMode::Awake, "Awake"; "Awake")]
                #[test_case(SleepMode::Sleep, "Sleeping"; "Sleep")]
                #[test_case(SleepMode::Unknown, "Unknown"; "Unknown")]
                fn sleep_mode(mode: SleepMode, expected: &str) {
                    assert_eq!(&mode.to_string(), expected);
                }
            }

            mod ai_mode {
                use crate::AIMode;
                use test_case::test_case;

                #[test_case(AIMode::NoTracking, "Static"; "no tracking")]
                #[test_case(AIMode::NormalTracking, "Normal Tracking"; "normal tracking")]
                #[test_case(AIMode::UpperBody, "Upper Body"; "upper body")]
                #[test_case(AIMode::CloseUp, "Close-up"; "close up")]
                #[test_case(AIMode::Headless, "Headless"; "headless")]
                #[test_case(AIMode::LowerBody, "Lower Body"; "lower body")]
                #[test_case(AIMode::DeskMode, "Desk Mode"; "desk mode")]
                #[test_case(AIMode::Whiteboard, "Whiteboard"; "whiteboard")]
                #[test_case(AIMode::Hand, "Hand"; "hand")]
                #[test_case(AIMode::Group, "Group"; "group")]
                #[test_case(AIMode::Unknown, "Unknown"; "unknown")]
                fn ai_mode(mode: AIMode, expected: &str) {
                    assert_eq!(&mode.to_string(), expected);
                }
            }

            mod tracking_speed {
                use crate::TrackingSpeed;
                use test_case::test_case;

                #[test_case(TrackingSpeed::Standard, "Standard"; "standard")]
                #[test_case(TrackingSpeed::Sport, "Sport"; "sport")]
                fn tracking_speed(mode: TrackingSpeed, expected: &str) {
                    assert_eq!(&mode.to_string(), expected);
                }
            }

            mod exposure_mode {
                use crate::ExposureMode;
                use test_case::test_case;

                #[test_case(ExposureMode::Manual, "Manual"; "manual")]
                #[test_case(ExposureMode::Global, "Global"; "global")]
                #[test_case(ExposureMode::Face, "Face"; "face")]
                fn exposure_mode(mode: ExposureMode, expected: &str) {
                    assert_eq!(&mode.to_string(), expected);
                }
            }
        }
    }
}
