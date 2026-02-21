use libadwaita as adw;
use adw::prelude::*;
use adw::{HeaderBar, ApplicationWindow, ViewSwitcher, ViewStack, Application,glib};
use gtk4::{self as gtk, Label, MenuButton, Orientation, gio};
use gtk::{Box, Button, Align, Stack};
use crate::app_display;
use crate::backend::packagekit::update_list;


pub fn window_init(main: &Application) {
    // Notes to self:
    // Structure Hierarchy:
    // Window
    //  └── content (Vertical Box)
    //       ├── 1. head_bar (HeaderBar)
    //       │      ├── Start: refresh (Button)
    //       │      ├── Title: tabs_switcher (ViewSwitcher) ──[Links to]──┐
    //       │      └── End:   menu_btn (MenuButton)                      │
    //       │                                                            │
    //       └── 2. tabs_switch (ViewStack) <─────────────────────────────┘
    //              ├── Page 1: "explore" (Box: explore_page)
    //              │      └── (Dynamic) spinner (Appended here on click)
    //              ├── Page 2: "installed" (Label: ins_page)
    //              └── Page 3: "updates" (Label: upd_page)
    //the menu button on the right side
    let main_menu = gio::Menu::new();
    main_menu.append(Some("Preferences"), Some("app.preferences"));
    main_menu.append(Some("About"), Some("app.about"));
    
    
    //head bar and container
    let content = Box::new(Orientation::Vertical, 0);
    let head_bar = HeaderBar::builder()
        .build();    
    //buttons
    let menu_btn = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&main_menu)
        .build();

    let tabs_switch = ViewStack::new();
    tabs_switch.set_vexpand(true);

    let explore_page = app_display::build_explore_page();
    
    let spinner_box = Box::new(Orientation::Vertical, 0);
    spinner_box.set_valign(Align::Center);
    spinner_box.set_halign(Align::Center);
        
    let spinner = adw::Spinner::new();
    spinner.set_size_request(64, 64);
    spinner_box.append(&spinner);
    
    let explore_stack = Stack::new();
    explore_stack.add_named(&explore_page, Some("content"));
    explore_stack.add_named(&spinner_box, Some("loading"));
    explore_stack.set_visible_child_name("content");
    
    
    let page1 = tabs_switch.add_titled(&explore_stack, Some("explore"), "Explore");
    page1.set_icon_name(Some("emoji-symbols-symbolic"));    

    let ins_page = Label::new(Some("Installed "));
    let page2 = tabs_switch.add_titled(&ins_page, Some("installed"), "Installed");
    page2.set_icon_name(Some("system-run-symbolic"));    
    
    
    let upd_page = Label::new(Some("Updates"));
    let page3 = tabs_switch.add_titled(&upd_page, Some("updates"), "Updates");
    page3.set_icon_name(Some("applications-engineering-symbolic"));
    
    let tabs_switcher = ViewSwitcher::builder()
        .stack(&tabs_switch)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build();
    
    let refresh = Button::builder()
        .icon_name("view-refresh-symbolic")
        .build();
    
    
    let stack_clone = explore_stack.clone();
        refresh.connect_clicked(move |btn|{
            btn.set_icon_name("media-playback-stop-symbolic");
            btn.set_sensitive(false);
            stack_clone.set_visible_child_name("loading");
            let stack_clone2 = stack_clone.clone();
            let btn_clone = btn.clone();
            glib::spawn_future_local(async move {
            update_list().await; 
            btn_clone.set_icon_name("view-refresh-symbolic");
            btn_clone.set_sensitive(true);
            stack_clone2.set_visible_child_name("content");
        });
    });
    
    
    //packing of the declared buttons and viewswithcers
    // aka packers
    head_bar.set_title_widget(Some(&tabs_switcher));
    head_bar.pack_end(&menu_btn);
    head_bar.pack_start(&refresh);
    content.append(&head_bar);
    content.append(&tabs_switch);
   
    
    
    //display the window 
    let window = ApplicationWindow::builder()
        .application(main)
        .default_width(700)
        .default_height(530)
        .content(&content) 
        .build();
    window.present();
}