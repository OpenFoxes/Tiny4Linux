// SPDX-License-Identifier: EUPL-1.2

mod styles;
mod ui_modules;

use crate::styles::theme::obsbot_theme;
use crate::ui_modules::window_layout::window_layout;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Container, text};
use iced::window::Position;
use iced::{Element, Point};
use iced::{Length, Size, Subscription, Task, time, window};
use rust_i18n::{i18n, set_locale, t};
use std::time::Duration;
use tiny4linux::{
    AIMode, Camera, CustomPreset, CustomPresetStore, ExposureMode, SUPPORTED_CAMERAS, SleepMode,
    Tiny2Camera, TrackingSpeed, get_language,
};

i18n!("src/locales", fallback = "en");

#[derive(Debug, Clone, PartialEq)]
enum Message {
    RequestWindowModeChange(WindowMode),
    ApplyWindowMode(WindowMode),
    ChangeMainWindowId(Option<window::Id>),
    ChangeSleeping(bool),
    ChangeTracking(AIMode),
    ChangeTrackingSpeed(TrackingSpeed),
    ChangePresetPosition(i8),
    ChangeHDR(bool),
    ChangeExposure(ExposureMode),
    ChangeZoom(i32),
    ChangeZoomText(String),
    ChangePan(i32),
    ChangePanText(String),
    ChangeTilt(i32),
    ChangeTiltText(String),
    CustomPresetName(String),
    SaveCustomPreset,
    RecallCustomPreset(String),
    DeleteCustomPreset(String),
    ChangeDebugging(bool),
    TextInput(String),
    TextInput02(String),
    CheckCamera,
    SendCommand,
    SendCommand02,
    HexDump,
    HexDump02,
}

struct MainPanel {
    camera: Option<Camera>,
    main_window_id: Option<window::Id>,
    window_mode: WindowMode,
    awake: SleepMode,
    tracking: AIMode,
    tracking_speed: TrackingSpeed,
    hdr_on: bool,
    zoom: Option<i32>,
    zoom_min: i32,
    zoom_max: i32,
    zoom_text: String,
    pan: Option<i32>,
    pan_min: i32,
    pan_max: i32,
    pan_text: String,
    tilt: Option<i32>,
    tilt_min: i32,
    tilt_max: i32,
    tilt_text: String,
    custom_presets: Vec<CustomPreset>,
    custom_preset_name: String,
    debugging_on: bool,
    text_input: String,
    text_input_02: String,
}

