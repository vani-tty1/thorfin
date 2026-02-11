use std::process::Command;
use crate::app_display::AppEntry;



pub fn update_list() {
    Command::new("pkcon")
        .arg("refresh")
        .status()
        .expect("Failed to refresh cache");
}



pub fn install_app(app: &AppEntry) {
    Command::new("pkcon")
        .arg("install")
        .arg(&app.name.to_lowercase())
        .status()
        .expect("Failed to run install!");
}
