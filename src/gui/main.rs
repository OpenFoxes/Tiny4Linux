mod styles;

use crate::styles::theme::obsbot_theme;
use iced::Element;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::button::{primary, secondary};
use iced::widget::{button, column, image, row, text, text_input, toggler};
use iced::{Alignment, Length, Padding, Size, Subscription, Task, time, window};
use std::time::Duration;
use tiny4linux::{AIMode, Camera, ExposureMode, OBSBotWebCam, SleepMode, TrackingSpeed};
use tiny4linux_assets::handle_t4l_asset;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    ChangeSleeping(bool),
    ChangeTracking(AIMode),
    ChangeTrackingSpeed(TrackingSpeed),
    ChangePresetPosition(i8),
    ChangeHDR(bool),
    ChangeExposure(ExposureMode),
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
    awake: SleepMode,
    tracking: AIMode,
    tracking_speed: TrackingSpeed,
    hdr_on: bool,
    debugging_on: bool,
    text_input: String,
    text_input_02: String,
}

impl MainPanel {
    fn init_state() -> (Self, Task<Message>) {
        let camera = Camera::new("OBSBOT Tiny 2").ok();

        let status = camera
            .as_ref()
            .and_then(|c| c.get_status().ok())
            .unwrap_or_else(|| tiny4linux::CameraStatus::default());

        (
            MainPanel {
                camera,
                awake: status.awake,
                tracking: status.ai_mode,
                tracking_speed: status.speed,
                hdr_on: status.hdr_on,
                debugging_on: false,
                text_input: String::new(),
                text_input_02: String::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if self.camera.is_none() {
            self.camera = Camera::new("OBSBOT Tiny 2").ok();

            if self.camera.is_none() {
                return Task::none();
            }
        } else if !self.camera.as_ref().unwrap().get_status().is_ok() {
            self.camera = None;
            return Task::none();
        }

        let camera = self.camera.as_ref().unwrap();

        match message {
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
            let mut elements = column![
                row![
                    text("Tiny4Linux")
                        .size(26)
                        .height(100)
                        .align_y(Vertical::Center),
                    image(if self.awake == SleepMode::Awake {
                        handle_t4l_asset("generated/png/icons/inverted-camera.png")
                    } else {
                        handle_t4l_asset("generated/png/icons/inverted-camera-asleep.png")
                    })
                    .height(50)
                ]
                .spacing(30)
                .align_y(Vertical::Center),
                toggler(self.awake != SleepMode::Awake)
                    .label("Sleeping".to_string())
                    .on_toggle(Message::ChangeSleeping),
                button("Static")
                    .on_press(Message::ChangeTracking(AIMode::NoTracking))
                    .style(if self.tracking == AIMode::NoTracking {
                        primary
                    } else {
                        secondary
                    }),
                button("Normal Tracking")
                    .on_press(Message::ChangeTracking(AIMode::NormalTracking))
                    .style(if self.tracking == AIMode::NormalTracking {
                        primary
                    } else {
                        secondary
                    }),
                row![
                    button("Upper Body")
                        .on_press(Message::ChangeTracking(AIMode::UpperBody))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::UpperBody {
                            primary
                        } else {
                            secondary
                        }),
                    button("Close-up")
                        .on_press(Message::ChangeTracking(AIMode::CloseUp))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::CloseUp {
                            primary
                        } else {
                            secondary
                        }),
                ]
                .spacing(10),
                row![
                    button("Headless")
                        .on_press(Message::ChangeTracking(AIMode::Headless))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Headless {
                            primary
                        } else {
                            secondary
                        }),
                    button("Lower Body")
                        .on_press(Message::ChangeTracking(AIMode::LowerBody))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::LowerBody {
                            primary
                        } else {
                            secondary
                        }),
                ]
                .spacing(10),
                row![
                    button("Desk")
                        .on_press(Message::ChangeTracking(AIMode::DeskMode))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::DeskMode {
                            primary
                        } else {
                            secondary
                        }),
                    button("Whiteboard")
                        .on_press(Message::ChangeTracking(AIMode::Whiteboard))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Whiteboard {
                            primary
                        } else {
                            secondary
                        }),
                ]
                .spacing(10),
                row![
                    button("Hand")
                        .on_press(Message::ChangeTracking(AIMode::Hand))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Hand {
                            primary
                        } else {
                            secondary
                        }),
                    button("Group")
                        .on_press(Message::ChangeTracking(AIMode::Group))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Group {
                            primary
                        } else {
                            secondary
                        }),
                ]
                .spacing(10),
                column![
                    text("Exposure: "),
                    row![
                        button("Manual")
                            .on_press(Message::ChangeExposure(ExposureMode::Manual))
                            .width(Length::Fill)
                            .style(secondary),
                        button("Face")
                            .on_press(Message::ChangeExposure(ExposureMode::Face))
                            .width(Length::Fill)
                            .style(secondary),
                        button("Global")
                            .on_press(Message::ChangeExposure(ExposureMode::Global))
                            .width(Length::Fill)
                            .style(secondary),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .padding(Padding::from([10, 0])),
                column![
                    text("Presets: "),
                    row![
                        button("1")
                            .on_press(Message::ChangePresetPosition(0))
                            .width(Length::Fill)
                            .style(secondary),
                        button("2")
                            .on_press(Message::ChangePresetPosition(1))
                            .width(Length::Fill)
                            .style(secondary),
                        button("3")
                            .on_press(Message::ChangePresetPosition(2))
                            .width(Length::Fill)
                            .style(secondary),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .padding(Padding::from([10, 0])),
                column![
                    text("Tracking Speed: "),
                    row![
                        button("Standard")
                            .on_press(Message::ChangeTrackingSpeed(TrackingSpeed::Standard))
                            .style(if self.tracking_speed == TrackingSpeed::Standard {
                                primary
                            } else {
                                secondary
                            }),
                        button("Sport")
                            .on_press(Message::ChangeTrackingSpeed(TrackingSpeed::Sport))
                            .style(if self.tracking_speed == TrackingSpeed::Sport {
                                primary
                            } else {
                                secondary
                            }),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .padding(Padding::from([10, 0])),
                toggler(self.hdr_on)
                    .label("HDR".to_string())
                    .on_toggle(Message::ChangeHDR),
                toggler(self.debugging_on)
                    .label("Debugging".to_string())
                    .on_toggle(Message::ChangeDebugging),
                text(if self.awake == SleepMode::Awake {
                    format!(
                        "{tracking_mode} ({tracking_speed})",
                        tracking_mode = self.tracking.to_string(),
                        tracking_speed = self.tracking_speed.to_string()
                    )
                } else {
                    self.awake.to_string()
                })
            ];

            if self.debugging_on {
                elements = elements.push(column![
                    text_input("0x06 hex string", &self.text_input)
                        .on_input(Message::TextInput)
                        .on_submit(Message::SendCommand),
                    text_input("0x02 hex string", &self.text_input_02)
                        .on_input(Message::TextInput02)
                        .on_submit(Message::SendCommand02),
                    button("Dump 0x06")
                        .on_press(Message::HexDump)
                        .width(Length::Fill),
                    button("Dump 0x02")
                        .on_press(Message::HexDump02)
                        .width(Length::Fill),
                ]);
            }

            let c = elements
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .spacing(10)
                .padding(10)
                .into();
            c
        } else {
            text("Camera could not be found. Please check the connection of the camera.")
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
enum StartMode {
    Dashboard,
    Widget,
    Invalid,
}

fn get_start_mode() -> StartMode {
    let args: Vec<String> = std::env::args().collect();

    if let Some(start_mode_flag_pos) = args.iter().position(|a| a == ("--start-as")) {
        if let Some(start_mode_arg) = args.get(start_mode_flag_pos + 1) {
            return if start_mode_arg.eq_ignore_ascii_case("dashboard") {
                StartMode::Dashboard
            } else if start_mode_arg.eq_ignore_ascii_case("widget") {
                StartMode::Widget
            } else {
                StartMode::Invalid
            };
        }
    }

    StartMode::Dashboard
}

fn main() -> iced::Result {
    let start_mode = get_start_mode();

    if start_mode == StartMode::Invalid {
        println!(
            "Invalid start mode. Please use --start-as dashboard or --start-as widget or remove the flag."
        );
        panic!();
    }

    println!("Starting Tiny4Linux in {:?} mode", start_mode);

    let window_size = match start_mode {
        StartMode::Dashboard => Size::new(860.0, 720.0), // 43:36
        StartMode::Widget => Size::new(300.0, 550.0),    // 6:11
        StartMode::Invalid => Size::ZERO,
    };

    iced::application("Tiny4Linux", MainPanel::update, MainPanel::view)
        .theme(|_| obsbot_theme())
        .window(window::Settings {
            size: window_size,
            resizable: false,
            decorations: true,
            ..Default::default()
        })
        .subscription(MainPanel::subscription)
        .run_with(|| MainPanel::init_state())
}
