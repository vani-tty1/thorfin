use libadwaita as adw;
use adw::prelude::*;
use adw::{HeaderBar, ApplicationWindow, ViewSwitcher, ViewStack, Application,glib};
use gtk4::{self as gtk, Label, MenuButton, Orientation, gio};
use gtk::{Box, Button};
use crate::app_display;


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
    
    let page1 = tabs_switch.add_titled(&explore_page, Some("explore"), "Explore");
    page1.set_icon_name(Some("system-search-symbolic"));    

    let ins_page = Label::new(Some("Installed "));
    let page2 = tabs_switch.add_titled(&ins_page, Some("installed"), "Installed");
    page2.set_icon_name(Some("preferences-other-symbolic"));    
    
    
    let upd_page = Label::new(Some("Updates"));
    let page3 = tabs_switch.add_titled(&upd_page, Some("updates"), "Updates");
    page3.set_icon_name(Some("preferences-system-symbolic"));
    
    let tabs_switcher = ViewSwitcher::builder()
        .stack(&tabs_switch)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build();
    
    let refresh = Button::builder()
        .icon_name("view-refresh-symbolic")
        .build();
    
    
    
    let explore_page_clone = explore_page.clone();
    refresh.connect_clicked(move |btn|{
        btn.set_icon_name("media-playback-stop-symbolic");
        btn.set_sensitive(false);
        
        let spinner = adw::Spinner::new();
        spinner.set_size_request(128, 128);
        
        let spinner_clone = spinner.clone();
        let btn_clone = btn.clone();
        
        glib::timeout_add_seconds_local(3, move ||  {
            spinner_clone.unparent();
            btn_clone.set_icon_name("view-refresh-symbolic");
            btn_clone.set_sensitive(true);
            glib::ControlFlow::Break
        });
        
        
        explore_page_clone.append(&spinner);
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