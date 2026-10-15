// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::custom_presets::{CustomPreset, CustomPresetStore};
use crate::libs::camera::enums::{AIMode, ExposureMode, SleepMode, TrackingSpeed};
use crate::libs::camera::status::CameraStatus;
use crate::libs::camera::transport::CameraTransport;
use crate::libs::errors::T4lError;
use crate::libs::usbio::{V4L2_CID_PAN_ABSOLUTE, V4L2_CID_TILT_ABSOLUTE};
use crate::{
    AIModeCommand, ExposureModeCommand, ExposureModeTypeCommand, GotoPresetPositionCommand,
    HdrModeCommand, SleepCommand, TrackingSpeedCommand,
};
use errno::Errno;

pub struct Camera {
    transport: CameraTransport,
    debugging: bool,
}

impl Camera {
    pub fn new(hints: &[&str]) -> Result<Self, T4lError> {
        Ok(Self {
            transport: CameraTransport::new(hints)?,
            debugging: false,
        })
    }

    pub fn info(&self) -> Result<(), Errno> {
        self.transport.info()
    }

    pub fn send_cmd(&self, unit: u8, selector: u8, cmd: &[u8]) -> Result<(), T4lError> {
        self.transport.send_cmd(unit, selector, cmd, self.debugging)
    }

    pub fn get_status(&self) -> Result<CameraStatus, T4lError> {
        self.transport.get_status(self.debugging)
    }

    pub fn dump(&self) -> Result<(), Errno> {
        self.transport.dump()
    }

    pub fn dump_02(&self) -> Result<(), Errno> {
        self.transport.dump_02()
    }

    pub fn set_debugging(&mut self, debugging: bool) {
        self.debugging = debugging
    }
}

pub trait Tiny2Camera {
    fn set_sleep_mode(&self, mode: SleepMode) -> Result<(), T4lError>;
    fn get_sleep_mode(&self) -> Result<SleepMode, T4lError>;
    fn set_ai_mode(&self, mode: AIMode) -> Result<(), T4lError>;
    fn get_ai_mode(&self) -> Result<AIMode, T4lError>;
    fn goto_preset_position(&self, preset_nr: i8) -> Result<(), T4lError>;
    fn get_tracking_speed(&self) -> Result<TrackingSpeed, T4lError>;
    fn set_tracking_speed(&self, speed: TrackingSpeed) -> Result<(), T4lError>;
    fn set_hdr_mode(&self, mode: bool) -> Result<(), T4lError>;
    fn set_exposure_mode(&self, mode: ExposureMode) -> Result<(), T4lError>;
    fn get_zoom_absolute(&self) -> Result<i32, T4lError>;
    fn set_zoom_absolute(&self, value: i32) -> Result<(), T4lError>;
    fn get_zoom_range(&self) -> Result<(i32, i32), T4lError>;
    fn get_pan_absolute(&self) -> Result<i32, T4lError>;
    fn set_pan_absolute(&self, value: i32) -> Result<(), T4lError>;
    fn get_pan_range(&self) -> Result<(i32, i32), T4lError>;
    fn get_tilt_absolute(&self) -> Result<i32, T4lError>;
    fn set_tilt_absolute(&self, value: i32) -> Result<(), T4lError>;
    fn get_tilt_range(&self) -> Result<(i32, i32), T4lError>;
    fn save_custom_preset(&self, name: &str) -> Result<(), T4lError>;
    fn recall_custom_preset(&self, name: &str) -> Result<(), T4lError>;
    fn delete_custom_preset(&self, name: &str) -> Result<(), T4lError>;
    fn get_custom_presets(&self) -> Vec<CustomPreset>;
    fn set_debugging(&mut self, debugging: bool);
}

