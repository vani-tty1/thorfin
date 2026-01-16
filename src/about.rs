use libadwaita as adw;
use adw::prelude::*;
use adw::AboutDialog;
use gtk4 as gtk;
use gtk::gio;

pub fn abt_shortc(app: &adw::Application) {
    let action_abt = gio::SimpleAction::new("about", None);
    let app_weak = app.downgrade();
    
    action_abt.connect_activate(move |_, _| {
        if let Some(app) = app_weak.upgrade() {
            let active_window = app.active_window();
            show_dialog(active_window.as_ref());
        }
    });
    
    app.add_action(&action_abt);
}

fn show_dialog(parent: Option<&gtk::Window>) {
    let dialog = AboutDialog::builder()
        .application_icon("org.gnome.Software")
        .application_name("Thorfin")
        .developer_name("Vani1-2")
        .version("0.1.0")
        .website("https://github.com/Vani1-2/thorfin")
        .copyright("© 2026 Vani1-2")
        .license_type(gtk::License::Gpl30)
        .build();
    
    dialog.present(parent);
}