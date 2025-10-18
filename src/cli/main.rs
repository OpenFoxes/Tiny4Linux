use clap::{Parser, Subcommand};
use clap_complete::generate;
use tiny4linux::{AIMode, Camera, OBSBotWebCam};

/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "t4l", bin_name = "t4l", version, about, long_about = None, disable_version_flag = true)]
struct Args {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Sets the camera to sleep
    Sleep,
    /// Wakes the camera up
    Wake,
    /// Turns the camera on or off (alias for `sleep` and `wake`)
    #[command(hide = true)]
    Turn {
        #[command(subcommand)]
        action: OnOffArg,
    },
    /// Controls the AI-tracking-mode of the camera
    #[command(alias = "track")]
    Tracking {
        #[command(subcommand)]
        tracking_mode: TrackingArg,
    },
    /// Controls the tracking speed of the camera
    #[command(alias = "tracking-speed")]
    Speed {
        #[command(subcommand)]
        speed: TrackingSpeedArg,
    },
    /// Sets the camera to a specific preset position previously defined in OBSBOT Center
    #[command(alias = "position")]
    Preset { position_id: i8 },
    /// Controls the HDR-mode of the camera
    Hdr {
        #[command(subcommand)]
        hdr_mode: OnOffArg,
    },
    /// Controls the exposure-mode of the camera
    Exposure {
        #[command(subcommand)]
        exposure_mode: ExposureArg,
    },
    /// Displays information about the current state of the camera
    Info,
    /// Displays the version of the CLI-tool
    Version,
    /// Generates shell-completion scripts for the CLI-tool
    Completions { shell: clap_complete::Shell },
}

#[derive(Subcommand)]
enum OnOffArg {
    On,
    Off,
}

#[derive(Subcommand)]
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

#[derive(Subcommand)]
enum TrackingSpeedArg {
    #[command(aliases = ["normal", "default", "slow", "low"])]
    Standard,
    #[command(aliases = ["sport", "high"])]
    Fast,
}

#[derive(Subcommand)]
enum ExposureArg {
    Manual,
    Global,
    Face,
}

fn main() {
    let args = Args::parse();

    let camera = Camera::new("OBSBOT Tiny 2").ok();

    if camera.is_none() {
        println!("Camera could not be found. Please check the connection of the camera.");
        return;
    }

    match &args.subcommand {
        Command::Turn { action } => match action {
            OnOffArg::On => set_sleep(false, camera.unwrap()),
            OnOffArg::Off => set_sleep(true, camera.unwrap()),
        },
        Command::Sleep => set_sleep(true, camera.unwrap()),
        Command::Wake => set_sleep(false, camera.unwrap()),
        Command::Tracking { tracking_mode } => match tracking_mode {
            TrackingArg::Static => {
                println!("Setting the camera to static (no tracking)");
                camera.unwrap().set_ai_mode(AIMode::NoTracking).unwrap();
            }
            TrackingArg::Normal => {
                println!("Setting the camera to normal tracking");
                camera.unwrap().set_ai_mode(AIMode::NormalTracking).unwrap();
            }
            TrackingArg::CloseUp => {
                println!("Setting the camera to close up tracking");
                camera.unwrap().set_ai_mode(AIMode::CloseUp).unwrap();
            }
            TrackingArg::UpperBody => {
                println!("Setting the camera to upper body tracking");
                camera.unwrap().set_ai_mode(AIMode::UpperBody).unwrap();
            }
            TrackingArg::Headless => {
                println!("Setting the camera to headless tracking");
                camera.unwrap().set_ai_mode(AIMode::Headless).unwrap();
            }
            TrackingArg::LowerBody => {
                println!("Setting the camera to lower body tracking");
                camera.unwrap().set_ai_mode(AIMode::LowerBody).unwrap();
            }
            TrackingArg::Desk => {
                println!("Setting the camera to desk tracking");
                camera.unwrap().set_ai_mode(AIMode::DeskMode).unwrap();
            }
            TrackingArg::Whiteboard => {
                println!("Setting the camera to whiteboard tracking");
                camera.unwrap().set_ai_mode(AIMode::Whiteboard).unwrap();
            }
            TrackingArg::Hand => {
                println!("Setting the camera to hand tracking");
                camera.unwrap().set_ai_mode(AIMode::Hand).unwrap();
            }
            TrackingArg::Group => {
                println!("Setting the camera to group tracking");
                camera.unwrap().set_ai_mode(AIMode::Group).unwrap();
            }
        },
        Command::Speed { speed } => match speed {
            TrackingSpeedArg::Standard => {
                println!("Setting the camera to standard tracking speed");
                camera
                    .unwrap()
                    .set_tracking_speed(tiny4linux::TrackingSpeed::Standard)
                    .unwrap();
            }
            TrackingSpeedArg::Fast => {
                println!("Setting the camera to fast tracking speed");
                camera
                    .unwrap()
                    .set_tracking_speed(tiny4linux::TrackingSpeed::Sport)
                    .unwrap();
            }
        },
        Command::Preset { position_id } => {
            println!("Stopping camera tracking");
            camera
                .as_ref()
                .unwrap()
                .set_ai_mode(AIMode::NoTracking)
                .unwrap();

            println!("Setting the camera to preset position {}", position_id);
            camera
                .unwrap()
                .goto_preset_position(position_id - 1)
                .unwrap();
        }
        Command::Hdr { hdr_mode } => match hdr_mode {
            OnOffArg::On => {
                println!("Enabling HDR");
                camera.unwrap().set_hdr_mode(true).unwrap();
            }
            OnOffArg::Off => {
                println!("Disabling HDR");
                camera.unwrap().set_hdr_mode(false).unwrap();
            }
        },
        Command::Exposure { exposure_mode } => match exposure_mode {
            ExposureArg::Manual => {
                println!("Setting the camera to manual exposure");
                camera
                    .unwrap()
                    .set_exposure_mode(tiny4linux::ExposureMode::Manual)
                    .unwrap();
            }
            ExposureArg::Global => {
                println!("Setting the camera to global exposure");
                camera
                    .unwrap()
                    .set_exposure_mode(tiny4linux::ExposureMode::Global)
                    .unwrap();
            }
            ExposureArg::Face => {
                println!("Setting the camera to face exposure");
                camera
                    .unwrap()
                    .set_exposure_mode(tiny4linux::ExposureMode::Face)
                    .unwrap();
            }
        },
        Command::Info => {
            let info = camera.unwrap().get_status();

            if info.is_err() {
                println!(
                    "Camera could not be found or gave a faulty info. Please check the connection of the camera."
                );
                return;
            } else {
                let info = info.unwrap();

                println!("Camera status:");
                println!("  Sleep Mode: {}", info.awake);
                println!("  AI Mode: {}", info.ai_mode);
                println!("  Tracking Speed: {}", info.speed);
                println!("  HDR: {}", info.hdr_on);
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

fn set_sleep(sleep: bool, camera: Camera) {
    if sleep {
        println!("Setting the camera to sleep");
        camera.set_sleep_mode(tiny4linux::SleepMode::Sleep).unwrap();
    } else {
        println!("Waking up the camera");
        camera.set_sleep_mode(tiny4linux::SleepMode::Awake).unwrap();
    }
}
