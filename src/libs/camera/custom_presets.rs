// SPDX-License-Identifier: EUPL-1.2

use crate::libs::errors::T4lError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CustomPreset {
    pub name: String,
    pub pan: i32,
    pub tilt: i32,
    pub zoom: i32,
}

pub struct CustomPresetStore {
    path: PathBuf,
}

impl CustomPresetStore {
    pub fn new() -> Self {
        Self::with_path(presets_file_path())
    }

    fn with_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Vec<CustomPreset> {
        let Ok(data) = fs::read(&self.path) else {
            return Vec::new();
        };

        serde_json::from_slice(&data).unwrap_or_default()
    }

    pub fn save(&self, preset: &CustomPreset) -> Result<(), T4lError> {
        let mut presets = self.load();

        if let Some(existing) = presets.iter_mut().find(|p| p.name == preset.name) {
            *existing = preset.clone();
        } else {
            presets.push(preset.clone());
        }

        self.write(&presets)
    }

    pub fn delete(&self, name: &str) -> Result<(), T4lError> {
        let mut presets = self.load();

        let Some(index) = presets.iter().position(|p| p.name == name) else {
            return Err(T4lError::PresetNotFound(name.to_string()));
        };

        presets.remove(index);

        self.write(&presets)
    }

    fn write(&self, presets: &[CustomPreset]) -> Result<(), T4lError> {
        let data = serde_json::to_vec_pretty(presets).map_err(|_| T4lError::InvalidSetting)?;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, data)?;

        Ok(())
    }
}

fn presets_file_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join("custom_presets.json")))
        .unwrap_or_else(|| PathBuf::from("custom_presets.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::assert_err;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_store() -> CustomPresetStore {
        let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        CustomPresetStore::with_path(std::env::temp_dir().join(format!(
            "tiny4linux-custom-presets-test-{}-{}",
            std::process::id(),
            id
        )))
    }

    fn test_preset(name: &str) -> CustomPreset {
        CustomPreset {
            name: name.to_string(),
            pan: 0,
            tilt: 0,
            zoom: 0,
        }
    }

    #[test]
    fn load_missing_file_returns_empty() {
        let store = test_store();

        assert_eq!(store.load(), Vec::<CustomPreset>::new());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let store = test_store();
        let preset = test_preset("Desk");

        store.save(&preset).unwrap();

        assert_eq!(store.load(), vec![preset]);
    }

    #[test]
    fn save_overwrites_existing_preset() {
        let store = test_store();
        let mut preset = test_preset("Desk");
        store.save(&preset).unwrap();

        preset.pan = 180_000;
        store.save(&preset).unwrap();

        let presets = store.load();
        assert_eq!(presets.len(), 1);
        assert_eq!(presets[0], preset);
    }

    #[test]
    fn delete_removes_preset() {
        let store = test_store();
        store.save(&test_preset("Desk")).unwrap();
        store.save(&test_preset("Whiteboard")).unwrap();

        store.delete("Desk").unwrap();

        let presets = store.load();
        assert_eq!(presets.len(), 1);
        assert_eq!(presets[0].name, "Whiteboard");
    }

    #[test]
    fn delete_missing_preset_errors() {
        let store = test_store();

        assert_err!(
            store.delete("Missing"),
            "deleting a missing preset should error"
        );
    }

    #[test]
    fn load_corrupt_file_returns_empty() {
        let store = test_store();
        fs::write(&store.path, b"this is not json").unwrap();

        assert_eq!(store.load(), Vec::<CustomPreset>::new());
    }

    #[test]
    fn store_paths_next_to_binary() {
        let path = presets_file_path();

        assert_eq!(path.file_name().unwrap(), "custom_presets.json");
        assert!(path.parent().is_some());
    }

    #[test]
    fn writes_valid_json() {
        let store = test_store();
        store.save(&test_preset("Desk")).unwrap();

        let raw = fs::read_to_string(&store.path).unwrap();
        let parsed: Vec<CustomPreset> = serde_json::from_str(&raw).unwrap();
        assert_eq!(parsed, store.load());
        assert!(raw.contains("\"name\""));
    }
}
