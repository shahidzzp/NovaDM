use adw::prelude::*;
use gtk::prelude::*;

use adw::Application;
use gtk::{
    Align,
    Box as GtkBox,
    Button,
    CssProvider,
    Entry,
    Label,
    Orientation,
    ProgressBar,
    ScrolledWindow,
    Separator,
};

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
        .width_request(1000)
        .height_request(650)
        .build();

    // ============================================================
    // ROOT
    // ============================================================

    let root = GtkBox::new(Orientation::Horizontal, 0);
    root.add_css_class("app-root");

    // ============================================================
    // SIDEBAR
    // ============================================================

    let sidebar = GtkBox::new(Orientation::Vertical, 0);
    sidebar.set_width_request(245);
    sidebar.add_css_class("sidebar");

    // ------------------------------------------------------------
    // LOGO
    // ------------------------------------------------------------

    let logo_box = GtkBox::new(Orientation::Horizontal, 12);

    logo_box.set_margin_top(24);
    logo_box.set_margin_start(22);
    logo_box.set_margin_end(22);
    logo_box.set_margin_bottom(24);

    let logo = Label::new(Some("N"));
    logo.add_css_class("logo");

    let brand_box = GtkBox::new(Orientation::Vertical, 0);

    let brand = Label::new(Some("NovaDM"));
    brand.add_css_class("brand-name");
    brand.set_halign(Align::Start);

    let tagline = Label::new(Some("DOWNLOAD MANAGER"));
    tagline.add_css_class("brand-tagline");
    tagline.set_halign(Align::Start);

    brand_box.append(&brand);
    brand_box.append(&tagline);

    logo_box.append(&logo);
    logo_box.append(&brand_box);

    sidebar.append(&logo_box);

    // ------------------------------------------------------------
    // LIBRARY
    // ------------------------------------------------------------

    let nav_title = Label::new(Some("LIBRARY"));
    nav_title.add_css_class("section-label");
    nav_title.set_halign(Align::Start);
    nav_title.set_margin_start(22);
    nav_title.set_margin_bottom(8);

    sidebar.append(&nav_title);

    let overview_button = sidebar_button("⌂", "Overview", true);
    let downloads_button = sidebar_button("↓", "Downloads", false);
    let completed_button = sidebar_button("✓", "Completed", false);

    sidebar.append(&overview_button);
    sidebar.append(&downloads_button);
    sidebar.append(&completed_button);

    // ------------------------------------------------------------
    // SEPARATOR
    // ------------------------------------------------------------

    let separator = Separator::new(Orientation::Horizontal);

    separator.set_margin_top(18);
    separator.set_margin_bottom(18);
    separator.set_margin_start(18);
    separator.set_margin_end(18);

    sidebar.append(&separator);

    // ------------------------------------------------------------
    // TOOLS
    // ------------------------------------------------------------

    let tools_title = Label::new(Some("TOOLS"));
    tools_title.add_css_class("section-label");
    tools_title.set_halign(Align::Start);
    tools_title.set_margin_start(22);
    tools_title.set_margin_bottom(8);

    sidebar.append(&tools_title);

    let queue_button = sidebar_button("≡", "Queue", false);
    let settings_button = sidebar_button("⚙", "Settings", false);

    sidebar.append(&queue_button);
    sidebar.append(&settings_button);

    // ------------------------------------------------------------
    // SPACER
    // ------------------------------------------------------------

    let sidebar_spacer = GtkBox::new(Orientation::Vertical, 0);
    sidebar_spacer.set_vexpand(true);

    sidebar.append(&sidebar_spacer);

    // ------------------------------------------------------------
    // ENGINE STATUS
    // ------------------------------------------------------------

    let engine_box = GtkBox::new(Orientation::Vertical, 5);

    engine_box.add_css_class("engine-box");

    engine_box.set_margin_start(18);
    engine_box.set_margin_end(18);
    engine_box.set_margin_bottom(18);

    let engine_title = Label::new(Some("NOVADM ENGINE"));
    engine_title.add_css_class("engine-title");
    engine_title.set_halign(Align::Start);

    let engine_status = Label::new(Some("●  Ready"));
    engine_status.add_css_class("engine-status");
    engine_status.set_halign(Align::Start);

    engine_box.append(&engine_title);
    engine_box.append(&engine_status);

    sidebar.append(&engine_box);

    root.append(&sidebar);

    // ============================================================
    // MAIN CONTENT
    // ============================================================

    let content = GtkBox::new(Orientation::Vertical, 0);
    content.add_css_class("content");

    // ============================================================
    // HEADER
    // ============================================================

    let header = GtkBox::new(Orientation::Horizontal, 0);

    header.set_margin_start(32);
    header.set_margin_end(32);
    header.set_margin_top(26);
    header.set_margin_bottom(20);

    let title_box = GtkBox::new(Orientation::Vertical, 3);

    let title = Label::new(Some("Overview"));
    title.add_css_class("page-title");
    title.set_halign(Align::Start);

    let subtitle = Label::new(Some(
        "Manage your downloads with NovaDM",
    ));

    subtitle.add_css_class("page-subtitle");
    subtitle.set_halign(Align::Start);

    title_box.append(&title);
    title_box.append(&subtitle);

    header.append(&title_box);

    let header_spacer = GtkBox::new(Orientation::Horizontal, 0);
    header_spacer.set_hexpand(true);

    header.append(&header_spacer);

    let add_button = Button::with_label("＋  Add Download");

    add_button.add_css_class("primary-button");
    add_button.set_height_request(44);

    header.append(&add_button);

    content.append(&header);

    // ============================================================
    // SCROLL AREA
    // ============================================================

    let scrolled = ScrolledWindow::new();

    scrolled.set_vexpand(true);

    scrolled.set_policy(
        gtk::PolicyType::Never,
        gtk::PolicyType::Automatic,
    );

    scrolled.add_css_class("main-scroll");

    let page = GtkBox::new(Orientation::Vertical, 22);

    page.set_margin_start(32);
    page.set_margin_end(32);
    page.set_margin_bottom(32);

    // ============================================================
    // ADD DOWNLOAD CARD
    // ============================================================

    let add_card = GtkBox::new(Orientation::Vertical, 14);
    add_card.add_css_class("hero-card");

    let add_title = Label::new(Some("Add a new download"));
    add_title.add_css_class("card-title");
    add_title.set_halign(Align::Start);

    let add_description = Label::new(Some(
        "Paste a direct file or media URL and NovaDM will handle the rest.",
    ));

    add_description.add_css_class("card-description");
    add_description.set_halign(Align::Start);
    add_description.set_wrap(true);

    let url_row = GtkBox::new(Orientation::Horizontal, 10);

    let url_entry = Entry::new();

    url_entry.set_placeholder_text(Some(
        "https://example.com/file.zip",
    ));

    url_entry.set_hexpand(true);
    url_entry.set_height_request(46);
    url_entry.add_css_class("url-entry");

    let paste_button = Button::with_label("Paste");

    paste_button.add_css_class("secondary-button");
    paste_button.set_height_request(46);

    let download_button = Button::with_label("Download");

    download_button.add_css_class("primary-button");
    download_button.set_height_request(46);

    url_row.append(&url_entry);
    url_row.append(&paste_button);
    url_row.append(&download_button);

    add_card.append(&add_title);
    add_card.append(&add_description);
    add_card.append(&url_row);

    page.append(&add_card);

    // ============================================================
    // STAT CARDS
    // ============================================================

    let stats = GtkBox::new(Orientation::Horizontal, 16);

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

    // ============================================================
    // CURRENT DOWNLOADS HEADER
    // ============================================================

    let current_header = GtkBox::new(Orientation::Horizontal, 0);

    let current_title = Label::new(Some("Current Downloads"));

    current_title.add_css_class("section-title");
    current_title.set_halign(Align::Start);

    current_header.append(&current_title);

    let current_spacer = GtkBox::new(Orientation::Horizontal, 0);
    current_spacer.set_hexpand(true);

    current_header.append(&current_spacer);

    let view_all = Button::with_label("View all");
    view_all.add_css_class("text-button");

    current_header.append(&view_all);

    page.append(&current_header);

    // ============================================================
    // EMPTY STATE
    // ============================================================

    let empty_card = GtkBox::new(Orientation::Vertical, 8);

    empty_card.add_css_class("empty-card");

    let empty_icon = Label::new(Some("↓"));
    empty_icon.add_css_class("empty-icon");
    empty_icon.set_halign(Align::Center);

    let empty_title = Label::new(Some(
        "No active downloads",
    ));

    empty_title.add_css_class("empty-title");
    empty_title.set_halign(Align::Center);

    let empty_description = Label::new(Some(
        "Add a URL above to start downloading.",
    ));

    empty_description.add_css_class("empty-description");
    empty_description.set_halign(Align::Center);

    empty_card.append(&empty_icon);
    empty_card.append(&empty_title);
    empty_card.append(&empty_description);

    page.append(&empty_card);

    // ============================================================
    // DEMO DOWNLOAD CARD
    // ============================================================

    let demo_download = download_card(
        "NovaDM-test-file.zip",
        "https://example.com/NovaDM-test-file.zip",
        67.0,
        "6.42 MB / 9.58 MB",
        "2.91 MB/s",
        "00:01",
    );

    demo_download.set_visible(false);

    page.append(&demo_download);

    // ============================================================
    // COMPLETED SECTION
    // ============================================================

    let completed_header = GtkBox::new(Orientation::Horizontal, 0);

    completed_header.set_margin_top(4);

    let completed_title = Label::new(Some(
        "Recently Completed",
    ));

    completed_title.add_css_class("section-title");
    completed_title.set_halign(Align::Start);

    completed_header.append(&completed_title);

    let completed_spacer = GtkBox::new(Orientation::Horizontal, 0);
    completed_spacer.set_hexpand(true);

    completed_header.append(&completed_spacer);

    let clear_button = Button::with_label("Clear");
    clear_button.add_css_class("text-button");

    completed_header.append(&clear_button);

    page.append(&completed_header);

    let completed_empty = GtkBox::new(Orientation::Horizontal, 0);

    completed_empty.add_css_class("completed-empty");

    let completed_empty_icon = Label::new(Some("✓"));

    completed_empty_icon.add_css_class("completed-icon");

    let completed_empty_text = Label::new(Some(
        "Your completed downloads will appear here.",
    ));

    completed_empty_text.add_css_class("completed-text");
    completed_empty_text.set_halign(Align::Start);

    completed_empty.append(&completed_empty_icon);
    completed_empty.append(&completed_empty_text);

    page.append(&completed_empty);

    // ============================================================
    // FOOTER
    // ============================================================

    let footer = GtkBox::new(Orientation::Horizontal, 0);

    footer.add_css_class("footer");

    let footer_status = Label::new(Some(
        "●  NovaDM Engine Ready",
    ));

    footer_status.add_css_class("footer-status");

    let footer_spacer = GtkBox::new(Orientation::Horizontal, 0);
    footer_spacer.set_hexpand(true);

    let footer_connections = Label::new(Some(
        "Connections: 0 / 4",
    ));

    footer_connections.add_css_class("footer-text");

    let footer_separator = Label::new(Some("  •  "));
    footer_separator.add_css_class("footer-text");

    let footer_version = Label::new(Some("v0.1.0"));
    footer_version.add_css_class("footer-text");

    footer.append(&footer_status);
    footer.append(&footer_spacer);
    footer.append(&footer_connections);
    footer.append(&footer_separator);
    footer.append(&footer_version);

    // ============================================================
    // PASTE BUTTON
    // ============================================================

    {
        let entry = url_entry.clone();

        paste_button.connect_clicked(move |_| {
            if let Some(display) = gtk::gdk::Display::default() {
                let clipboard = display.clipboard();

                let entry_clone = entry.clone();

                glib::MainContext::default().spawn_local(
                    async move {
                        if let Ok(Some(text)) =
                            clipboard.read_text_future().await
                        {
                            entry_clone.set_text(&text);
                        }
                    },
                );
            }
        });
    }

    // ============================================================
    // ADD DOWNLOAD BUTTON
    // ============================================================

    {
        let entry = url_entry.clone();

        add_button.connect_clicked(move |_| {
            entry.grab_focus();
        });
    }

    // ============================================================
    // DOWNLOAD BUTTON
    // ============================================================

    {
        let entry = url_entry.clone();

        download_button.connect_clicked(move |button| {
            let url = entry.text().trim().to_string();

            if url.is_empty() {
                entry.add_css_class("error-entry");
                entry.grab_focus();

                return;
            }

            entry.remove_css_class("error-entry");

            button.set_label("Added ✓");

            let button_clone = button.clone();

            glib::timeout_add_local(
                std::time::Duration::from_millis(1200),
                move || {
                    button_clone.set_label("Download");

                    glib::ControlFlow::Break
                },
            );

            println!(
                "NovaDM GUI received URL: {}",
                url
            );
        });
    }

    // ============================================================
    // CLEAR BUTTON
    // ============================================================

    {
        let completed_empty = completed_empty.clone();

        clear_button.connect_clicked(move |_| {
            completed_empty.set_visible(true);
        });
    }

    // ============================================================
    // FINAL ASSEMBLY
    // ============================================================

    scrolled.set_child(Some(&page));

    content.append(&scrolled);
    content.append(&footer);

    root.append(&content);

    window.set_content(Some(&root));

    window.present();
}

