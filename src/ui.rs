use gtk::prelude::*;
use gtk::{
    ApplicationWindow,
    Box as GtkBox,
    Button,
    Label,
    Orientation,
    ProgressBar,
    ScrolledWindow,
    Separator,
};

use adw::prelude::*;
use adw::Application;

const APP_ID: &str = "com.novadm.NovaDM";

pub fn run() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    load_css();

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("NovaDM")
        .default_width(1280)
        .default_height(820)
        .resizable(true)
        .build();

    let main_box = GtkBox::new(Orientation::Horizontal, 0);
    main_box.add_css_class("main-window");

    let sidebar = build_sidebar();
    main_box.append(&sidebar);

    let content = build_content();
    main_box.append(&content);

    window.set_content(Some(&main_box));
    window.present();
}

fn build_sidebar() -> GtkBox {
    let sidebar = GtkBox::new(Orientation::Vertical, 0);
    sidebar.set_width_request(250);
    sidebar.add_css_class("sidebar");

    // Logo
    let logo_area = GtkBox::new(Orientation::Horizontal, 12);
    logo_area.set_margin_top(20);
    logo_area.set_margin_start(18);
    logo_area.set_margin_end(18);
    logo_area.set_margin_bottom(28);

    let logo = Label::new(Some("N"));
    logo.add_css_class("logo");

    let brand_box = GtkBox::new(Orientation::Vertical, 0);

    let brand = Label::new(Some("NovaDM"));
    brand.add_css_class("brand");

    let subtitle = Label::new(Some("DOWNLOAD MANAGER"));
    subtitle.add_css_class("brand-subtitle");

    brand_box.append(&brand);
    brand_box.append(&subtitle);

    logo_area.append(&logo);
    logo_area.append(&brand_box);

    sidebar.append(&logo_area);

    // Library title
    let library_title = Label::new(Some("LIBRARY"));
    library_title.set_xalign(0.0);
    library_title.set_margin_start(18);
    library_title.set_margin_bottom(8);
    library_title.add_css_class("section-title");

    sidebar.append(&library_title);

    // Navigation
    let overview = nav_button("⌂", "Overview", true);
    let downloads = nav_button("↓", "Downloads", false);
    let completed = nav_button("✓", "Completed", false);

    sidebar.append(&overview);
    sidebar.append(&downloads);
    sidebar.append(&completed);

    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(18);
    separator.set_margin_bottom(18);
    separator.set_margin_start(14);
    separator.set_margin_end(14);

    sidebar.append(&separator);

    let tools_title = Label::new(Some("TOOLS"));
    tools_title.set_xalign(0.0);
    tools_title.set_margin_start(18);
    tools_title.set_margin_bottom(8);
    tools_title.add_css_class("section-title");

    sidebar.append(&tools_title);

    let queue = nav_button("≡", "Queue", false);
    let settings = nav_button("⚙", "Settings", false);

    sidebar.append(&queue);
    sidebar.append(&settings);

    // Engine status at bottom
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    sidebar.append(&spacer);

    let engine = GtkBox::new(Orientation::Vertical, 4);
    engine.set_margin_start(14);
    engine.set_margin_end(14);
    engine.set_margin_bottom(18);
    engine.set_margin_top(10);
    engine.set_margin_end(18);
    engine.add_css_class("engine-card");

    let engine_title = Label::new(Some("NOVADM ENGINE"));
    engine_title.set_xalign(0.0);
    engine_title.add_css_class("engine-title");

    let engine_status = Label::new(Some("● Ready"));
    engine_status.set_xalign(0.0);
    engine_status.add_css_class("engine-status");

    engine.append(&engine_title);
    engine.append(&engine_status);

    sidebar.append(&engine);

    sidebar
}

fn nav_button(icon: &str, text: &str, active: bool) -> Button {
    let button = Button::new();

    button.set_hexpand(true);
    button.set_halign(gtk::Align::Fill);

    let row = GtkBox::new(Orientation::Horizontal, 14);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("nav-icon");

    let text_label = Label::new(Some(text));
    text_label.set_xalign(0.0);
    text_label.add_css_class("nav-text");

    row.append(&icon_label);
    row.append(&text_label);

    button.set_child(Some(&row));
    button.add_css_class("nav-button");

    if active {
        button.add_css_class("nav-active");
    }

    button
}

