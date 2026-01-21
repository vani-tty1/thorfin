use std::process::Command;
use crate::app_display::AppEntry;



pub fn install_app(app: &AppEntry) {
    Command::new("pkcon")
        .arg("install")
        .arg(&app.name)
        .status()
        .expect("Failed to run install!");
}
