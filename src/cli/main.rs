// SPDX-License-Identifier: EUPL-1.2

use clap::{Parser, Subcommand};
use clap_complete::generate;
use dialoguer::{FuzzySelect, Select};
use rust_i18n::{i18n, set_locale, t};
use tiny4linux::{AIMode, Camera, SleepMode, Tiny2Camera, get_language};

i18n!("src/locales", fallback = "en");

#[derive(Parser)]
#[command(name = "t4l", bin_name = "t4l", version, about, long_about = None, disable_version_flag = true)]
struct Args {
    #[command(subcommand)]
    subcommand: Command,
    #[arg(short, long, help = "Turns the debug logging on", global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = t!("cli.help.sleep"))]
    Sleep,
    #[command(about = t!("cli.help.wake"))]
    Wake,
    #[command(hide = true, subcommand_required = false, about = t!("cli.help.turn.command"))]
    Turn {
        #[command(subcommand)]
        action: Option<OnOffArg>,
    },
    #[command(alias = "track", subcommand_required = false, about = t!("cli.help.tracking"))]
    Tracking {
        #[command(subcommand)]
        tracking_mode: Option<TrackingArg>,
    },
    #[command(alias = "tracking-speed", subcommand_required = false, about = t!("cli.help.speed"))]
    Speed {
        #[command(subcommand)]
        speed: Option<TrackingSpeedArg>,
    },
    #[command(alias = "position", subcommand_required = false, about = t!("cli.help.preset"))]
    Preset { position_id: Option<i8> },
    #[command(about = t!("cli.help.hdr"))]
    Hdr {
        #[command(subcommand)]
        hdr_mode: Option<OnOffArg>,
    },
    #[command(about = t!("cli.help.exposure"))]
    Exposure {
        #[command(subcommand)]
        exposure_mode: Option<ExposureArg>,
    },
    #[command(about = t!("cli.help.info"))]
    Info,
    #[command(about = t!("cli.help.version"))]
    Version,
    #[command(about = t!("cli.help.completions"))]
    Completions { shell: clap_complete::Shell },
}

#[derive(Subcommand, Clone)]
enum OnOffArg {
    On,
    Off,
}

#[derive(Subcommand, Clone)]
enum TrackingArg {
    #[command(aliases = ["none", "off"])]
    Static,
    #[command(aliases = ["standard", "on"])]
    Normal,
    #[command(alias = "close")]
    CloseUp,
    UpperBody,
    Headless,
    LowerBody,
    Desk,
    Whiteboard,
    #[command(alias = "point")]
    Hand,
    Group,
}

#[derive(Subcommand, Clone)]
enum TrackingSpeedArg {
    #[command(aliases = ["normal", "default", "slow", "low"])]
    Standard,
    #[command(aliases = ["sport", "high"])]
    Fast,
}

#[derive(Subcommand, Clone)]
enum ExposureArg {
    Manual,
    Global,
    Face,
}

fn main() {
    let language = get_language(false);
    set_locale(language.as_str());

    let args = Args::parse();

    let mut camera = Camera::new("OBSBOT Tiny 2").ok();

    if camera.is_none() {
        println!("{}", t!("shared.errors.no_camera"));
        return;
    }

    if args.verbose {
        camera.as_mut().unwrap().set_debugging(true);
    }

    let camera = camera.unwrap();

    match &args.subcommand {
        Command::Turn { action } => evaluate_sleep_arg(action.clone(), camera),
        Command::Sleep => evaluate_sleep_arg(Option::from(OnOffArg::Off), camera),
        Command::Wake => evaluate_sleep_arg(Option::from(OnOffArg::On), camera),
        Command::Tracking { tracking_mode } => evaluate_tracking_arg(tracking_mode.clone(), camera),
        Command::Speed { speed } => evaluate_speed_arg(speed.clone(), camera),
        Command::Preset { position_id } => evaluate_preset_arg(*position_id, camera),
        Command::Hdr { hdr_mode } => evaluate_hdr_arg(hdr_mode.clone(), camera),
        Command::Exposure { exposure_mode } => evaluate_exposure_arg(exposure_mode.clone(), camera),
        Command::Info => {
            let info = camera.get_status();

            if info.is_err() {
                println!("{}", t!("cli.errors.info_error"),);
                return;
            } else {
                let info = info.unwrap();

                println!("{}:", t!("shared.info.camera_status"));
                println!("  💤  {}: {}", t!("shared.info.sleep_mode"), info.awake);
                println!("  🤖  {}: {}", t!("shared.info.ai_mode"), info.ai_mode);
                println!("  🏃  {}: {}", t!("shared.info.tracking_speed"), info.speed);
                println!("  💐  {}: {}", t!("shared.info.hdr"), info.hdr_on);
            }
        }
        Command::Version => {
            println!("t4l version: {}", env!("CARGO_PKG_VERSION"));
        }
        Command::Completions { shell } => {
            use clap::CommandFactory;
            let mut cmd = Args::command();
            generate(*shell, &mut cmd, "t4l", &mut std::io::stdout());
        }
    }
}

