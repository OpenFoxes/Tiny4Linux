// SPDX-License-Identifier: EUPL-1.2

mod usbio;

use errno::Errno;
use std::fmt::Debug;
use std::{fmt::Display, io};
use thiserror::Error;
use usbio::UvcUsbIo;

#[derive(Error, Debug)]
pub enum Error {
    #[error("value of {1} is not supported for {0}")]
    UnsupportedIntValue(String, i32),
    #[error("USB IO error: {0}")]
    USBIOError(i32),
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("no camera found")]
    NoCameraFound,
    #[error("Invalid setting")]
    InvalidSetting,
}

#[derive(Debug)]
pub struct Camera {
    handle: usbio::CameraHandle,
    debugging: bool,
}

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

impl TryFrom<i32> for AIMode {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AIMode::NoTracking),
            1 => Ok(AIMode::NormalTracking),
            2 => Ok(AIMode::UpperBody),
            3 => Ok(AIMode::CloseUp),
            4 => Ok(AIMode::Headless),
            5 => Ok(AIMode::LowerBody),
            6 => Ok(AIMode::DeskMode),
            7 => Ok(AIMode::Whiteboard),
            8 => Ok(AIMode::Hand),
            9 => Ok(AIMode::Group),
            _ => Err(Error::UnsupportedIntValue("AIMode".to_string(), value)),
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

pub enum TrackingMode {
    Headroom,
    Standard,
    Motion,
}

impl TryFrom<i32> for TrackingMode {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TrackingMode::Headroom),
            1 => Ok(TrackingMode::Standard),
            2 => Ok(TrackingMode::Motion),
            _ => Err(Error::UnsupportedIntValue(
                "TrackingMode".to_string(),
                value,
            )),
        }
    }
}

pub trait OBSBotWebCam {
    fn set_sleep_mode(&self, mode: SleepMode) -> Result<(), Error>;
    fn get_sleep_mode(&self) -> Result<SleepMode, Error>;
    fn set_ai_mode(&self, mode: AIMode) -> Result<(), Error>;
    fn get_ai_mode(&self) -> Result<AIMode, Error>;
    fn goto_preset_position(&self, preset_nr: i8) -> Result<(), Error>;
    fn get_tracking_speed(&self) -> Result<TrackingSpeed, Error>;
    fn set_tracking_speed(&self, speed: TrackingSpeed) -> Result<(), Error>;
    fn set_hdr_mode(&self, mode: bool) -> Result<(), Error>;
    fn set_exposure_mode(&self, mode: ExposureMode) -> Result<(), Error>;
    fn set_exposure_mode_type(&self, mode: ExposureModeType) -> Result<(), Error>;
    fn set_debugging(&mut self, debugging: bool);
}