fn build_content() -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.add_css_class("content");

    let scroll = ScrolledWindow::new();
    scroll.set_hexpand(true);
    scroll.set_vexpand(true);
    scroll.set_policy(
        gtk::PolicyType::Never,
        gtk::PolicyType::Automatic,
    );

    let page = GtkBox::new(Orientation::Vertical, 0);
    page.set_margin_top(24);
    page.set_margin_start(32);
    page.set_margin_end(32);
    page.set_margin_bottom(30);

    // Header
    let header = GtkBox::new(Orientation::Horizontal, 0);

    let heading_box = GtkBox::new(Orientation::Vertical, 3);
    heading_box.set_hexpand(true);

    let heading = Label::new(Some("Overview"));
    heading.set_xalign(0.0);
    heading.add_css_class("page-title");

    let description =
        Label::new(Some("Manage your downloads with NovaDM"));
    description.set_xalign(0.0);
    description.add_css_class("page-description");

    heading_box.append(&heading);
    heading_box.append(&description);

    let add_button = Button::with_label("+  Add Download");
    add_button.add_css_class("pink-button");

    header.append(&heading_box);
    header.append(&add_button);

    page.append(&header);

    // Add download card
    let add_card = GtkBox::new(Orientation::Vertical, 12);
    add_card.set_margin_top(28);
    add_card.set_margin_bottom(22);
    add_card.set_margin_start(0);
    add_card.set_margin_end(0);
    add_card.add_css_class("card");

    let add_title = Label::new(Some("Add a new download"));
    add_title.set_xalign(0.0);
    add_title.add_css_class("card-title");

    let add_description = Label::new(Some(
        "Paste a direct file or media URL and NovaDM will handle the rest.",
    ));
    add_description.set_xalign(0.0);
    add_description.add_css_class("card-description");

    let input_row = GtkBox::new(Orientation::Horizontal, 10);

    let url_entry = gtk::Entry::new();
    url_entry.set_hexpand(true);
    url_entry.set_placeholder_text(Some(
        "https://example.com/file.zip",
    ));
    url_entry.add_css_class("url-entry");

    let paste_button = Button::with_label("Paste");
    paste_button.add_css_class("text-button");

    let download_button = Button::with_label("Download");
    download_button.add_css_class("download-button");

    input_row.append(&url_entry);
    input_row.append(&paste_button);
    input_row.append(&download_button);

    add_card.append(&add_title);
    add_card.append(&add_description);
    add_card.append(&input_row);

    page.append(&add_card);

    // Statistics
    let stats = GtkBox::new(Orientation::Horizontal, 14);
    stats.set_hexpand(true);

    let active_card = stat_card(
        "ACTIVE DOWNLOADS",
        "0",
        "Currently downloading",
        "↓",
    );

    let completed_card = stat_card(
        "COMPLETED",
        "0",
        "Files downloaded",
        "✓",
    );

    let speed_card = stat_card(
        "DOWNLOAD SPEED",
        "0 B/s",
        "Current transfer speed",
        "⚡",
    );

    let connections_card = stat_card(
        "CONNECTIONS",
        "0 / 4",
        "Maximum connections",
        "↔",
    );

    stats.append(&active_card);
    stats.append(&completed_card);
    stats.append(&speed_card);
    stats.append(&connections_card);

    page.append(&stats);

    // Current downloads heading
    let current_header = GtkBox::new(Orientation::Horizontal, 0);
    current_header.set_margin_top(30);
    current_header.set_margin_bottom(12);

    let current_title = Label::new(Some("Current Downloads"));
    current_title.set_xalign(0.0);
    current_title.set_hexpand(true);
    current_title.add_css_class("section-heading");

    let view_all = Button::with_label("View all");
    view_all.add_css_class("link-button");

    current_header.append(&current_title);
    current_header.append(&view_all);

    page.append(&current_header);

    // Empty download state
    let empty = GtkBox::new(Orientation::Vertical, 8);
    empty.set_height_request(165);
    empty.set_valign(gtk::Align::Center);
    empty.set_halign(gtk::Align::Fill);
    empty.add_css_class("empty-card");

    let empty_icon = Label::new(Some("↓"));
    empty_icon.add_css_class("empty-icon");

    let empty_title = Label::new(Some("No active downloads"));
    empty_title.add_css_class("empty-title");

    let empty_description =
        Label::new(Some("Add a URL above to start downloading."));
    empty_description.add_css_class("empty-description");

    empty.append(&empty_icon);
    empty.append(&empty_title);
    empty.append(&empty_description);

    page.append(&empty);

    // Recently completed
    let completed_header = GtkBox::new(Orientation::Horizontal, 0);
    completed_header.set_margin_top(30);
    completed_header.set_margin_bottom(12);

    let recently_title = Label::new(Some("Recently Completed"));
    recently_title.set_xalign(0.0);
    recently_title.set_hexpand(true);
    recently_title.add_css_class("section-heading");

    let clear_button = Button::with_label("Clear");
    clear_button.add_css_class("link-button");

    completed_header.append(&recently_title);
    completed_header.append(&clear_button);

    page.append(&completed_header);

    let completed_empty = GtkBox::new(Orientation::Horizontal, 12);
    completed_empty.set_height_request(62);
    completed_empty.set_margin_start(16);
    completed_empty.set_margin_end(16);

    let check = Label::new(Some("✓"));
    check.add_css_class("completed-icon");

    let completed_text =
        Label::new(Some("Your completed downloads will appear here."));
    completed_text.set_xalign(0.0);
    completed_text.add_css_class("completed-text");

    completed_empty.append(&check);
    completed_empty.append(&completed_text);
    completed_empty.add_css_class("completed-card");

    page.append(&completed_empty);

    // Bottom status
    let status_bar = GtkBox::new(Orientation::Horizontal, 0);
    status_bar.set_margin_top(20);

    let status = Label::new(Some("●  NovaDM Engine Ready"));
    status.set_xalign(0.0);
    status.set_hexpand(true);
    status.add_css_class("status-ready");

    let connections =
        Label::new(Some("Connections: 0 / 4  •  v0.1.0"));
    connections.set_xalign(1.0);
    connections.add_css_class("status-info");

    status_bar.append(&status);
    status_bar.append(&connections);

    page.append(&status_bar);

    // Basic button interactions
    let entry_for_paste = url_entry.clone();

    paste_button.connect_clicked(move |_| {
        if let Some(display) = gtk::gdk::Display::default() {
            let clipboard = display.clipboard();
            let entry = entry_for_paste.clone();

            glib::MainContext::default().spawn_local(async move {
                if let Ok(Some(text)) =
                    clipboard.read_text_future().await
                {
                    entry.set_text(&text);
                }
            });
        }
    });

    let entry_for_download = url_entry.clone();

    download_button.connect_clicked(move |_| {
        let url = entry_for_download.text();

        if !url.is_empty() {
            println!("NovaDM: download requested: {}", url);
        }
    });

    scroll.set_child(Some(&page));
    content.append(&scroll);

    content
}