struct SelectionOption<'a, T> {
    result: T,
    option: &'a str,
}

fn evaluate_sleep_arg(state: Option<OnOffArg>, camera: Camera) {
    match state {
        Some(OnOffArg::Off) => {
            println!("{}", t!("cli.sleep.response_to_sleep"));
            camera.set_sleep_mode(SleepMode::Sleep).unwrap();
        }
        Some(OnOffArg::On) => {
            println!("{}", t!("cli.sleep.response_to_awake"));
            camera.set_sleep_mode(SleepMode::Awake).unwrap();
        }
        None => {
            let option_on1 = t!("cli.sleep.option_on1");
            let option_off1 = t!("cli.sleep.option_off1");
            let option_on2 = t!("cli.sleep.option_on2");
            let option_off2 = t!("cli.sleep.option_off2");
            let option_off3 = t!("cli.sleep.option_off3");

            let options = [
                SelectionOption {
                    result: OnOffArg::On,
                    option: &option_on1,
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: &option_off1,
                },
                SelectionOption {
                    result: OnOffArg::On,
                    option: &option_on2,
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: &option_off2,
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: &option_off3,
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt(format!("{}", t!("cli.sleep.request_should_be")))
                .default(0)
                .items(
                    &options
                        .iter()
                        .map(|option| option.option)
                        .collect::<Vec<&str>>(),
                )
                .interact()
                .unwrap();

            evaluate_sleep_arg(Option::from(options[selection].result.clone()), camera);
        }
    }
}

fn evaluate_tracking_arg(tracking_mode: Option<TrackingArg>, camera: Camera) {
    let response_setting_to = t!("cli.tracking_mode.response_setting_to");
    let static_mode = t!("cli.tracking_mode.static");
    let normal = t!("cli.tracking_mode.normal");
    let close_up = t!("cli.tracking_mode.close_up");
    let upper_body = t!("cli.tracking_mode.upper_body");
    let headless = t!("cli.tracking_mode.headless");
    let lower_body = t!("cli.tracking_mode.lower_body");
    let desk = t!("cli.tracking_mode.desk");
    let whiteboard = t!("cli.tracking_mode.whiteboard");
    let hand = t!("cli.tracking_mode.hand");
    let group = t!("cli.tracking_mode.group");

    match tracking_mode {
        Some(TrackingArg::Static) => {
            println!("{} {}", response_setting_to, static_mode);
            camera.set_ai_mode(AIMode::NoTracking).unwrap();
        }
        Some(TrackingArg::Normal) => {
            println!("{} {}", response_setting_to, normal);
            camera.set_ai_mode(AIMode::NormalTracking).unwrap();
        }
        Some(TrackingArg::CloseUp) => {
            println!("{} {}", response_setting_to, close_up);
            camera.set_ai_mode(AIMode::CloseUp).unwrap();
        }
        Some(TrackingArg::UpperBody) => {
            println!("{} {}", response_setting_to, upper_body);
            camera.set_ai_mode(AIMode::UpperBody).unwrap();
        }
        Some(TrackingArg::Headless) => {
            println!("{} {}", response_setting_to, headless);
            camera.set_ai_mode(AIMode::Headless).unwrap();
        }
        Some(TrackingArg::LowerBody) => {
            println!("{} {}", response_setting_to, lower_body);
            camera.set_ai_mode(AIMode::LowerBody).unwrap();
        }
        Some(TrackingArg::Desk) => {
            println!("{} {}", response_setting_to, desk);
            camera.set_ai_mode(AIMode::DeskMode).unwrap();
        }
        Some(TrackingArg::Whiteboard) => {
            println!("{} {}", response_setting_to, whiteboard);
            camera.set_ai_mode(AIMode::Whiteboard).unwrap();
        }
        Some(TrackingArg::Hand) => {
            println!("{} {}", response_setting_to, hand);
            camera.set_ai_mode(AIMode::Hand).unwrap();
        }
        Some(TrackingArg::Group) => {
            println!("{} {}", response_setting_to, group);
            camera.set_ai_mode(AIMode::Group).unwrap();
        }
        None => {
            let options = [
                SelectionOption {
                    result: TrackingArg::Static,
                    option: &static_mode,
                },
                SelectionOption {
                    result: TrackingArg::Normal,
                    option: &normal,
                },
                SelectionOption {
                    result: TrackingArg::CloseUp,
                    option: &close_up,
                },
                SelectionOption {
                    result: TrackingArg::UpperBody,
                    option: &upper_body,
                },
                SelectionOption {
                    result: TrackingArg::Headless,
                    option: &headless,
                },
                SelectionOption {
                    result: TrackingArg::LowerBody,
                    option: &lower_body,
                },
                SelectionOption {
                    result: TrackingArg::Desk,
                    option: &desk,
                },
                SelectionOption {
                    result: TrackingArg::Whiteboard,
                    option: &whiteboard,
                },
                SelectionOption {
                    result: TrackingArg::Hand,
                    option: &hand,
                },
                SelectionOption {
                    result: TrackingArg::Group,
                    option: &group,
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt(t!("cli.tracking_mode.request_should_be"))
                .default(0)
                .items(
                    &options
                        .iter()
                        .map(|option| option.option)
                        .collect::<Vec<&str>>(),
                )
                .interact()
                .unwrap();

            evaluate_tracking_arg(Option::from(options[selection].result.clone()), camera);
        }
    }
}

fn evaluate_speed_arg(speed: Option<TrackingSpeedArg>, camera: Camera) {
    match speed {
        Some(TrackingSpeedArg::Standard) => {
            println!("{}", t!("cli.tracking_speed.response_to_standard"));
            camera
                .set_tracking_speed(tiny4linux::TrackingSpeed::Standard)
                .unwrap();
        }
        Some(TrackingSpeedArg::Fast) => {
            println!("{}", t!("cli.tracking_speed.response_to_fast"));
            camera
                .set_tracking_speed(tiny4linux::TrackingSpeed::Sport)
                .unwrap();
        }
        None => {
            let option_standard = t!("cli.tracking_speed.option_standard");
            let option_sport = t!("cli.tracking_speed.option_sport");

            let options = [
                SelectionOption {
                    result: TrackingSpeedArg::Standard,
                    option: &option_standard,
                },
                SelectionOption {
                    result: TrackingSpeedArg::Fast,
                    option: &option_sport,
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt(t!("cli.tracking_speed.request_should_be"))
                .default(0)
                .items(
                    &options
                        .iter()
                        .map(|option| option.option)
                        .collect::<Vec<&str>>(),
                )
                .interact()
                .unwrap();

            evaluate_speed_arg(Option::from(options[selection].result.clone()), camera);
        }
    }
}

fn evaluate_preset_arg(position_id: Option<i8>, camera: Camera) {
    if position_id.is_none() {
        let options = [1, 2, 3];
        let selection = Select::new()
            .with_prompt(t!("cli.preset_position.request_position_id"))
            .default(0)
            .items(options)
            .interact()
            .unwrap();

        return evaluate_preset_arg(Option::from(options[selection]), camera);
    }

    println!("{}", t!("cli.preset_position.stopping_tracking"));
    camera.set_ai_mode(AIMode::NoTracking).unwrap();

    println!(
        "{}",
        t!(
            "cli.preset_position.response_to_position",
            position_id = position_id.unwrap()
        ),
    );
    camera
        .goto_preset_position(position_id.unwrap() - 1)
        .unwrap();
}

fn evaluate_hdr_arg(hdr_mode: Option<OnOffArg>, camera: Camera) {
    match hdr_mode {
        Some(OnOffArg::On) => {
            println!("{}", t!("cli.hdr.response_to_hdr_on"));
            camera.set_hdr_mode(true).unwrap();
        }
        Some(OnOffArg::Off) => {
            println!("{}", t!("cli.hdr.response_to_hdr_off"));
            camera.set_hdr_mode(false).unwrap();
        }
        None => {
            let option_on = t!("shared.options.hdr.on");
            let option_off = t!("shared.options.hdr.off");

            let options = [
                SelectionOption {
                    result: OnOffArg::On,
                    option: &option_on,
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: &option_off,
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt(t!("cli.hdr.request_should_be"))
                .default(0)
                .items(
                    &options
                        .iter()
                        .map(|option| option.option)
                        .collect::<Vec<&str>>(),
                )
                .interact()
                .unwrap();

            evaluate_hdr_arg(Option::from(options[selection].result.clone()), camera);
        }
    }
}

fn evaluate_exposure_arg(exposure_mode: Option<ExposureArg>, camera: Camera) {
    match exposure_mode {
        Some(ExposureArg::Manual) => {
            println!("{}", t!("cli.exposure.response_to_manual"));
            camera
                .set_exposure_mode(tiny4linux::ExposureMode::Manual)
                .unwrap();
        }
        Some(ExposureArg::Global) => {
            println!("{}", t!("cli.exposure.response_to_global"));
            camera
                .set_exposure_mode(tiny4linux::ExposureMode::Global)
                .unwrap();
        }
        Some(ExposureArg::Face) => {
            println!("{}", t!("cli.exposure.response_to_face"));
            camera
                .set_exposure_mode(tiny4linux::ExposureMode::Face)
                .unwrap();
        }
        None => {
            let option_manual = t!("cli.exposure.option_manual");
            let option_global = t!("cli.exposure.option_global");
            let option_face = t!("cli.exposure.option_face");

            let options = [
                SelectionOption {
                    result: ExposureArg::Manual,
                    option: &option_manual,
                },
                SelectionOption {
                    result: ExposureArg::Global,
                    option: &option_global,
                },
                SelectionOption {
                    result: ExposureArg::Face,
                    option: &option_face,
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt(t!("cli.exposure.request_should_be"))
                .default(0)
                .items(
                    &options
                        .iter()
                        .map(|option| option.option)
                        .collect::<Vec<&str>>(),
                )
                .interact()
                .unwrap();

            evaluate_exposure_arg(Option::from(options[selection].result.clone()), camera);
        }
    }
}
