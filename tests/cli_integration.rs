use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_cmd::cargo_bin;

fn bin() -> Command {
    let mut cmd = Command::new(cargo_bin!("tiny4linux-cli"));
    cmd.env("T4L_MOCK", "1"); // ensure mock transport (no USB needed)
    cmd
}

#[test]
fn version_prints_version() {
    let mut cmd = bin();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("t4l version:"));
}

#[test]
fn info_prints_status() {
    let mut cmd = bin();
    cmd.arg("info")
        .assert()
        .success()
        .stdout(predicate::str::contains("Camera status:"));
}

#[test]
fn sleep_prints_message() {
    let mut cmd = bin();
    cmd.arg("sleep")
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting the camera to sleep"));
}

#[test]
fn wake_prints_message() {
    let mut cmd = bin();
    cmd.arg("wake")
        .assert()
        .success()
        .stdout(predicate::str::contains("Waking up the camera"));
}

#[test]
fn tracking_normal_prints_message() {
    let mut cmd = bin();
    cmd.args(["tracking", "normal"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting the camera to normal tracking"));
}

#[test]
fn speed_fast_prints_message() {
    let mut cmd = bin();
    cmd.args(["speed", "fast"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting the camera to fast tracking speed"));
}

#[test]
fn preset_two_prints_messages() {
    let mut cmd = bin();
    cmd.args(["preset", "2"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopping camera tracking"))
        .stdout(predicate::str::contains("Setting the camera to preset position 2"));
}

#[test]
fn hdr_on_prints_message() {
    let mut cmd = bin();
    cmd.args(["hdr", "on"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Enabling HDR"));
}

#[test]
fn hdr_off_prints_message() {
    let mut cmd = bin();
    cmd.args(["hdr", "off"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Disabling HDR"));
}

#[test]
fn exposure_face_prints_message() {
    let mut cmd = bin();
    cmd.args(["exposure", "face"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting the camera to face exposure"));
}

#[test]
fn exposure_global_prints_message() {
    let mut cmd = bin();
    cmd.args(["exposure", "global"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting the camera to global exposure"));
}

#[test]
fn exposure_manual_prints_message() {
    let mut cmd = bin();
    cmd.args(["exposure", "manual"]) // non-interactive
        .assert()
        .success()
        .stdout(predicate::str::contains("Setting the camera to manual exposure"));
}
