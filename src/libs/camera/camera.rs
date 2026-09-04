// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::enums::{AIMode, CameraModel, ExposureMode, SleepMode, TrackingSpeed};
use crate::libs::camera::status::CameraStatus;
use crate::libs::camera::transport::CameraTransport;
use crate::libs::errors::T4lError;
use crate::{
    AIModeCommand, ExposureModeCommand, ExposureModeTypeCommand, GotoPresetPositionCommand,
    HdrModeCommand, LegacyAiModeCommand, LegacyPresetPositionCommand, LegacySleepCommand,
    SleepCommand, TrackingSpeedCommand,
};
use errno::Errno;
use std::thread::sleep;
use std::time::Duration;

/// Pause between two legacy frames sent in one sequence.
///
/// The Tiny 4K drops frames that arrive back to back. In captures of the vendor
/// app no two legacy frames are ever closer than about 50 ms, with a median of
/// roughly 95 ms, so the sequence is paced accordingly (#72).
const LEGACY_FRAME_GAP: Duration = Duration::from_millis(100);

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

    /// Whether this camera has the given AI tracking mode.
    ///
    /// The Tiny 4K only knows a part of the Tiny 2 modes (#72).
    pub fn supports_ai_mode(&self, mode: AIMode) -> bool {
        self.model.supports_ai_mode(mode)
    }

    /// Whether this camera has a tracking speed setting.
    ///
    /// The Tiny 4K has none: the setting does not exist in its protocol.
    pub fn supports_tracking_speed(&self) -> bool {
        self.model.supports_tracking_speed()
    }

    /// Whether this camera takes the manual exposure mode over this interface.
    ///
    /// The Tiny 4K drives manual exposure through the standard UVC camera
    /// terminal instead, which is reachable via V4L2.
    pub fn supports_manual_exposure(&self) -> bool {
        self.model.supports_manual_exposure()
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

    /// Sends the AI tracking command matching the detected model.
    ///
    /// The Tiny 2 takes a single setting on selector 6. The Tiny 4K needs a
    /// sequence of legacy frames on selector 2 instead (#72).
    fn set_ai_mode(&self, mode: AIMode) -> Result<(), T4lError> {
        match self.model {
            CameraModel::Tiny2 => self.send_cmd(0x2, 0x6, &AIModeCommand::build(mode)?),
            CameraModel::Tiny4K => {
                for (index, frame) in LegacyAiModeCommand::build(mode)?.iter().enumerate() {
                    if index > 0 {
                        sleep(LEGACY_FRAME_GAP);
                    }

                    self.send_cmd(0x2, 0x2, frame)?;
                }

                Ok(())
            }
        }
    }

    fn get_ai_mode(&self) -> Result<AIMode, T4lError> {
        Ok(self.get_status()?.ai_mode)
    }

    /// Moves the camera to a stored preset position.
    ///
    /// The Tiny 2 has a recall command. The Tiny 4K has not: it stores the positions
    /// and hands them out on request, so the recall is a read followed by an absolute
    /// move (#72). Presets are stored by the vendor app; this only recalls them.
    fn goto_preset_position(&self, preset_nr: i8) -> Result<(), T4lError> {
        match self.model {
            CameraModel::Tiny2 => {
                self.send_cmd(0x2, 0x2, &GotoPresetPositionCommand::build(preset_nr)?)
            }
            CameraModel::Tiny4K => {
                let slot = u8::try_from(preset_nr).map_err(|_| {
                    T4lError::UnsupportedIntValue("preset position".to_string(), preset_nr as i32)
                })?;
                let table = self.transport.get_preset_positions(self.debugging)?;
                let preset = LegacyPresetPositionCommand::find(&table, slot)?;

                for (index, frame) in LegacyPresetPositionCommand::build(&preset)
                    .iter()
                    .enumerate()
                {
                    if index > 0 {
                        sleep(LEGACY_FRAME_GAP);
                    }

                    self.send_cmd(0x2, 0x2, frame)?;
                }

                Ok(())
            }
        }
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