impl MainPanel {
    fn init_state(window_mode: WindowMode) -> (Self, Task<Message>) {
        let camera = Camera::new(SUPPORTED_CAMERAS).ok();

        let status = camera
            .as_ref()
            .and_then(|c| c.get_status().ok())
            .unwrap_or_else(|| tiny4linux::CameraStatus::default());

        let zoom_range = camera.as_ref().and_then(|c| c.get_zoom_range().ok());
        let zoom = camera.as_ref().and_then(|c| c.get_zoom_absolute().ok());

        let pan_range = camera.as_ref().and_then(|c| c.get_pan_range().ok());
        let pan = camera.as_ref().and_then(|c| c.get_pan_absolute().ok());

        let tilt_range = camera.as_ref().and_then(|c| c.get_tilt_range().ok());
        let tilt = camera.as_ref().and_then(|c| c.get_tilt_absolute().ok());

        let custom_presets = camera
            .as_ref()
            .map(|c| c.get_custom_presets())
            .unwrap_or_default();

        (
            MainPanel {
                camera,
                main_window_id: None,
                window_mode,
                awake: status.awake,
                tracking: status.ai_mode,
                tracking_speed: status.speed,
                hdr_on: status.hdr_on,
                zoom,
                zoom_min: zoom_range.map(|r| r.0).unwrap_or(0),
                zoom_max: zoom_range.map(|r| r.1).unwrap_or(0),
                zoom_text: zoom.map(|z| z.to_string()).unwrap_or_default(),
                pan,
                pan_min: pan_range.map(|r| r.0).unwrap_or(0),
                pan_max: pan_range.map(|r| r.1).unwrap_or(0),
                pan_text: pan.map(|p| p.to_string()).unwrap_or_default(),
                tilt,
                tilt_min: tilt_range.map(|r| r.0).unwrap_or(0),
                tilt_max: tilt_range.map(|r| r.1).unwrap_or(0),
                tilt_text: tilt.map(|t| t.to_string()).unwrap_or_default(),
                custom_presets,
                custom_preset_name: String::new(),
                debugging_on: false,
                text_input: String::new(),
                text_input_02: String::new(),
            },
            window::get_latest().map(Message::ChangeMainWindowId),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if self.camera.is_none() {
            self.camera = Camera::new(SUPPORTED_CAMERAS).ok();

            if self.camera.is_none() {
                return Task::none();
            }
        } else if !self.camera.as_ref().unwrap().get_status().is_ok() {
            self.camera = None;
            return Task::none();
        }

        let camera = self.camera.as_ref().unwrap();

        match message {
            Message::RequestWindowModeChange(new_mode) => {
                let close_task = self
                    .main_window_id
                    .map(|main_window_id| window::close::<Message>(main_window_id))
                    .unwrap_or_else(Task::none);

                let (new_id, open_task) =
                    window::open(get_window_settings_for_window_mode(new_mode));
                let open_task = open_task.map(move |_| Message::ChangeMainWindowId(Some(new_id)));

                let apply_task = Task::done(Message::ApplyWindowMode(new_mode));

                Task::batch([close_task, open_task, apply_task])
            }
            Message::ApplyWindowMode(new_mode) => {
                self.window_mode = new_mode;
                Task::none()
            }
            Message::ChangeMainWindowId(id) => {
                self.main_window_id = id;
                Task::none()
            }
            Message::ChangeSleeping(should_sleep) => {
                if should_sleep {
                    self.awake = SleepMode::Sleep;
                    camera.set_sleep_mode(SleepMode::Sleep).unwrap();
                } else {
                    self.awake = SleepMode::Awake;
                    camera.set_sleep_mode(SleepMode::Awake).unwrap();
                }

                Task::none()
            }
            Message::ChangeTracking(tracking_type) => {
                self.tracking = tracking_type;
                camera.set_ai_mode(tracking_type).unwrap();
                Task::none()
            }
            Message::ChangeTrackingSpeed(new_speed) => {
                self.tracking_speed = new_speed;
                camera.set_tracking_speed(new_speed).unwrap();
                Task::none()
            }
            Message::ChangePresetPosition(new_position) => {
                self.tracking = AIMode::NoTracking;
                self.awake = SleepMode::Awake;
                camera.set_ai_mode(AIMode::NoTracking).unwrap();
                camera.goto_preset_position(new_position).unwrap();
                Task::none()
            }
            Message::ChangeHDR(new_mode) => {
                self.hdr_on = new_mode;
                camera.set_hdr_mode(new_mode).unwrap();
                Task::none()
            }
            Message::ChangeExposure(mode) => {
                camera.set_exposure_mode(mode).unwrap();
                Task::none()
            }
            Message::ChangeZoom(value) => {
                self.zoom = Some(value);
                self.zoom_text = value.to_string();
                camera.set_zoom_absolute(value).unwrap();
                Task::none()
            }
            Message::ChangeZoomText(s) => {
                self.zoom_text = s;
                Task::none()
            }
            Message::ChangePan(value) => {
                let value = value.clamp(self.pan_min, self.pan_max);
                self.pan = Some(value);
                self.pan_text = value.to_string();
                camera.set_pan_absolute(value).unwrap();
                Task::none()
            }
            Message::ChangePanText(s) => {
                self.pan_text = s;
                Task::none()
            }
            Message::ChangeTilt(value) => {
                let value = value.clamp(self.tilt_min, self.tilt_max);
                self.tilt = Some(value);
                self.tilt_text = value.to_string();
                camera.set_tilt_absolute(value).unwrap();
                Task::none()
            }
            Message::ChangeTiltText(s) => {
                self.tilt_text = s;
                Task::none()
            }
            Message::CustomPresetName(s) => {
                self.custom_preset_name = s;
                Task::none()
            }
            Message::SaveCustomPreset => {
                let store = CustomPresetStore::new();

                if !self.custom_preset_name.trim().is_empty()
                    && let (Some(pan), Some(tilt), Some(zoom)) = (self.pan, self.tilt, self.zoom)
                {
                    store
                        .save(&CustomPreset {
                            name: self.custom_preset_name.clone(),
                            pan,
                            tilt,
                            zoom,
                        })
                        .unwrap();

                    self.custom_presets = store.load();
                    self.custom_preset_name.clear();
                }

                Task::none()
            }
            Message::RecallCustomPreset(name) => {
                camera.recall_custom_preset(&name).unwrap();

                let store = CustomPresetStore::new();
                let preset = store.load().into_iter().find(|p| p.name == name).unwrap();

                self.tracking = AIMode::NoTracking;
                self.awake = SleepMode::Awake;
                self.pan = Some(preset.pan);
                self.pan_text = preset.pan.to_string();
                self.tilt = Some(preset.tilt);
                self.tilt_text = preset.tilt.to_string();
                self.zoom = Some(preset.zoom);
                self.zoom_text = preset.zoom.to_string();

                Task::none()
            }
            Message::DeleteCustomPreset(name) => {
                let store = CustomPresetStore::new();
                store.delete(&name).unwrap();
                self.custom_presets = store.load();
                Task::none()
            }
            Message::ChangeDebugging(new_mode) => {
                self.debugging_on = new_mode;
                let mutable_camera = self.camera.as_mut().unwrap();
                mutable_camera.set_debugging(new_mode);
                Task::none()
            }
            Message::TextInput(s) => {
                self.text_input = s;
                Task::none()
            }
            Message::TextInput02(s) => {
                self.text_input_02 = s;
                Task::none()
            }
            Message::SendCommand => {
                let c = hex::decode(&self.text_input).unwrap();
                camera.send_cmd(0x2, 0x6, &c).unwrap();
                Task::none()
            }
            Message::SendCommand02 => {
                let c = hex::decode(&self.text_input_02).unwrap();
                camera.send_cmd(0x2, 0x2, &c).unwrap();
                Task::none()
            }
            Message::HexDump => {
                camera.dump().unwrap();
                Task::none()
            }
            Message::HexDump02 => {
                camera.dump_02().unwrap();
                Task::none()
            }
            Message::CheckCamera => Task::none(),
        }
    }

    fn view(&'_ self) -> Element<'_, Message> {
        if self.camera.is_some() {
            get_current_ui_elements(self).into()
        } else {
            text(t!("shared.errors.no_camera"))
                .size(20)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.camera.is_none() {
            time::every(Duration::from_secs(4)).map(|_| Message::CheckCamera)
        } else {
            time::every(Duration::from_secs(20)).map(|_| Message::CheckCamera)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WindowMode {
    Dashboard,
    Widget,
    Invalid,
}

fn get_size_for_window_mode(window_mode: WindowMode) -> Size {
    match window_mode {
        WindowMode::Dashboard => Size::new(860.0, 780.0), // 43:39
        WindowMode::Widget => Size::new(300.0, 550.0),    // 6:11
        WindowMode::Invalid => Size::ZERO,
    }
}

fn get_position_for_window_mode(window_mode: WindowMode) -> Position {
    match window_mode {
        WindowMode::Dashboard => Position::Centered,
        WindowMode::Widget => Position::SpecificWith(|window_size, screen_size| Point {
            x: (screen_size.width - window_size.width),
            y: (screen_size.height - window_size.height),
        }),
        WindowMode::Invalid => Position::Centered,
    }
}

fn get_window_settings_for_window_mode(window_mode: WindowMode) -> window::Settings {
    let window_size = get_size_for_window_mode(window_mode);
    window::Settings {
        size: window_size,
        resizable: false,
        min_size: Some(window_size),
        max_size: Some(window_size),
        position: get_position_for_window_mode(window_mode),
        decorations: true,
        ..Default::default()
    }
}

fn get_start_mode() -> WindowMode {
    let args: Vec<String> = std::env::args().collect();

    if let Some(start_mode_flag_pos) = args.iter().position(|a| a == ("--start-as")) {
        if let Some(start_mode_arg) = args.get(start_mode_flag_pos + 1) {
            return if start_mode_arg.eq_ignore_ascii_case("dashboard") {
                WindowMode::Dashboard
            } else if start_mode_arg.eq_ignore_ascii_case("widget") {
                WindowMode::Widget
            } else {
                WindowMode::Invalid
            };
        }
    }

    WindowMode::Dashboard
}

fn get_current_ui_elements(app: &MainPanel) -> Container<'static, Message> {
    window_layout(app).width(Length::Fill).height(Length::Fill)
}

fn main() -> iced::Result {
    let language = get_language(false);
    set_locale(language.as_str());

    let start_mode = get_start_mode();

    if start_mode == WindowMode::Invalid {
        println!(
            "Invalid start mode. Please use --start-as dashboard or --start-as widget or remove the flag."
        );
        panic!();
    }

    println!("Starting Tiny4Linux in {:?} mode", start_mode);

    iced::application("Tiny4Linux", MainPanel::update, MainPanel::view)
        .theme(|_| obsbot_theme())
        .window(get_window_settings_for_window_mode(start_mode))
        .subscription(MainPanel::subscription)
        .run_with(move || MainPanel::init_state(start_mode))
}
