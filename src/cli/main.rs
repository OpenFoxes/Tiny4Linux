use clap::{Parser, Subcommand};
use clap_complete::generate;
use dialoguer::{FuzzySelect, Select};
use tiny4linux::{AIMode, Camera, SleepMode, Tiny2Camera};

/// Simple program to greet a person
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
    /// Sets the camera to sleep
    Sleep,
    /// Wakes the camera up
    Wake,
    /// Turns the camera on or off (alias for `sleep` and `wake`)
    #[command(hide = true, subcommand_required = false)]
    Turn {
        #[command(subcommand)]
        action: Option<OnOffArg>,
    },
    /// Controls the AI-tracking-mode of the camera
    #[command(alias = "track", subcommand_required = false)]
    Tracking {
        #[command(subcommand)]
        tracking_mode: Option<TrackingArg>,
    },
    /// Controls the tracking speed of the camera
    #[command(alias = "tracking-speed", subcommand_required = false)]
    Speed {
        #[command(subcommand)]
        speed: Option<TrackingSpeedArg>,
    },
    /// Sets the camera to a specific preset position previously defined in OBSBOT Center
    #[command(alias = "position", subcommand_required = false)]
    Preset { position_id: Option<i8> },
    /// Controls the HDR-mode of the camera
    Hdr {
        #[command(subcommand)]
        hdr_mode: Option<OnOffArg>,
    },
    /// Controls the exposure-mode of the camera
    Exposure {
        #[command(subcommand)]
        exposure_mode: Option<ExposureArg>,
    },
    /// Displays information about the current state of the camera
    Info,
    /// Displays the version of the CLI-tool
    Version,
    /// Generates shell-completion scripts for the CLI-tool
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
    let args = Args::parse();

    let mut camera = Camera::new("OBSBOT Tiny 2").ok();

    if camera.is_none() {
        println!("Camera could not be found. Please check the connection of the camera.");
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
                println!(
                    "Camera could not be found or gave a faulty info. Please check the connection of the camera."
                );
                return;
            } else {
                let info = info.unwrap();

                println!("Camera status:");
                println!("  💤  Sleep Mode: {}", info.awake);
                println!("  🤖  AI Mode: {}", info.ai_mode);
                println!("  🏃  Tracking Speed: {}", info.speed);
                println!("  💐  HDR: {}", info.hdr_on);
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
            println!("Setting the camera to sleep");
            camera.set_sleep_mode(SleepMode::Sleep).unwrap();
        }
        Some(OnOffArg::On) => {
            println!("Waking up the camera");
            camera.set_sleep_mode(SleepMode::Awake).unwrap();
        }
        None => {
            let options = [
                SelectionOption {
                    result: OnOffArg::On,
                    option: "awake",
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: "sleeping",
                },
                SelectionOption {
                    result: OnOffArg::On,
                    option: "on",
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: "off",
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: "asleep",
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt("The camera should be")
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
    match tracking_mode {
        Some(TrackingArg::Static) => {
            println!("Setting the camera to static (no tracking)");
            camera.set_ai_mode(AIMode::NoTracking).unwrap();
        }
        Some(TrackingArg::Normal) => {
            println!("Setting the camera to normal tracking");
            camera.set_ai_mode(AIMode::NormalTracking).unwrap();
        }
        Some(TrackingArg::CloseUp) => {
            println!("Setting the camera to close up tracking");
            camera.set_ai_mode(AIMode::CloseUp).unwrap();
        }
        Some(TrackingArg::UpperBody) => {
            println!("Setting the camera to upper body tracking");
            camera.set_ai_mode(AIMode::UpperBody).unwrap();
        }
        Some(TrackingArg::Headless) => {
            println!("Setting the camera to headless tracking");
            camera.set_ai_mode(AIMode::Headless).unwrap();
        }
        Some(TrackingArg::LowerBody) => {
            println!("Setting the camera to lower body tracking");
            camera.set_ai_mode(AIMode::LowerBody).unwrap();
        }
        Some(TrackingArg::Desk) => {
            println!("Setting the camera to desk tracking");
            camera.set_ai_mode(AIMode::DeskMode).unwrap();
        }
        Some(TrackingArg::Whiteboard) => {
            println!("Setting the camera to whiteboard tracking");
            camera.set_ai_mode(AIMode::Whiteboard).unwrap();
        }
        Some(TrackingArg::Hand) => {
            println!("Setting the camera to hand tracking");
            camera.set_ai_mode(AIMode::Hand).unwrap();
        }
        Some(TrackingArg::Group) => {
            println!("Setting the camera to group tracking");
            camera.set_ai_mode(AIMode::Group).unwrap();
        }
        None => {
            let options = [
                SelectionOption {
                    result: TrackingArg::Static,
                    option: "static (no tracking)",
                },
                SelectionOption {
                    result: TrackingArg::Normal,
                    option: "normal tracking",
                },
                SelectionOption {
                    result: TrackingArg::CloseUp,
                    option: "close up tracking",
                },
                SelectionOption {
                    result: TrackingArg::UpperBody,
                    option: "upper body tracking",
                },
                SelectionOption {
                    result: TrackingArg::Headless,
                    option: "headless tracking",
                },
                SelectionOption {
                    result: TrackingArg::LowerBody,
                    option: "lower body tracking",
                },
                SelectionOption {
                    result: TrackingArg::Desk,
                    option: "desk",
                },
                SelectionOption {
                    result: TrackingArg::Whiteboard,
                    option: "whiteboard",
                },
                SelectionOption {
                    result: TrackingArg::Hand,
                    option: "hand tracking",
                },
                SelectionOption {
                    result: TrackingArg::Group,
                    option: "group tracking",
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt("What tracking type should be set?")
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
            println!("Setting the camera to standard tracking speed");
            camera
                .set_tracking_speed(tiny4linux::TrackingSpeed::Standard)
                .unwrap();
        }
        Some(TrackingSpeedArg::Fast) => {
            println!("Setting the camera to fast tracking speed");
            camera
                .set_tracking_speed(tiny4linux::TrackingSpeed::Sport)
                .unwrap();
        }
        None => {
            let options = [
                SelectionOption {
                    result: TrackingSpeedArg::Standard,
                    option: "Standard (slower)",
                },
                SelectionOption {
                    result: TrackingSpeedArg::Fast,
                    option: "Sport (fast)",
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt("Select the cameras tracking speed!")
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
            .with_prompt("Select a predefined position preset!")
            .default(0)
            .items(options)
            .interact()
            .unwrap();

        return evaluate_preset_arg(Option::from(options[selection]), camera);
    }

    println!("Stopping camera tracking");
    camera.set_ai_mode(AIMode::NoTracking).unwrap();

    println!(
        "Setting the camera to preset position {}",
        position_id.unwrap()
    );
    camera
        .goto_preset_position(position_id.unwrap() - 1)
        .unwrap();
}

fn evaluate_hdr_arg(hdr_mode: Option<OnOffArg>, camera: Camera) {
    match hdr_mode {
        Some(OnOffArg::On) => {
            println!("Enabling HDR");
            camera.set_hdr_mode(true).unwrap();
        }
        Some(OnOffArg::Off) => {
            println!("Disabling HDR");
            camera.set_hdr_mode(false).unwrap();
        }
        None => {
            let options = [
                SelectionOption {
                    result: OnOffArg::On,
                    option: "HDR on",
                },
                SelectionOption {
                    result: OnOffArg::Off,
                    option: "HDR off",
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt("Select the HDR state!")
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
            println!("Setting the camera to manual exposure");
            camera
                .set_exposure_mode(tiny4linux::ExposureMode::Manual)
                .unwrap();
        }
        Some(ExposureArg::Global) => {
            println!("Setting the camera to global exposure");
            camera
                .set_exposure_mode(tiny4linux::ExposureMode::Global)
                .unwrap();
        }
        Some(ExposureArg::Face) => {
            println!("Setting the camera to face exposure");
            camera
                .set_exposure_mode(tiny4linux::ExposureMode::Face)
                .unwrap();
        }
        None => {
            let options = [
                SelectionOption {
                    result: ExposureArg::Manual,
                    option: "Manual",
                },
                SelectionOption {
                    result: ExposureArg::Global,
                    option: "Global",
                },
                SelectionOption {
                    result: ExposureArg::Face,
                    option: "Face",
                },
            ];
            let selection = FuzzySelect::new()
                .with_prompt("Choose the exposure mode")
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
