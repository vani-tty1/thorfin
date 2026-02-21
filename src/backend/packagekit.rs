use std::process::Command;
use crate::app_display::AppEntry;


pub async fn update_list() {
    Command::new("pkgcli")
        .arg("refresh")
        .status()
        .expect("Failed to refresh cache");
}



pub async fn install_app(app: &AppEntry) {
    Command::new("pkgcli")
        .arg("install")
        .arg(&app.name.to_lowercase())
        .status()
        .expect("Failed to run install!");
}
