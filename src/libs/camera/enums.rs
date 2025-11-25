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
