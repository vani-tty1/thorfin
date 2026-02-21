use crate::app_display::AppEntry;
use serde_json::Value;

pub async fn fetch_popular() -> Vec<AppEntry> {
    let url = "https://flathub.org/api/v2/apps";
    
    let response = match reqwest::get(url).await {
        Ok(res) => res,
        Err(e) => { eprintln!("HTTP Error: {}", e); return vec![]; }
    };
    
    let apps: Vec<Value> = match response.json().await {
        Ok(json) => json,
        Err(e) => { eprintln!("JSON Error: {}", e); return vec![]; }
    };
    
    apps.into_iter().filter_map(|app| {
        let id = app.get("flatpakAppId")?.as_str()?.to_string();
        let name = app.get("name").and_then(|n| n.as_str()).unwrap_or("Unknown").to_string();
        let summary = app.get("summary").and_then(|s| s.as_str()).unwrap_or("No description").to_string();
        
        Some(AppEntry { id, name, summary })
    }).take(20).collect()
}