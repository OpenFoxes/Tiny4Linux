// SPDX-License-Identifier: EUPL-1.2

use crate::libs::camera::enums::{
    AIMode, AutoSleepTime, ExposureMode, FieldOfView, Gesture, NoiseReduction, PickupDistance,
    SleepBackground, SleepMode, TrackingSpeed,
};
use crate::libs::camera::frame::FrameSequence;
use crate::libs::camera::status::CameraStatus;
use crate::libs::camera::transport::CameraTransport;
use crate::libs::errors::T4lError;
use crate::{
    AIModeCommand, AutoGainCommand, AutoSleepCommand, ExposureModeCommand, ExposureModeTypeCommand,
    FactoryResetCommand, FieldOfViewCommand, GestureCommand, GestureZoomRatioCommand,
    GotoPresetPositionCommand, HdrModeCommand, InvertCommand, LedCommand, MicInSleepCommand,
    MicrophoneCommand, MirrorCommand, NoiseReductionCommand, PickupDistanceCommand,
    SleepBackgroundCommand, SleepBackgroundMirrorCommand, SleepCommand, TrackingSpeedCommand,
};
use errno::Errno;

pub struct Camera {
    transport: CameraTransport,
    debugging: bool,
    sequence: FrameSequence,
}

impl Camera {
    pub fn new(hint: &str) -> Result<Self, T4lError> {
        Ok(Self {
            transport: CameraTransport::new(hint)?,
            debugging: false,
            sequence: FrameSequence::new(),
        })
    }

    pub fn info(&self) -> Result<(), Errno> {
        self.transport.info()
    }

    pub fn send_cmd(&self, unit: u8, selector: u8, cmd: &[u8]) -> Result<(), T4lError> {
        self.transport.send_cmd(unit, selector, cmd, self.debugging)
    }

    /// Returns a fresh sequence number for the next framed command (unit 2, selector 2).
    pub fn next_sequence_nr(&self) -> u16 {
        self.sequence.next()
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
    fn set_auto_sleep(&self, time: AutoSleepTime) -> Result<(), T4lError>;
    /// Turns auto-sleep off while keeping the configured time, like OBSBOT Center does.
    fn disable_auto_sleep(&self) -> Result<(), T4lError>;
    fn set_sleep_background(&self, background: SleepBackground) -> Result<(), T4lError>;
    fn set_sleep_background_mirror(&self, mirrored: bool) -> Result<(), T4lError>;
    fn set_led_brightness(&self, brightness: u8) -> Result<(), T4lError>;
    fn set_field_of_view(&self, fov: FieldOfView) -> Result<(), T4lError>;
    fn set_mirror(&self, mirrored: bool) -> Result<(), T4lError>;
    /// Reverses the gimbal's left/right (pan) direction; the image does not change. The camera
    /// does not report this setting back.
    fn set_invert(&self, inverted: bool) -> Result<(), T4lError>;
    fn set_mic_in_sleep(&self, enabled: bool) -> Result<(), T4lError>;
    fn set_noise_reduction(&self, level: NoiseReduction) -> Result<(), T4lError>;
    fn set_auto_gain(&self, enabled: bool) -> Result<(), T4lError>;
    fn set_pickup_distance(&self, distance: PickupDistance) -> Result<(), T4lError>;
    /// Enables or disables the microphone. Disabling it makes the camera restart.
    fn set_microphone(&self, enabled: bool) -> Result<(), T4lError>;
    fn set_gesture(&self, gesture: Gesture, enabled: bool) -> Result<(), T4lError>;
    /// Sets how far the zoom gesture zooms in (1.0 to 4.0). The camera does not report it back.
    fn set_gesture_zoom_ratio(&self, ratio: f32) -> Result<(), T4lError>;
    /// Resets the camera to its factory settings. The camera restarts afterwards.
    fn factory_reset(&self) -> Result<(), T4lError>;
    fn set_debugging(&mut self, debugging: bool);
}

impl Tiny2Camera for Camera {
    fn set_sleep_mode(&self, mode: SleepMode) -> Result<(), T4lError> {
        let cmd = SleepCommand::build(mode, self.next_sequence_nr())?;

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
        let cmd = GotoPresetPositionCommand::build(preset_nr, self.next_sequence_nr())?;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn get_tracking_speed(&self) -> Result<TrackingSpeed, T4lError> {
        Ok(self.get_status()?.speed)
    }

    fn set_tracking_speed(&self, speed: TrackingSpeed) -> Result<(), T4lError> {
        let cmd = TrackingSpeedCommand::build(speed, self.next_sequence_nr())?;

        self.get_status()?.speed = speed;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn set_hdr_mode(&self, mode: bool) -> Result<(), T4lError> {
        let cmd = HdrModeCommand::build(mode);

        self.send_cmd(0x2, 0x6, &cmd)
    }

    fn set_exposure_mode(&self, mode: ExposureMode) -> Result<(), T4lError> {
        let exposure_mode_type_command =
            ExposureModeTypeCommand::build(mode, self.next_sequence_nr());

        self.send_cmd(0x2, 0x2, &exposure_mode_type_command)?;

        let exposure_mode_command = ExposureModeCommand::build(mode);

        exposure_mode_command
            .map(|exposure_mode_command| self.send_cmd(0x2, 0x6, &exposure_mode_command));

        Ok(())
    }

    fn set_auto_sleep(&self, time: AutoSleepTime) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &AutoSleepCommand::build(time))
    }

