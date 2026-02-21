use gtk4::prelude::*;
use gtk4::{Box, Orientation, Align, Label, Button, Image, ListBox, SelectionMode, ScrolledWindow, glib};
use crate::backend::packagekit;
use crate::backend::flathub;

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
        .icon_name(&app.id)
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
        let app_to_install = app_clone.clone();
        glib::spawn_future_local(async move {
                packagekit::install_app(&app_to_install).await;
        });
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

    let list_box_clone = list_box.clone();
    
    glib::spawn_future_local(async move {
        println!("Async block started!");
        
        let dummy = AppEntry {
            id: "system-run-symbolic".to_string(),
            name: "Test Dummy".to_string(),
            summary: "If you see this, GTK dynamic updates work.".to_string(),
        };
        list_box_clone.append(&app_row(&dummy));
    
        let mut all_apps = flathub::fetch_popular().await;
        let repo_apps = packagekit::search_repo("browser").await; 
        
        println!("Flathub fetched: {} apps", all_apps.len());
        println!("Repo fetched: {} apps", repo_apps.len());
    
        all_apps.extend(repo_apps); 
    
        for app in all_apps {
            list_box_clone.append(&app_row(&app));
        }
    });
    
    scroll.set_child(Some(&list_box));
    parent_box.append(&scroll);
    parent_box
}