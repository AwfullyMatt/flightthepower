use bevy::{
    prelude::*,
    scene::ron::{
        de::from_reader,
        ser::{to_writer_pretty, PrettyConfig},
    },
};
use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, ErrorKind, Result},
};

use crate::{
    game::{PowerUnlockFlags, Powers, TotalPower},
    settings::Settings,
    AppState,
};

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn name(&self) -> &str {
        "Save Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_event::<Save>()
            .add_systems(OnEnter(AppState::Exit), evw_save)
            .add_systems(Update, evr_save);
    }
}

pub trait Saveable {
    fn save(&self, filename: &str) -> Result<()>;
    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized;
}

pub fn format_save<'a, T>(t: &'a T, filename: &str) -> Result<()>
where
    T: Saveable + Serialize + Deserialize<'a>,
{
    // Get platform-specific project directory
    let project_dirs = ProjectDirs::from("me", "awfullymatt", "flightthepower")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Failed to find project directory"))?;

    // Create save directory if it doesn't exist
    let save_dir = project_dirs.data_dir().join("ron");
    std::fs::create_dir_all(&save_dir)?;

    // Create full file path
    let path = save_dir.join(filename);
    let file = File::create(&path)?;

    // Rest of your serialization code remains the same
    let t =
        to_writer_pretty(file, t, PrettyConfig::new()).map_err(|e| Error::new(ErrorKind::Other, e));

    info!("[SAVED] {}", path.display());
    t
}

pub fn format_load<T>(filename: &str) -> Result<T>
where
    T: Saveable + Serialize + DeserializeOwned,
{
    // Get platform-specific project directory (same as in save function)
    let project_dirs = ProjectDirs::from("me", "awfullymatt", "flightthepower")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Failed to find project directory"))?;

    // Build the full load path
    let save_dir = project_dirs.data_dir().join("ron");
    let path = save_dir.join(filename);

    let file = File::open(&path)?;
    let t = from_reader(file).map_err(|e| Error::new(ErrorKind::Other, e));

    info!("[LOADED] {}", path.display());
    t
}

/* pub fn format_save<'a, T>(t: &'a T, filename: &str) -> Result<()>
where
    T: Saveable + Serialize + Deserialize<'a>,
{
    let path = format!("{}/ron/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let file = File::create(&path)?;
    let t =
        to_writer_pretty(file, t, PrettyConfig::new()).map_err(|e| Error::new(ErrorKind::Other, e));
    info!("[SAVED] {path}");
    t
} */

/* pub fn format_load<T>(filename: &str) -> Result<T>
where
    T: Saveable + Serialize + DeserializeOwned,
{
    let path = format!("{}/ron/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let file = File::open(&path)?;
    let t = from_reader(file).map_err(|e| Error::new(ErrorKind::Other, e));
    info!("[LOADED] {path}");
    t
} */

#[derive(Event)]
pub struct Save;

fn evr_save(
    mut evr_save: EventReader<Save>,
    settings: Res<Settings>,
    power_flags: Res<PowerUnlockFlags>,
    total_power: Res<TotalPower>,
    powers: Res<Powers>,
) {
    for _ev in evr_save.read() {
        info!("[EVENT] [READ] Save Game");
        let _ = settings.save("settings.ron");
        let _ = power_flags.save("power_unlocks.ron");
        let _ = total_power.save("total_power.ron");
        let _ = powers.save("powers.ron");
    }
}

fn evw_save(mut evw_save: EventWriter<Save>, mut evw_exit: EventWriter<AppExit>) {
    evw_save.send(Save);
    info!("[EVENT] [WRITE] Save Game.");
    evw_exit.send(AppExit::Success);
}