fn stat_card(
    title: &str,
    value: &str,
    description: &str,
    icon: &str,
) -> GtkBox {
    let card = GtkBox::new(Orientation::Vertical, 6);
    card.set_hexpand(true);
    card.set_height_request(128);
    card.set_margin_top(0);
    card.set_margin_bottom(0);
    card.set_margin_start(0);
    card.set_margin_end(0);
    card.add_css_class("stat-card");

    let top = GtkBox::new(Orientation::Horizontal, 0);

    let title_label = Label::new(Some(title));
    title_label.set_xalign(0.0);
    title_label.set_hexpand(true);
    title_label.add_css_class("stat-title");

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("stat-icon");

    top.append(&title_label);
    top.append(&icon_label);

    let value_label = Label::new(Some(value));
    value_label.set_xalign(0.0);
    value_label.add_css_class("stat-value");

    let desc_label = Label::new(Some(description));
    desc_label.set_xalign(0.0);
    desc_label.add_css_class("stat-description");

    card.append(&top);
    card.append(&value_label);
    card.append(&desc_label);

    card
}

fn load_css() {
    let provider = gtk::CssProvider::new();

    provider.load_from_data(
        r#"
        * {
            font-family: Sans;
        }

        window {
            background: #0c090f;
        }

        .main-window {
            background: #0c090f;
        }

        .sidebar {
            background: #100c14;
            border-right: 1px solid #2a202d;
        }

        .logo {
            background: #ff329b;
            color: white;
            border-radius: 14px;
            padding: 10px 15px;
            font-size: 22px;
            font-weight: 900;
        }

        .brand {
            color: #ffffff;
            font-size: 21px;
            font-weight: 800;
        }

        .brand-subtitle {
            color: #756a79;
            font-size: 8px;
            font-weight: 800;
            letter-spacing: 2px;
        }

        .section-title {
            color: #716675;
            font-size: 10px;
            font-weight: 800;
            letter-spacing: 1.5px;
        }

        .nav-button {
            min-height: 42px;
            margin: 2px 10px;
            padding: 0 14px;
            border-radius: 10px;
            background: transparent;
            border: none;
            color: #b3a9b7;
        }

        .nav-button:hover {
            background: #1b1420;
        }

        .nav-active {
            background: #291529;
            color: #ff3ba0;
        }

        .nav-icon {
            min-width: 20px;
            color: #b5a9b9;
            font-size: 17px;
        }

        .nav-active .nav-icon {
            color: #ff3ba0;
        }

        .nav-text {
            color: inherit;
            font-size: 14px;
            font-weight: 600;
        }

        .engine-card {
            background: #17111b;
            border: 1px solid #302333;
            border-radius: 12px;
            padding: 13px;
        }

        .engine-title {
            color: #766a7b;
            font-size: 9px;
            font-weight: 800;
            letter-spacing: 1.2px;
        }

        .engine-status {
            color: #ff3ba0;
            font-size: 12px;
            font-weight: 700;
        }

        .content {
            background: #0c090f;
        }

        .page-title {
            color: #ffffff;
            font-size: 30px;
            font-weight: 900;
        }

        .page-description {
            color: #786d7d;
            font-size: 13px;
        }

        .pink-button {
            background: #ff329b;
            color: #ffffff;
            border-radius: 9px;
            padding: 10px 18px;
            font-weight: 700;
            border: none;
        }

        .pink-button:hover {
            background: #ff4ba8;
        }

        .card {
            background: #151017;
            border: 1px solid #302433;
            border-radius: 16px;
            padding: 22px;
        }

        .card-title {
            color: #ffffff;
            font-size: 18px;
            font-weight: 800;
        }

        .card-description {
            color: #786d7d;
            font-size: 12px;
        }

        .url-entry {
            min-height: 44px;
            background: #0e0a11;
            border: 1px solid #3b2d40;
            border-radius: 10px;
            color: #ffffff;
            padding: 0 14px;
        }

        .url-entry:focus {
            border-color: #ff329b;
        }

        .text-button {
            color: #ff45a5;
            background: transparent;
            border: none;
            font-weight: 700;
            padding: 10px 15px;
        }

        .download-button {
            background: #ff329b;
            color: white;
            border-radius: 9px;
            border: none;
            padding: 10px 18px;
            font-weight: 800;
        }

        .stat-card {
            background: #151017;
            border: 1px solid #302433;
            border-radius: 15px;
            padding: 17px;
        }

        .stat-title {
            color: #776c7d;
            font-size: 9px;
            font-weight: 800;
            letter-spacing: 1px;
        }

        .stat-icon {
            color: #ff3ba0;
            font-size: 19px;
            font-weight: 900;
        }

        .stat-value {
            color: #ffffff;
            font-size: 24px;
            font-weight: 900;
            margin-top: 5px;
        }

        .stat-description {
            color: #716674;
            font-size: 10px;
        }

        .section-heading {
            color: #ffffff;
            font-size: 17px;
            font-weight: 800;
        }

        .link-button {
            color: #ff3ba0;
            background: transparent;
            border: none;
            font-weight: 700;
        }

        .empty-card {
            background: #0f0b12;
            border: 1px dashed #3a2b3e;
            border-radius: 15px;
        }

        .empty-icon {
            color: #ff3ba0;
            font-size: 27px;
            font-weight: 900;
        }

        .empty-title {
            color: #d5ccd8;
            font-size: 13px;
            font-weight: 700;
        }

        .empty-description {
            color: #716675;
            font-size: 11px;
        }

        .completed-card {
            background: #110d14;
            border: 1px solid #2b222f;
            border-radius: 13px;
            padding: 12px;
        }

        .completed-icon {
            color: #ff3ba0;
            font-size: 17px;
            font-weight: 900;
        }

        .completed-text {
            color: #756a79;
            font-size: 11px;
        }

        .status-ready {
            color: #ff3ba0;
            font-size: 10px;
            font-weight: 700;
        }

        .status-info {
            color: #625967;
            font-size: 10px;
        }

        scrollbar slider {
            background: #3a2d40;
            border-radius: 8px;
        }

        scrollbar slider:hover {
            background: #ff329b;
        }
        "#,
    );

    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