    fn disable_auto_sleep(&self) -> Result<(), T4lError> {
        let current = self.get_status()?.auto_sleep;

        self.set_auto_sleep(current.disabled())
    }

    fn set_sleep_background(&self, background: SleepBackground) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &SleepBackgroundCommand::build(background)?)
    }

    fn set_sleep_background_mirror(&self, mirrored: bool) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &SleepBackgroundMirrorCommand::build(mirrored))
    }

    fn set_led_brightness(&self, brightness: u8) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &LedCommand::build(brightness)?)
    }

    fn set_field_of_view(&self, fov: FieldOfView) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &FieldOfViewCommand::build(fov)?)
    }

    fn set_mirror(&self, mirrored: bool) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &MirrorCommand::build(mirrored))
    }

    fn set_invert(&self, inverted: bool) -> Result<(), T4lError> {
        self.send_cmd(
            0x2,
            0x2,
            &InvertCommand::build(inverted, self.next_sequence_nr()),
        )
    }

    fn set_mic_in_sleep(&self, enabled: bool) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &MicInSleepCommand::build(enabled))
    }

    fn set_noise_reduction(&self, level: NoiseReduction) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &NoiseReductionCommand::build(level)?)
    }

    fn set_auto_gain(&self, enabled: bool) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &AutoGainCommand::build(enabled))
    }

    fn set_pickup_distance(&self, distance: PickupDistance) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &PickupDistanceCommand::build(distance)?)
    }

    fn set_microphone(&self, enabled: bool) -> Result<(), T4lError> {
        self.send_cmd(0x2, 0x6, &MicrophoneCommand::build(enabled))
    }

    fn set_gesture(&self, gesture: Gesture, enabled: bool) -> Result<(), T4lError> {
        let cmd = GestureCommand::build(gesture, enabled, self.next_sequence_nr());

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn set_gesture_zoom_ratio(&self, ratio: f32) -> Result<(), T4lError> {
        let cmd = GestureZoomRatioCommand::build(ratio, self.next_sequence_nr())?;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn factory_reset(&self) -> Result<(), T4lError> {
        self.send_cmd(
            0x2,
            0x2,
            &FactoryResetCommand::build(self.next_sequence_nr()),
        )
    }

    fn set_debugging(&mut self, debugging: bool) {
        self.set_debugging(debugging);
    }
}
