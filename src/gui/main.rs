mod styles;

use crate::styles::theme::obsbot_theme;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button;
use iced::widget::{button, column, image, row, text, text_input, toggler};
use iced::{Alignment, Length, Size, Subscription, executor, time, window};
use iced::{Application, Command, Element, Settings, Theme};
use std::time::Duration;
use tiny4linux::{AIMode, Camera, ExposureMode, OBSBotWebCam, SleepMode, TrackingSpeed};

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

impl Application for MainPanel {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
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
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Tiny4Linux".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if self.camera.is_none() {
            self.camera = Camera::new("OBSBOT Tiny 2").ok();

            if self.camera.is_none() {
                return Command::none();
            }
        } else if !self.camera.as_ref().unwrap().get_status().is_ok() {
            self.camera = None;
            return Command::none();
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

                Command::none()
            }
            Message::ChangeTracking(tracking_type) => {
                self.tracking = tracking_type;
                camera.set_ai_mode(tracking_type).unwrap();
                Command::none()
            }
            Message::ChangeTrackingSpeed(new_speed) => {
                self.tracking_speed = new_speed;
                camera.set_tracking_speed(new_speed).unwrap();
                Command::none()
            }
            Message::ChangePresetPosition(new_position) => {
                self.tracking = AIMode::NoTracking;
                camera.set_ai_mode(AIMode::NoTracking).unwrap();
                camera.goto_preset_position(new_position).unwrap();
                Command::none()
            }
            Message::ChangeHDR(new_mode) => {
                self.hdr_on = new_mode;
                camera.set_hdr_mode(new_mode).unwrap();
                Command::none()
            }
            Message::ChangeExposure(mode) => {
                camera.set_exposure_mode(mode).unwrap();
                Command::none()
            }
            Message::ChangeDebugging(new_mode) => {
                self.debugging_on = new_mode;
                Command::none()
            }
            Message::TextInput(s) => {
                self.text_input = s;
                Command::none()
            }
            Message::TextInput02(s) => {
                self.text_input_02 = s;
                Command::none()
            }
            Message::SendCommand => {
                let c = hex::decode(&self.text_input).unwrap();
                camera.send_cmd(0x2, 0x6, &c).unwrap();
                Command::none()
            }
            Message::SendCommand02 => {
                let c = hex::decode(&self.text_input_02).unwrap();
                camera.send_cmd(0x2, 0x2, &c).unwrap();
                Command::none()
            }
            Message::HexDump => {
                camera.dump().unwrap();
                Command::none()
            }
            Message::HexDump02 => {
                camera.dump_02().unwrap();
                Command::none()
            }
            Message::CheckCamera => Command::none(),
        }
    }

    fn view(&'_ self) -> Element<'_, Message> {
        if self.camera.is_some() {
            let mut elements = column![
                row![
                    text("Tiny4Linux")
                        .size(26)
                        .height(100)
                        .vertical_alignment(Vertical::Center),
                    image(if self.awake == SleepMode::Awake {
                        "src/assets/obsbot-tiny-2.png"
                    } else {
                        "src/assets/obsbot-tiny-2-side.png"
                    })
                    .height(100)
                ],
                toggler(
                    Some("Sleeping".to_string()),
                    self.awake != SleepMode::Awake,
                    Message::ChangeSleeping
                ),
                button("Static")
                    .on_press(Message::ChangeTracking(AIMode::NoTracking))
                    .style(if self.tracking == AIMode::NoTracking {
                        Button::Primary
                    } else {
                        Button::Secondary
                    }),
                button("Normal Tracking")
                    .on_press(Message::ChangeTracking(AIMode::NormalTracking))
                    .style(if self.tracking == AIMode::NormalTracking {
                        Button::Primary
                    } else {
                        Button::Secondary
                    }),
                row![
                    button("Upper Body")
                        .on_press(Message::ChangeTracking(AIMode::UpperBody))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::UpperBody {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                    button("Close-up")
                        .on_press(Message::ChangeTracking(AIMode::CloseUp))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::CloseUp {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                ]
                .spacing(10),
                row![
                    button("Headless")
                        .on_press(Message::ChangeTracking(AIMode::Headless))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Headless {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                    button("Lower Body")
                        .on_press(Message::ChangeTracking(AIMode::LowerBody))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::LowerBody {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                ]
                .spacing(10),
                row![
                    button("Desk")
                        .on_press(Message::ChangeTracking(AIMode::DeskMode))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::DeskMode {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                    button("Whiteboard")
                        .on_press(Message::ChangeTracking(AIMode::Whiteboard))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Whiteboard {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                ]
                .spacing(10),
                row![
                    button("Hand")
                        .on_press(Message::ChangeTracking(AIMode::Hand))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Hand {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                    button("Group")
                        .on_press(Message::ChangeTracking(AIMode::Group))
                        .width(Length::Fill)
                        .style(if self.tracking == AIMode::Group {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                ]
                .spacing(10),
                column![
                    text("Exposure: "),
                    row![
                        button("Manual")
                            .on_press(Message::ChangeExposure(ExposureMode::Manual))
                            .width(Length::Fill)
                            .style(Button::Secondary),
                        button("Face")
                            .on_press(Message::ChangeExposure(ExposureMode::Face))
                            .width(Length::Fill)
                            .style(Button::Secondary),
                        button("Global")
                            .on_press(Message::ChangeExposure(ExposureMode::Global))
                            .width(Length::Fill)
                            .style(Button::Secondary),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .padding([10, 0, 10, 0]),
                column![
                    text("Presets: "),
                    row![
                        button("1")
                            .on_press(Message::ChangePresetPosition(0))
                            .width(Length::Fill)
                            .style(Button::Secondary),
                        button("2")
                            .on_press(Message::ChangePresetPosition(1))
                            .width(Length::Fill)
                            .style(Button::Secondary),
                        button("3")
                            .on_press(Message::ChangePresetPosition(2))
                            .width(Length::Fill)
                            .style(Button::Secondary),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .padding([10, 0, 10, 0]),
                row![
                    text("Tracking Speed: "),
                    button("Standard")
                        .on_press(Message::ChangeTrackingSpeed(TrackingSpeed::Standard))
                        .style(if self.tracking_speed == TrackingSpeed::Standard {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                    button("Sport")
                        .on_press(Message::ChangeTrackingSpeed(TrackingSpeed::Sport))
                        .style(if self.tracking_speed == TrackingSpeed::Sport {
                            Button::Primary
                        } else {
                            Button::Secondary
                        }),
                ]
                .spacing(10)
                .padding([10, 0, 10, 0]),
                toggler(Some("HDR".to_string()), self.hdr_on, Message::ChangeHDR),
                toggler(
                    Some("Debugging".to_string()),
                    self.debugging_on,
                    Message::ChangeDebugging
                ),
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
                .align_items(Alignment::Center)
                .spacing(10)
                .padding(10)
                .into();
            c
        } else {
            text("Camera could not be found. Please check the connection of the camera.")
                .size(20)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }

    fn theme(&self) -> Self::Theme {
        obsbot_theme()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.camera.is_none() {
            time::every(Duration::from_secs(4)).map(|_| Message::CheckCamera)
        } else {
            time::every(Duration::from_secs(20)).map(|_| Message::CheckCamera)
        }
    }
}

fn main() -> iced::Result {
    MainPanel::run(Settings {
        window: window::Settings {
            size: Size::from([300, 900]),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
