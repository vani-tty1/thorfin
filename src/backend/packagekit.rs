use std::process::Command;
use crate::app_display::AppEntry;
use std::path::Path;

fn is_flatpak() -> bool {
    Path::new("/.flatpak-info").exists()
}

pub async fn update_list() {
    let (cmd, args) = if is_flatpak() {
        ("flatpak-spawn", vec!["--host", "pkcon", "refresh"])
    } else {
        ("pkcon", vec!["refresh"])
    };

    if let Err(e) = Command::new(cmd).args(args).status() {
        eprintln!("Failed to refresh cache: {}", e);
    }
}

pub async fn install_app(app: &AppEntry) {
    let app_name = app.name.to_lowercase();
    let (cmd, args) = if is_flatpak() {
        ("flatpak-spawn", vec!["--host", "pkcon", "install", "-y", &app_name])
    } else {
        ("pkcon", vec!["install", "-y", &app_name])
    };

    if let Err(e) = Command::new(cmd).args(args).status() {
        eprintln!("Failed to run install for {}: {}", app.name, e);
    }
}