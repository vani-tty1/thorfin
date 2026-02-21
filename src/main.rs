mod about;
mod window;
use window::window_init;
use libadwaita as adw;
use adw::prelude::*;
use adw::{Application};
mod app_display;
mod backend;





#[tokio::main]
async fn main() {
    let app = Application::builder()
        .application_id("io.github.vani1_2.thorfin")
        .build();
    app.connect_startup(about::abt_shortc);
    app.connect_activate(window_init);
    app.run();
}