// ================================================================
// SIDEBAR BUTTON
// ================================================================

fn sidebar_button(
    icon: &str,
    text: &str,
    active: bool,
) -> Button {
    let button = Button::new();

    button.set_halign(Align::Fill);
    button.set_hexpand(true);
    button.set_height_request(46);

    button.add_css_class("sidebar-button");

    if active {
        button.add_css_class(
            "sidebar-button-active",
        );
    }

    let row = GtkBox::new(
        Orientation::Horizontal,
        13,
    );

    row.set_margin_start(16);
    row.set_margin_end(16);

    let icon_label = Label::new(Some(icon));

    icon_label.add_css_class(
        "sidebar-icon",
    );

    let text_label = Label::new(Some(text));

    text_label.add_css_class(
        "sidebar-text",
    );

    text_label.set_halign(
        Align::Start,
    );

    row.append(&icon_label);
    row.append(&text_label);

    button.set_child(Some(&row));

    button
}

// ================================================================
// STAT CARD
// ================================================================

fn stat_card(
    title: &str,
    value: &str,
    description: &str,
    icon: &str,
) -> GtkBox {
    let card = GtkBox::new(
        Orientation::Vertical,
        6,
    );

    card.add_css_class("stat-card");

    card.set_hexpand(true);
    card.set_size_request(0, 128);

    let top = GtkBox::new(
        Orientation::Horizontal,
        0,
    );

    let title_label = Label::new(Some(title));

    title_label.add_css_class(
        "stat-title",
    );

    title_label.set_halign(
        Align::Start,
    );

    let spacer = GtkBox::new(
        Orientation::Horizontal,
        0,
    );

    spacer.set_hexpand(true);

    let icon_label = Label::new(Some(icon));

    icon_label.add_css_class(
        "stat-icon",
    );

    top.append(&title_label);
    top.append(&spacer);
    top.append(&icon_label);

    let value_label = Label::new(Some(value));

    value_label.add_css_class(
        "stat-value",
    );

    value_label.set_halign(
        Align::Start,
    );

    let description_label =
        Label::new(Some(description));

    description_label.add_css_class(
        "stat-description",
    );

    description_label.set_halign(
        Align::Start,
    );

    card.append(&top);
    card.append(&value_label);
    card.append(&description_label);

    card
}

