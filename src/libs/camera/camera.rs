// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::enums::{AIMode, CameraModel, ExposureMode, SleepMode, TrackingSpeed};
use crate::libs::camera::status::CameraStatus;
use crate::libs::camera::transport::CameraTransport;
use crate::libs::errors::T4lError;
use crate::{
    AIModeCommand, ExposureModeCommand, ExposureModeTypeCommand, GotoPresetPositionCommand,
    HdrModeCommand, LegacySleepCommand, SleepCommand, TrackingSpeedCommand,
};
use errno::Errno;

/// Device hints tried in this order by [`Camera::detect`].
///
/// A hint has to be part of the V4L2 card name of the camera.
/// The Tiny 2 is the primarily supported model and therefore preferred,
/// the Tiny 4K is only found if no Tiny 2 is connected (#72).
const DEFAULT_CAMERA_HINTS: [&str; 2] = ["OBSBOT Tiny 2", "OBSBOT Tiny 4K"];

pub struct Camera {
    transport: CameraTransport,
    model: CameraModel,
    debugging: bool,
}

impl Camera {
    pub fn new(hint: &str) -> Result<Self, T4lError> {
        Ok(Self {
            transport: CameraTransport::new(hint)?,
            model: CameraModel::from_hint(hint),
            debugging: false,
        })
    }

    /// Opens the first camera matching one of the default hints, see [`DEFAULT_CAMERA_HINTS`].
    pub fn detect() -> Result<Self, T4lError> {
        DEFAULT_CAMERA_HINTS
            .iter()
            .find_map(|hint| Self::new(hint).ok())
            .ok_or(T4lError::NoCameraFound)
    }

    pub fn info(&self) -> Result<(), Errno> {
        self.transport.info()
    }

    pub fn send_cmd(&self, unit: u8, selector: u8, cmd: &[u8]) -> Result<(), T4lError> {
        self.transport.send_cmd(unit, selector, cmd, self.debugging)
    }

    pub fn get_status(&self) -> Result<CameraStatus, T4lError> {
        self.transport.get_status(self.debugging, self.model)
    }

    /// The model this camera was detected as, see [`CameraModel`].
    pub fn model(&self) -> CameraModel {
        self.model
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
    fn set_debugging(&mut self, debugging: bool);
}

impl Tiny2Camera for Camera {
    /// Sends the sleep command matching the detected model.
    ///
    /// The Tiny 4K accepts the Tiny 2 frame on USB but ignores it, so it gets the
    /// legacy frame instead (#72). Both models take the command on unit 2,
    /// selector 2.
    fn set_sleep_mode(&self, mode: SleepMode) -> Result<(), T4lError> {
        match self.model {
            CameraModel::Tiny2 => self.send_cmd(0x2, 0x2, &SleepCommand::build(mode)?),
            CameraModel::Tiny4K => self.send_cmd(0x2, 0x2, &LegacySleepCommand::build(mode)?),
        }
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
}
