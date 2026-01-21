use gtk4::prelude::*;
use gtk4::{Box, Orientation, Align, Label, Button, Image, ListBox, SelectionMode, ScrolledWindow};
use crate::backend::install;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppEntry{
    pub name: String,
    pub id: String,
    pub summary: String,
}

fn app_row(app: &AppEntry) -> Box {
    let row_box = Box::new(Orientation::Horizontal, 12);
    row_box.set_margin_top(12);
    row_box.set_margin_bottom(12);
    row_box.set_margin_start(12);
    row_box.set_margin_end(12);
    
    let icon = Image::builder()
        .icon_name("application-x-executable-symbolic")
        .pixel_size(48)
        .build();
    
    let text_box = Box::new(Orientation::Vertical, 4);
    text_box.set_hexpand(true);
    text_box.set_valign(Align::Center);
    
    let title = Label::builder()
        .label(&app.name)
        .halign(Align::Start)
        .build();
    title.add_css_class("heading");
    
    let subtitle = Label::builder()
        .label(&app.summary)
        .halign(Align::Start)
        .ellipsize(gtk4::pango::EllipsizeMode::End)
        .build();
    subtitle.add_css_class("dim-label");
    
    text_box.append(&title);
    text_box.append(&subtitle);
    let install_btn = Button::builder()
        .label("Install")
        .valign(Align::Center)
        .build();
    install_btn.add_css_class("pill");
    
    let app_clone = app.clone();
    install_btn.connect_clicked(move |_| {
        install::install_app(&app_clone)
    });
    
    //packers
    row_box.append(&icon);
    row_box.append(&text_box);
    row_box.append(&install_btn);
    row_box
}


pub fn build_explore_page() -> Box {
    
    let parent_box = Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
    
    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vexpand(true)
        .build();

    let list_box = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();
    list_box.add_css_class("boxed-list");
    list_box.set_margin_top(12);
    list_box.set_margin_bottom(12);
    list_box.set_margin_start(12);
    list_box.set_margin_end(12);

    // test dta source 
    // don't know how to communicate to flathub API yet
    let apps_data = vec![
        AppEntry {
            name: "Firefox".to_string(),
            id: "org.mozilla.firefox".to_string(),
            summary: "Fast, Private & Safe Web Browser".to_string(),
        },
        AppEntry {
            name: "OBS Studio".to_string(),
            id: "com.obsproject.Studio".to_string(),
            summary: "Live streaming and video recording software".to_string(),
        },
        AppEntry {
            name: "GIMP".to_string(),
            id: "org.gimp.GIMP".to_string(),
            summary: "Create images and edit photographs".to_string(),
        },
        AppEntry {
            name: "VLC".to_string(),
            id: "org.videolan.VLC".to_string(),
            summary: "The ultimate media player".to_string(),
        },
    ];

    for app in apps_data {
        let row_widget = app_row(&app);
        list_box.append(&row_widget);
    }

    scroll.set_child(Some(&list_box));
    parent_box.append(&scroll);
    parent_box
}