// ================================================================
// DOWNLOAD CARD
// ================================================================

fn download_card(
    filename: &str,
    url: &str,
    progress: f64,
    size: &str,
    speed: &str,
    eta: &str,
) -> GtkBox {
    let card = GtkBox::new(
        Orientation::Vertical,
        12,
    );

    card.add_css_class(
        "download-card",
    );

    // ------------------------------------------------------------
    // TOP
    // ------------------------------------------------------------

    let top = GtkBox::new(
        Orientation::Horizontal,
        12,
    );

    let file_icon = Label::new(Some("↓"));

    file_icon.add_css_class(
        "file-icon",
    );

    let info = GtkBox::new(
        Orientation::Vertical,
        3,
    );

    info.set_hexpand(true);

    let name = Label::new(Some(filename));

    name.add_css_class(
        "download-name",
    );

    name.set_halign(
        Align::Start,
    );

    let url_label = Label::new(Some(url));

    url_label.add_css_class(
        "download-url",
    );

    url_label.set_halign(
        Align::Start,
    );

    url_label.set_ellipsize(
        gtk::pango::EllipsizeMode::Middle,
    );

    info.append(&name);
    info.append(&url_label);

    let percentage =
        Label::new(Some(
            &format!("{:.0}%", progress),
        ));

    percentage.add_css_class(
        "download-percentage",
    );

    percentage.set_halign(
        Align::End,
    );

    top.append(&file_icon);
    top.append(&info);
    top.append(&percentage);

    // ------------------------------------------------------------
    // PROGRESS
    // ------------------------------------------------------------

    let progress_bar =
        ProgressBar::new();

    progress_bar.set_fraction(
        progress / 100.0,
    );

    progress_bar.add_css_class(
        "pink-progress",
    );

    // ------------------------------------------------------------
    // BOTTOM
    // ------------------------------------------------------------

    let bottom = GtkBox::new(
        Orientation::Horizontal,
        0,
    );

    let size_label =
        Label::new(Some(size));

    size_label.add_css_class(
        "download-meta",
    );

    let spacer = GtkBox::new(
        Orientation::Horizontal,
        0,
    );

    spacer.set_hexpand(true);

    let speed_label =
        Label::new(Some(speed));

    speed_label.add_css_class(
        "download-speed",
    );

    let eta_label =
        Label::new(Some(
            &format!("ETA {}", eta),
        ));

    eta_label.add_css_class(
        "download-meta",
    );

    eta_label.set_margin_start(15);

    let pause =
        Button::with_label("Pause");

    pause.add_css_class(
        "small-button",
    );

    pause.set_margin_start(15);

    let cancel =
        Button::with_label("×");

    cancel.add_css_class(
        "danger-button",
    );

    cancel.set_margin_start(7);

    bottom.append(&size_label);
    bottom.append(&spacer);
    bottom.append(&speed_label);
    bottom.append(&eta_label);
    bottom.append(&pause);
    bottom.append(&cancel);

    card.append(&top);
    card.append(&progress_bar);
    card.append(&bottom);

    card
}