impl Tiny2Camera for Camera {
    fn set_sleep_mode(&self, mode: SleepMode) -> Result<(), T4lError> {
        let cmd = SleepCommand::build(mode)?;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn get_sleep_mode(&self) -> Result<SleepMode, T4lError> {
        Ok(self.get_status()?.awake)
    }

    fn set_ai_mode(&self, mode: AIMode) -> Result<(), T4lError> {
        let cmd = AIModeCommand::build(mode)?;

        self.send_cmd(0x2, 0x6, &cmd)
    }

    fn get_ai_mode(&self) -> Result<AIMode, T4lError> {
        Ok(self.get_status()?.ai_mode)
    }

    fn goto_preset_position(&self, preset_nr: i8) -> Result<(), T4lError> {
        let cmd = GotoPresetPositionCommand::build(preset_nr)?;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn get_tracking_speed(&self) -> Result<TrackingSpeed, T4lError> {
        Ok(self.get_status()?.speed)
    }

    fn set_tracking_speed(&self, speed: TrackingSpeed) -> Result<(), T4lError> {
        let cmd = TrackingSpeedCommand::build(speed)?;

        self.get_status()?.speed = speed;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn set_hdr_mode(&self, mode: bool) -> Result<(), T4lError> {
        let cmd = HdrModeCommand::build(mode);

        self.send_cmd(0x2, 0x6, &cmd)
    }

    fn set_exposure_mode(&self, mode: ExposureMode) -> Result<(), T4lError> {
        let exposure_mode_type_command = ExposureModeTypeCommand::build(mode);

        self.send_cmd(0x2, 0x2, &exposure_mode_type_command)?;

        let exposure_mode_command = ExposureModeCommand::build(mode);

        exposure_mode_command
            .map(|exposure_mode_command| self.send_cmd(0x2, 0x6, &exposure_mode_command));

        Ok(())
    }

    fn set_debugging(&mut self, debugging: bool) {
        self.set_debugging(debugging);
    }

    fn get_zoom_absolute(&self) -> Result<i32, T4lError> {
        self.transport.get_zoom_absolute()
    }

    fn set_zoom_absolute(&self, value: i32) -> Result<(), T4lError> {
        self.transport.set_zoom_absolute(value)
    }

    fn get_zoom_range(&self) -> Result<(i32, i32), T4lError> {
        self.transport.get_zoom_range()
    }

    fn get_pan_absolute(&self) -> Result<i32, T4lError> {
        self.transport.get_control(V4L2_CID_PAN_ABSOLUTE)
    }

    fn set_pan_absolute(&self, value: i32) -> Result<(), T4lError> {
        self.transport.set_control(V4L2_CID_PAN_ABSOLUTE, value)
    }

    fn get_pan_range(&self) -> Result<(i32, i32), T4lError> {
        match self.transport.get_control_range(V4L2_CID_PAN_ABSOLUTE) {
            Ok(range) if range.1 > range.0 => Ok(range),
            // The uvcvideo driver reports a degenerate range for the OBSBOT
            // controls, so fall back to a safe superset. Out-of-range values
            // are clamped by the driver.
            _ => Ok((-648_000, 648_000)),
        }
    }

    fn get_tilt_absolute(&self) -> Result<i32, T4lError> {
        self.transport.get_control(V4L2_CID_TILT_ABSOLUTE)
    }

    fn set_tilt_absolute(&self, value: i32) -> Result<(), T4lError> {
        self.transport.set_control(V4L2_CID_TILT_ABSOLUTE, value)
    }

    fn get_tilt_range(&self) -> Result<(i32, i32), T4lError> {
        match self.transport.get_control_range(V4L2_CID_TILT_ABSOLUTE) {
            Ok(range) if range.1 > range.0 => Ok(range),
            // The uvcvideo driver reports a degenerate range for the OBSBOT
            // controls, so fall back to a safe superset. Out-of-range values
            // are clamped by the driver.
            _ => Ok((-324_000, 324_000)),
        }
    }

    fn save_custom_preset(&self, name: &str) -> Result<(), T4lError> {
        let store = CustomPresetStore::new();

        store.save(&CustomPreset {
            name: name.to_string(),
            pan: self.get_pan_absolute()?,
            tilt: self.get_tilt_absolute()?,
            zoom: self.get_zoom_absolute()?,
        })
    }

    fn recall_custom_preset(&self, name: &str) -> Result<(), T4lError> {
        let store = CustomPresetStore::new();

        let preset = store
            .load()
            .into_iter()
            .find(|p| p.name == name)
            .ok_or_else(|| T4lError::PresetNotFound(name.to_string()))?;

        self.set_ai_mode(AIMode::NoTracking)?;

        self.set_pan_absolute(preset.pan)?;
        self.set_tilt_absolute(preset.tilt)?;
        self.set_zoom_absolute(preset.zoom)?;

        Ok(())
    }

    fn delete_custom_preset(&self, name: &str) -> Result<(), T4lError> {
        CustomPresetStore::new().delete(name)
    }

    fn get_custom_presets(&self) -> Vec<CustomPreset> {
        CustomPresetStore::new().load()
    }
}
