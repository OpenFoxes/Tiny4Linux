fn main() {
    let gui_feature_enabled = std::env::var("CARGO_FEATURE_GUI").is_ok();

    if !gui_feature_enabled {
        return;
    }

    build_gui_assets();
}

#[cfg(feature = "gui")]
fn build_gui_assets() {
    use std::fs;
    use std::path::Path;
    use tiny4linux_assets::absolute_path_for_t4l_asset;

    let asset_src = absolute_path_for_t4l_asset("generated/png/title-icon/v2.0-soft-shadow.png");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("assets/icon.png");

    fs::create_dir_all(target_dir.parent().unwrap()).unwrap();
    fs::copy(&asset_src, &target_dir).unwrap();

    println!("cargo:rerun-if-changed={}", asset_src.display());
}

#[cfg(not(feature = "gui"))]
fn build_gui_assets() {
    // Noop
}