// ================================================================
// CSS
// ================================================================

fn load_css() {
    let provider =
        CssProvider::new();

    provider.load_from_data(
        r#"
        * {
            font-family: "Inter", "Cantarell", sans-serif;
        }

        window {
            background: #0c0910;
        }

        .app-root {
            background: #0c0910;
        }

        /* ========================================================
           SIDEBAR
           ======================================================== */

        .sidebar {
            background: #110d16;
            border-right: 1px solid #28202d;
        }

        .logo {
            background: #ff3b9d;
            color: #ffffff;
            font-size: 21px;
            font-weight: 900;
            border-radius: 13px;
            min-width: 44px;
            min-height: 44px;
            padding: 8px;
        }

        .brand-name {
            color: #ffffff;
            font-size: 21px;
            font-weight: 800;
        }

        .brand-tagline {
            color: #8d8292;
            font-size: 8px;
            font-weight: 800;
            letter-spacing: 1.4px;
        }

        .section-label {
            color: #766b7b;
            font-size: 10px;
            font-weight: 800;
            letter-spacing: 1.3px;
        }

        .sidebar-button {
            background: transparent;
            color: #aaa0ae;
            border-radius: 10px;
            margin-left: 12px;
            margin-right: 12px;
            border: none;
            box-shadow: none;
        }

        .sidebar-button:hover {
            background: #1c1520;
            color: #ffffff;
        }

        .sidebar-button-active {
            background: #241323;
            color: #ff5eac;
        }

        .sidebar-button-active:hover {
            background: #2b1728;
        }

        .sidebar-icon {
            font-size: 18px;
            min-width: 24px;
        }

        .sidebar-text {
            font-size: 14px;
            font-weight: 600;
        }

        .engine-box {
            background: #17111c;
            border: 1px solid #2b2130;
            border-radius: 12px;
            padding: 13px;
        }

        .engine-title {
            color: #766b7b;
            font-size: 9px;
            font-weight: 800;
            letter-spacing: 1px;
        }

        .engine-status {
            color: #ff55a7;
            font-size: 12px;
            font-weight: 700;
        }

        /* ========================================================
           CONTENT
           ======================================================== */

        .content {
            background: #0c0910;
        }

        .page-title {
            color: #ffffff;
            font-size: 30px;
            font-weight: 800;
        }

        .page-subtitle {
            color: #8d8292;
            font-size: 13px;
        }

        /* ========================================================
           BUTTONS
           ======================================================== */

        .primary-button {
            background: #ff3b9d;
            color: #ffffff;
            border-radius: 11px;
            border: none;
            padding-left: 18px;
            padding-right: 18px;
            font-weight: 750;
            font-size: 13px;
        }

        .primary-button:hover {
            background: #ff55aa;
        }

        .primary-button:active {
            background: #e62d8c;
        }

        .secondary-button {
            background: #211923;
            color: #e9e1eb;
            border: 1px solid #362b39;
            border-radius: 10px;
            padding-left: 17px;
            padding-right: 17px;
            font-weight: 650;
        }

        .secondary-button:hover {
            background: #2b202f;
        }

        .text-button {
            background: transparent;
            color: #ff4fa6;
            border: none;
            font-size: 12px;
            font-weight: 700;
        }

        .text-button:hover {
            color: #ff80bd;
        }

        /* ========================================================
           ADD DOWNLOAD
           ======================================================== */

        .hero-card {
            background: #151019;
            border: 1px solid #2c2230;
            border-radius: 17px;
            padding: 22px;
        }

        .card-title {
            color: #ffffff;
            font-size: 17px;
            font-weight: 750;
        }

        .card-description {
            color: #887d8e;
            font-size: 12px;
        }

        .url-entry {
            background: #0e0a12;
            color: #ffffff;
            border: 1px solid #34283a;
            border-radius: 10px;
            padding-left: 15px;
            padding-right: 15px;
            font-size: 13px;
            box-shadow: none;
        }

        .url-entry:focus {
            border: 1px solid #ff3b9d;
            box-shadow: 0 0 0 1px #ff3b9d;
        }

        .error-entry {
            border: 1px solid #ff4c8b;
        }

        /* ========================================================
           STATS
           ======================================================== */

        .stat-card {
            background: #151019;
            border: 1px solid #2a212f;
            border-radius: 15px;
            padding: 17px;
        }

        .stat-card:hover {
            border-color: #443448;
        }

        .stat-title {
            color: #827787;
            font-size: 9px;
            font-weight: 800;
            letter-spacing: 1px;
        }

        .stat-icon {
            color: #ff4da6;
            font-size: 18px;
            font-weight: 800;
        }

        .stat-value {
            color: #ffffff;
            font-size: 24px;
            font-weight: 800;
        }

        .stat-description {
            color: #756b7b;
            font-size: 10px;
        }

        /* ========================================================
           SECTIONS
           ======================================================== */

        .section-title {
            color: #ffffff;
            font-size: 17px;
            font-weight: 750;
        }

        /* ========================================================
           EMPTY STATE
           ======================================================== */

        .empty-card {
            background: #110d15;
            border: 1px dashed #312735;
            border-radius: 15px;
            padding: 38px;
        }

        .empty-icon {
            color: #ff4fa6;
            font-size: 34px;
            font-weight: 800;
        }

        .empty-title {
            color: #ded6e0;
            font-size: 14px;
            font-weight: 700;
        }

        .empty-description {
            color: #706675;
            font-size: 11px;
        }

        /* ========================================================
           DOWNLOAD CARD
           ======================================================== */

        .download-card {
            background: #151019;
            border: 1px solid #2c2231;
            border-radius: 15px;
            padding: 18px;
        }

        .file-icon {
            background: #291525;
            color: #ff50a6;
            border-radius: 11px;
            min-width: 43px;
            min-height: 43px;
            padding: 8px;
            font-size: 22px;
            font-weight: 800;
        }

        .download-name {
            color: #ffffff;
            font-size: 13px;
            font-weight: 700;
        }

        .download-url {
            color: #716776;
            font-size: 10px;
        }

        .download-percentage {
            color: #ff4fa6;
            font-size: 18px;
            font-weight: 800;
        }

        .pink-progress {
            min-height: 7px;
        }

        .pink-progress trough {
            background: #251d29;
            border-radius: 10px;
            min-height: 7px;
        }

        .pink-progress progress {
            background: #ff3b9d;
            border-radius: 10px;
            min-height: 7px;
        }

        .download-meta {
            color: #827787;
            font-size: 10px;
        }

        .download-speed {
            color: #ff5baa;
            font-size: 11px;
            font-weight: 700;
        }

        .small-button {
            background: #211923;
            color: #cfc5d2;
            border: 1px solid #372c3b;
            border-radius: 8px;
            font-size: 10px;
            font-weight: 700;
        }

        .small-button:hover {
            background: #2c2130;
        }

        .danger-button {
            background: #24151f;
            color: #ff719f;
            border: 1px solid #3a2531;
            border-radius: 8px;
            font-size: 13px;
            font-weight: 800;
            min-width: 32px;
        }

        .danger-button:hover {
            background: #351b29;
        }

        /* ========================================================
           COMPLETED
           ======================================================== */

        .completed-empty {
            background: #110d15;
            border: 1px solid #241d29;
            border-radius: 13px;
            padding: 17px;
        }

        .completed-icon {
            color: #ff4fa6;
            font-size: 17px;
            font-weight: 800;
            margin-right: 11px;
        }

        .completed-text {
            color: #716776;
            font-size: 11px;
        }

        /* ========================================================
           FOOTER
           ======================================================== */

        .footer {
            background: #110d15;
            border-top: 1px solid #27202c;
            padding: 9px 24px;
        }

        .footer-status {
            color: #ff50a6;
            font-size: 10px;
            font-weight: 700;
        }

        .footer-text {
            color: #625967;
            font-size: 10px;
        }

        /* ========================================================
           SCROLLBAR
           ======================================================== */

        scrollbar slider {
            background: #3b2d40;
            border-radius: 10px;
            min-width: 7px;
        }

        scrollbar slider:hover {
            background: #ff3b9d;
        }
        "#,
    );

    if let Some(display) =
        gtk::gdk::Display::default()
    {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