impl OBSBotWebCam for Camera {
    fn set_sleep_mode(&self, mode: SleepMode) -> Result<(), Error> {
        if mode == SleepMode::Unknown {
            return Err(Error::InvalidSetting);
        }

        const FUNCTION_GROUP_SLEEP: [u8; 6] = [0x0a, 0x02, 0xc2, 0xa0, 0x04, 0x00];

        let (sequence_nr, checksum, command) = match mode {
            SleepMode::Awake => (
                [0xa5, 0x00],
                [0x5f, 0xef],
                [0xbe, 0x07, 0x00, 0x00, 0x00, 0x00],
            ),
            SleepMode::Sleep => (
                [0x42, 0x00],
                [0xea, 0x63],
                [0xbf, 0xfb, 0x01, 0x00, 0x00, 0x00],
            ),
            SleepMode::Unknown => panic!(),
        };

        let cmd = Command02::new()
            .function_group(FUNCTION_GROUP_SLEEP)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .build();

        self.get_status()?.awake = mode;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn get_sleep_mode(&self) -> Result<SleepMode, Error> {
        Ok(self.get_status()?.awake)
    }

    fn set_ai_mode(&self, mode: AIMode) -> Result<(), Error> {
        let cmd = match mode {
            AIMode::NoTracking => [0x16, 0x02, 0x00, 0x00],
            AIMode::NormalTracking => [0x16, 0x02, 0x02, 0x00],
            AIMode::UpperBody => [0x16, 0x02, 0x02, 0x01],
            AIMode::DeskMode => [0x16, 0x02, 0x05, 0x00],
            AIMode::Whiteboard => [0x16, 0x02, 0x04, 0x00],
            AIMode::Group => [0x16, 0x02, 0x01, 0x00],
            AIMode::Hand => [0x16, 0x02, 0x03, 0x00],
            AIMode::CloseUp => [0x16, 0x02, 0x02, 0x02],
            AIMode::Headless => [0x16, 0x02, 0x02, 0x03],
            AIMode::LowerBody => [0x16, 0x02, 0x02, 0x04],
            AIMode::Unknown => [0x16, 0x02, 0x00, 0x00],
        };
        self.send_cmd(0x2, 0x6, &cmd)
    }

    fn get_ai_mode(&self) -> Result<AIMode, Error> {
        Ok(self.get_status()?.ai_mode)
    }

    fn goto_preset_position(&self, preset_nr: i8) -> Result<(), Error> {
        if preset_nr < 0 || preset_nr > 3 {
            return Err(Error::InvalidSetting);
        }

        const FUNCTION_GROUP_PRESETS: [u8; 6] = [0x0a, 0x04, 0xc4, 0x39, 0x14, 0x00];

        let (sequence_nr, checksum, command) = match preset_nr {
            0 => (
                [0x20, 0x00],
                [0x6b, 0xdc],
                [0xd6, 0xfb, 0x00, 0x00, 0x00, 0x00],
            ),
            1 => (
                [0x1a, 0x00],
                [0x4b, 0x03],
                [0xeb, 0x2a, 0x01, 0x00, 0x00, 0x00],
            ),
            2 => (
                [0x26, 0x00],
                [0x8b, 0xc3],
                [0xaf, 0x19, 0x02, 0x00, 0x00, 0x00],
            ),
            _ => {
                println!("Invalid preset nr {}", preset_nr + 1);
                return Err(Error::InvalidSetting);
            }
        };

        let cmd = Command02::new()
            .function_group(FUNCTION_GROUP_PRESETS)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .appendix({
                let mut arr = [0u8; 16];
                for i in 0..4 {
                    arr[i * 4..(i + 1) * 4].copy_from_slice(&[0x00, 0x00, 0x80, 0x3f]);
                }
                arr
            })
            .build();

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn get_tracking_speed(&self) -> Result<TrackingSpeed, Error> {
        Ok(self.get_status()?.speed)
    }

    fn set_tracking_speed(&self, speed: TrackingSpeed) -> Result<(), Error> {
        const FUNCTION_GROUP_TRACKING_SPEED: [u8; 6] = [0x0a, 0x04, 0xc4, 0x0c, 0x01, 0x00];

        let appendix: [u8; 16] = {
            let mut a = [0x00; 16];
            a[..4].fill(0x00);
            a
        };

        let (sequence_nr, checksum, command) = match speed {
            TrackingSpeed::Standard => (
                [0x20, 0x00],
                [0xab, 0xcb],
                [0xe6, 0x3f, 0x00, 0x00, 0x00, 0x00],
            ),
            TrackingSpeed::Sport => (
                [0x21, 0x00],
                [0xfa, 0x0e],
                [0x67, 0xfe, 0x02, 0x00, 0x00, 0x00],
            ),
        };

        let cmd = Command02::new()
            .function_group(FUNCTION_GROUP_TRACKING_SPEED)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .appendix(appendix)
            .build();

        self.get_status()?.speed = speed;

        self.send_cmd(0x2, 0x2, &cmd)
    }

    fn set_hdr_mode(&self, mode: bool) -> Result<(), Error> {
        let cmd = if mode {
            [0x01, 0x01, 0x01]
        } else {
            [0x01, 0x01, 0x00]
        };
        self.send_cmd(0x2, 0x6, &cmd)
    }

    fn set_exposure_mode(&self, mode: ExposureMode) -> Result<(), Error> {
        match mode {
            ExposureMode::Manual => {
                self.set_exposure_mode_type(ExposureModeType::Manual)?;
            }
            ExposureMode::Global => {
                self.set_exposure_mode_type(ExposureModeType::Auto)?;
                self.send_cmd(0x2, 0x6, &[0x03, 0x01, 0x00])?;
            }
            ExposureMode::Face => {
                self.set_exposure_mode_type(ExposureModeType::Auto)?;
                self.send_cmd(0x2, 0x6, &[0x03, 0x01, 0x01])?;
            }
        };
        Ok(())
    }

    fn set_exposure_mode_type(&self, mode: ExposureModeType) -> Result<(), Error> {
        const FUNCTION_GROUP_EXPOSURE_MODE_TYPE: [u8; 6] = [0x0a, 0x02, 0x82, 0x29, 0x05, 0x00];

        let (sequence_nr, checksum, command) = match mode {
            ExposureModeType::Auto => (
                [0x16, 0x00],
                [0x58, 0x91],
                [0xb2, 0xaf, 0x02, 0x04, 0x00, 0x00],
            ),
            ExposureModeType::Manual => (
                [0x15, 0x00],
                [0xa8, 0x9e],
                [0xf9, 0x27, 0x01, 0x32, 0x00, 0x00],
            ),
        };

        let command = Command02::new()
            .function_group(FUNCTION_GROUP_EXPOSURE_MODE_TYPE)
            .sequence_nr(sequence_nr)
            .checksum(checksum)
            .command(command)
            .build();

        self.send_cmd(0x2, 0x2, &command)?;

        Ok(())
    }

    fn set_debugging(&mut self, debugging: bool) {
        self.set_debugging(debugging);
    }
}

impl Camera {
    pub fn new(hint: &str) -> Result<Self, Error> {
        Ok(Self {
            handle: usbio::open_camera(hint)?,
            debugging: false,
        })
    }

    pub fn info(&self) -> Result<(), Errno> {
        self.handle.info()
    }

    pub fn get_status(&self) -> Result<CameraStatus, Error> {
        let mut data: [u8; 60] = [0u8; 60];
        self.get_cur(0x2, 0x6, &mut data)
            .map_err(|x| Error::USBIOError(x.0))?;

        if self.debugging {
            println!("Current state: {:?} {:}", data, hex::encode(&data));
        }

        Ok(CameraStatus::decode(&data))
    }

    pub fn dump(&self) -> Result<(), Errno> {
        let mut data: [u8; 60] = [0u8; 60];
        self.get_cur(0x2, 0x6, &mut data)?;
        hexdump::hexdump(&data);
        Ok(())
    }

    pub fn dump_02(&self) -> Result<(), Errno> {
        let mut data: [u8; 60] = [0u8; 60];
        self.get_cur(0x2, 0x2, &mut data)?;
        hexdump::hexdump(&data);
        Ok(())
    }

    pub fn send_cmd(&self, unit: u8, selector: u8, cmd: &[u8]) -> Result<(), Error> {
        let mut data = [0u8; 60];
        data[..cmd.len()].copy_from_slice(cmd);

        self.set_cur(unit, selector, &mut data)
            .map_err(|e| Error::USBIOError(e.0))
    }

    fn get_cur(&self, unit: u8, selector: u8, data: &mut [u8]) -> Result<(), Errno> {
        // always call get_len first
        match self.get_len(unit, selector) {
            Ok(size) => {
                if data.len() < size {
                    println!("Got size {}", size);
                    return Err(Errno(1));
                }
            }
            Err(err) => return Err(err),
        };

        // Why not &mut data here?
        match self.io(unit, selector, usbio::UVC_GET_CUR, data) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn set_cur(&self, unit: u8, selector: u8, data: &mut [u8]) -> Result<(), Errno> {
        match self.get_len(unit, selector) {
            Ok(size) => {
                if data.len() > size {
                    println!("Got size {}", size);
                    return Err(Errno(1));
                }
            }
            Err(err) => return Err(err),
        };

        if self.debugging {
            println!("{:} {:} {:}", unit, selector, hex::encode(&data));
        }

        match self.io(unit, selector, usbio::UVC_SET_CUR, data) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn get_len(&self, unit: u8, selector: u8) -> Result<usize, Errno> {
        let mut data = [0u8; 2];

        match self.io(unit, selector, usbio::UVC_GET_LEN, &mut data) {
            Ok(_) => Ok(u16::from_le_bytes(data).into()),
            Err(err) => Err(err),
        }
    }

    fn io(&self, unit: u8, selector: u8, query: u8, data: &mut [u8]) -> Result<(), Errno> {
        self.handle.io(unit, selector, query, data)
    }

    fn set_debugging(&mut self, debugging: bool) {
        self.debugging = debugging
    }
}

pub struct Command02 {
    pub function_group: Option<[u8; 6]>,
    pub sequence_nr: Option<[u8; 2]>,
    pub checksum: Option<[u8; 2]>,
    pub command: Option<[u8; 6]>,
    pub appendix: Option<[u8; 16]>,
}

impl Command02 {
    pub fn new() -> Self {
        Self {
            function_group: None,
            sequence_nr: None,
            checksum: None,
            command: None,
            appendix: None,
        }
    }

    pub fn function_group(mut self, function_group: [u8; 6]) -> Self {
        self.function_group = Some(function_group);
        self
    }

    pub fn sequence_nr(mut self, sequence_number: [u8; 2]) -> Self {
        self.sequence_nr = Some(sequence_number);
        self
    }

    pub fn checksum(mut self, checksum: [u8; 2]) -> Self {
        self.checksum = Some(checksum);
        self
    }

    pub fn command(mut self, cmd: [u8; 6]) -> Self {
        self.command = Some(cmd);
        self
    }

    pub fn appendix(mut self, app: [u8; 16]) -> Self {
        self.appendix = Some(app);
        self
    }

    pub fn build(self) -> [u8; 36] {
        const FRAME_ID: [u8; 2] = [0xaa, 0x25];
        const SEGMENT_SIZE: [u8; 2] = [0x0c, 0x00];

        [
            FRAME_ID.as_slice(),
            self.sequence_nr
                .expect("sequence_nr is required")
                .as_slice(),
            SEGMENT_SIZE.as_slice(),
            self.checksum.expect("checksum is required").as_slice(),
            self.function_group
                .expect("function_group is required")
                .as_slice(),
            self.command.expect("command is required").as_slice(),
            self.appendix.unwrap_or([0x00; 16]).as_slice(),
        ]
        .concat()
        .try_into()
        .unwrap()
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
