use std::process::Command;
use crate::app_display::AppEntry;
use std::path::Path;

pub async fn update_list() {
    let (cmd, args) = if is_flatpak() {
        ("flatpak-spawn", vec!["--host", "pkgcli", "refresh", "force"])
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
        ("flatpak-spawn", vec!["--host", "pkgcli", "install", "-y", &app_name])
    } else {
        ("pkcon", vec!["install", "-y", &app_name])
    };

    if let Err(e) = Command::new(cmd).args(args).status() {
        eprintln!("Failed to run install for {}: {}", app.name, e);
    }
}


fn is_flatpak() -> bool {
    Path::new("/.flatpak-info").exists()
}

pub async fn search_repo(query: &str) -> Vec<AppEntry> {
    let (cmd, args) = if is_flatpak() {
        ("flatpak-spawn", vec!["--host", "pkgcli", "--plain", "search", "name", query])
    } else {
        ("pkcon", vec!["--plain", "search", "name", query])
    };

    println!("Running: {} {:?}", cmd, args);

    let Ok(output) = Command::new(cmd).args(args).output() else {
        eprintln!("Failed to execute command!");
        return vec![];
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- PKCON RAW OUTPUT ---");
    println!("{}", stdout);
    println!("------------------------");

    let mut apps = Vec::new();
    
    for line in stdout.lines() {
        if line.starts_with("Available") || line.starts_with("Installed") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() >= 3 {
                let full_package_name = parts[1];
                let name = full_package_name.split('-').next().unwrap_or("Unknown").to_string();
                
                let summary = if parts.len() >= 4 {
                    parts[3..].join(" ")
                } else {
                    format!("Package: {}", full_package_name) // Fallback summary
                };
    
                apps.push(AppEntry {
                    id: full_package_name.to_string(), 
                    name,
                    summary,
                });
            }
        }
    }
    apps
}