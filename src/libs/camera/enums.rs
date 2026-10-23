// SPDX-License-Identifier: EUPL-1.2

use rust_i18n::t;
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
            SleepMode::Awake => write!(f, "{}", t!("display.sleep_mode.awake")),
            SleepMode::Sleep => write!(f, "{}", t!("display.sleep_mode.sleep")),
            SleepMode::Unknown => write!(f, "{}", t!("display.sleep_mode.unknown")),
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
            AIMode::NoTracking => write!(f, "{}", t!("display.ai_mode.static")),
            AIMode::NormalTracking => write!(f, "{}", t!("display.ai_mode.normal")),
            AIMode::UpperBody => write!(f, "{}", t!("display.ai_mode.upper_body")),
            AIMode::CloseUp => write!(f, "{}", t!("display.ai_mode.close_up")),
            AIMode::Headless => write!(f, "{}", t!("display.ai_mode.headless")),
            AIMode::LowerBody => write!(f, "{}", t!("display.ai_mode.lower_body")),
            AIMode::DeskMode => write!(f, "{}", t!("display.ai_mode.desk")),
            AIMode::Whiteboard => write!(f, "{}", t!("display.ai_mode.whiteboard")),
            AIMode::Hand => write!(f, "{}", t!("display.ai_mode.hand")),
            AIMode::Group => write!(f, "{}", t!("display.ai_mode.group")),
            AIMode::Unknown => write!(f, "{}", t!("display.ai_mode.unknown")),
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
            TrackingSpeed::Standard => write!(f, "{}", t!("display.tracking_speed.standard")),
            TrackingSpeed::Sport => write!(f, "{}", t!("display.tracking_speed.sport")),
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
            ExposureMode::Manual => write!(f, "{}", t!("display.exposure_mode.manual")),
            ExposureMode::Global => write!(f, "{}", t!("display.exposure_mode.global")),
            ExposureMode::Face => write!(f, "{}", t!("display.exposure_mode.face")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExposureModeType {
    Auto,
    Manual,
}

/// Auto-sleep time as reported in the status block and sent with the auto-sleep command.
///
/// The camera stores seconds as `i16`; a negative value means auto-sleep is off and keeps
/// the last time, so turning it on again restores it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoSleepTime(pub i16);

impl AutoSleepTime {
    /// Factory default, used when there is no previous time to keep.
    pub const DEFAULT_SECONDS: i16 = 120;

    pub fn is_enabled(&self) -> bool {
        self.0 > 0
    }

    pub fn seconds(&self) -> u16 {
        self.0.unsigned_abs()
    }

    /// The value that turns auto-sleep off: OBSBOT Center sends the current time negated
    /// (e.g. -120 when 120 s were set). An already disabled time is kept as it is.
    pub fn disabled(&self) -> Self {
        match self.0 {
            t if t > 0 => AutoSleepTime(-t),
            0 => AutoSleepTime(-Self::DEFAULT_SECONDS),
            t => AutoSleepTime(t),
        }
    }
}

impl Display for AutoSleepTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_enabled() {
            write!(
                f,
                "{}",
                t!("display.auto_sleep.on", seconds = self.seconds())
            )
        } else if self.0 == 0 {
            write!(f, "{}", t!("display.states.off"))
        } else {
            write!(
                f,
                "{}",
                t!("display.auto_sleep.off", seconds = self.seconds())
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SleepBackground {
    Black,
    Image,
    Unknown,
}

impl Display for SleepBackground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SleepBackground::Black => write!(f, "{}", t!("display.sleep_background.black")),
            SleepBackground::Image => write!(f, "{}", t!("display.sleep_background.image")),
            SleepBackground::Unknown => write!(f, "{}", t!("display.sleep_background.unknown")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldOfView {
    Wide,
    Medium,
    Narrow,
    Unknown,
}

impl Display for FieldOfView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldOfView::Wide => write!(f, "{}", t!("display.field_of_view.wide")),
            FieldOfView::Medium => write!(f, "{}", t!("display.field_of_view.medium")),
            FieldOfView::Narrow => write!(f, "{}", t!("display.field_of_view.narrow")),
            FieldOfView::Unknown => write!(f, "{}", t!("display.field_of_view.unknown")),
        }
    }
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

            mod auto_sleep_time {
                use crate::AutoSleepTime;
                use test_case::test_case;

                #[test_case(120, "120 s"; "enabled")]
                #[test_case(-120, "Off (120 s)"; "disabled keeps the time")]
                #[test_case(0, "Off"; "zero")]
                fn auto_sleep_time(seconds: i16, expected: &str) {
                    assert_eq!(&AutoSleepTime(seconds).to_string(), expected);
                }
            }

            mod field_of_view {
                use crate::FieldOfView;
                use test_case::test_case;

                #[test_case(FieldOfView::Wide, "Wide"; "wide")]
                #[test_case(FieldOfView::Medium, "Medium"; "medium")]
                #[test_case(FieldOfView::Narrow, "Narrow"; "narrow")]
                #[test_case(FieldOfView::Unknown, "Unknown"; "unknown")]
                fn field_of_view(fov: FieldOfView, expected: &str) {
                    assert_eq!(&fov.to_string(), expected);
                }
            }

            mod sleep_background {
                use crate::SleepBackground;
                use test_case::test_case;

                #[test_case(SleepBackground::Black, "Black"; "black")]
                #[test_case(SleepBackground::Image, "Image"; "image")]
                #[test_case(SleepBackground::Unknown, "Unknown"; "unknown")]
                fn sleep_background(background: SleepBackground, expected: &str) {
                    assert_eq!(&background.to_string(), expected);
                }
            }
        }

        mod auto_sleep_time {
            use crate::AutoSleepTime;
            use test_case::test_case;

            // Turning auto-sleep off must keep the time (negated) so the camera, like
            // OBSBOT Center, can restore it when auto-sleep is turned on again.
            #[test_case(120, -120; "enabled time is negated")]
            #[test_case(600, -600; "other enabled time is negated")]
            #[test_case(-30, -30; "already disabled stays as it is")]
            #[test_case(0, -120; "no time falls back to the factory default")]
            fn disabled(current: i16, expected: i16) {
                assert_eq!(AutoSleepTime(current).disabled(), AutoSleepTime(expected));
            }

            #[test_case(120, true; "positive is enabled")]
            #[test_case(-120, false; "negative is disabled")]
            #[test_case(0, false; "zero is disabled")]
            fn is_enabled(seconds: i16, expected: bool) {
                assert_eq!(AutoSleepTime(seconds).is_enabled(), expected);
            }
        }
    }
}
