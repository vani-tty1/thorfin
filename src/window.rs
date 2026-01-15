use libadwaita as adw;
use adw::prelude::*;
use adw::{HeaderBar, ApplicationWindow, Application};
use gtk4::{self as gtk, MenuButton, Orientation, gio};
use gtk::{Box};

pub fn window_init(main: &Application) {
    //notes to self:
    // this thing is like this:
    // dont forgor it future self
    // Window
    //  └── content(The (Parent) Vertical Box)
    //      ├── 1. head_bar (Added first -> Sits at the Top)
    //      └── 2. main_dis (Added second -> Sits at the Bottom)
    //              └── spinner (Inside main_dis)
    
    
    
    let main_menu = gio::Menu::new();
    main_menu.append(Some("Preferences"), Some("app.preferences"));
    main_menu.append(Some("About"), Some("app.about"));

    let menu_btn = MenuButton::builder()
        .icon_name("open-menu-symbolic") // Standard "Hamburger" icon
        .menu_model(&main_menu)
        .build();
    
    let content = Box::new(Orientation::Vertical, 0);
    let head_bar = HeaderBar::builder()
        .build();
    head_bar.pack_end(&menu_btn);
    content.append(&head_bar);
    
    
    let main_dis = Box::builder()
        .valign(gtk4::Align::Center)
        .halign(gtk4::Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    
    let spinner = adw::Spinner::new();
    spinner.set_size_request(128, 128);
    main_dis.append(&spinner);
    
    content.append(&main_dis);
    
    let window = ApplicationWindow::builder()
        .application(main)
        .title("Thorfin")
        .default_width(700)
        .default_height(530)
        .content(&content) 
        .build();
    window.present();
}