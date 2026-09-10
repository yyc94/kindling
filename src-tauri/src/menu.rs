//! Application Menu Setup
//!
//! Creates the native application menu with File menu items for:
//! - Import (Plottr, Markdown)
//! - Import (Longform)
//! - Export
//! - Close Project
//! - Project Settings
//! - Kindling Settings

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Emitter, Manager, Wry,
};

/// Menu item IDs for event handling
pub mod menu_ids {
    pub const NEW_PROJECT: &str = "new_project";
    pub const IMPORT_PLOTTR: &str = "import_plottr";
    pub const IMPORT_YWRITER: &str = "import_ywriter";
    pub const IMPORT_MARKDOWN: &str = "import_markdown";
    pub const IMPORT_LONGFORM: &str = "import_longform";
    pub const IMPORT_NOVELWRITER: &str = "import_novelwriter";
    pub const IMPORT_SCRIVENER: &str = "import_scrivener";
    pub const EXPORT: &str = "export";
    pub const CLOSE_PROJECT: &str = "close_project";
    pub const PROJECT_SETTINGS: &str = "project_settings";
    pub const KINDLING_SETTINGS: &str = "kindling_settings";
    pub const QUICK_START: &str = "quick_start";
    pub const TOGGLE_SIDEBAR: &str = "toggle_sidebar";
    pub const TOGGLE_REFERENCES: &str = "toggle_references";
    pub const SYNC: &str = "sync";
    pub const COMMAND_PALETTE: &str = "command_palette";
    pub const ABOUT: &str = "about";
    pub const SEND_FEEDBACK: &str = "send_feedback";
    pub const QUIT: &str = "quit";
}

fn label<'a>(zh: bool, english: &'a str, chinese: &'a str) -> &'a str {
    if zh {
        chinese
    } else {
        english
    }
}

/// Create the application menu
pub fn create_menu(app: &AppHandle<Wry>, locale: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zh = locale.eq_ignore_ascii_case("zh-cn") || locale.to_ascii_lowercase().starts_with("zh-");

    // Import submenu
    let import_plottr = MenuItemBuilder::new("Plottr (.pltr)")
        .id(menu_ids::IMPORT_PLOTTR)
        .accelerator("CmdOrCtrl+Shift+O")
        .build(app)?;

    let import_ywriter = MenuItemBuilder::new("yWriter 7 (.yw7)")
        .id(menu_ids::IMPORT_YWRITER)
        .accelerator("CmdOrCtrl+Shift+Y")
        .build(app)?;

    let import_markdown = MenuItemBuilder::new("Markdown (.md)")
        .id(menu_ids::IMPORT_MARKDOWN)
        .accelerator("CmdOrCtrl+Shift+M")
        .build(app)?;

    let import_longform = MenuItemBuilder::new(label(
        zh,
        "Longform (Index or Vault...)",
        "Longform（索引或仓库...）",
    ))
    .id(menu_ids::IMPORT_LONGFORM)
    .accelerator("CmdOrCtrl+Shift+L")
    .build(app)?;

    let import_scrivener = MenuItemBuilder::new("Scrivener 3 (.scriv)")
        .id(menu_ids::IMPORT_SCRIVENER)
        .accelerator("CmdOrCtrl+Shift+I")
        .build(app)?;

    let import_novelwriter = MenuItemBuilder::new(label(
        zh,
        "novelWriter (Project Folder)",
        "novelWriter（项目文件夹）",
    ))
    .id(menu_ids::IMPORT_NOVELWRITER)
    .build(app)?;

    let import_submenu = SubmenuBuilder::new(app, label(zh, "Import", "导入"))
        .item(&import_plottr)
        .item(&import_ywriter)
        .item(&import_markdown)
        .item(&import_longform)
        .item(&import_scrivener)
        .item(&import_novelwriter)
        .build()?;

    // Export menu item
    let export = MenuItemBuilder::new(label(zh, "Export...", "导出..."))
        .id(menu_ids::EXPORT)
        .accelerator("CmdOrCtrl+E")
        .build(app)?;

    // Close Project menu item
    let close_project = MenuItemBuilder::new(label(zh, "Close Project", "关闭项目"))
        .id(menu_ids::CLOSE_PROJECT)
        .accelerator("CmdOrCtrl+W")
        .build(app)?;

    // Settings menu items
    let project_settings = MenuItemBuilder::new(label(zh, "Project Settings...", "项目设置..."))
        .id(menu_ids::PROJECT_SETTINGS)
        .accelerator("CmdOrCtrl+Shift+P")
        .build(app)?;

    let kindling_settings =
        MenuItemBuilder::new(label(zh, "Kindling Settings...", "Kindling 设置..."))
            .id(menu_ids::KINDLING_SETTINGS)
            .accelerator("CmdOrCtrl+,")
            .build(app)?;

    let new_project = MenuItemBuilder::new(label(zh, "New Project", "新建项目"))
        .id(menu_ids::NEW_PROJECT)
        .accelerator("CmdOrCtrl+N")
        .build(app)?;

    let quit = MenuItemBuilder::new(label(zh, "Quit Kindling", "退出 Kindling"))
        .id(menu_ids::QUIT)
        .accelerator("CmdOrCtrl+Q")
        .build(app)?;

    // Build File submenu
    let file_submenu = SubmenuBuilder::new(app, label(zh, "File", "文件"))
        .item(&new_project)
        .item(
            &MenuItemBuilder::new(label(
                zh,
                "Open Review or Feedback File…",
                "打开审阅或反馈文件…",
            ))
            .id("editorial_open")
            .accelerator("CmdOrCtrl+O")
            .build(app)?,
        )
        .item(
            &MenuItemBuilder::new(label(zh, "Editorial Review…", "编辑审阅…"))
                .id("editorial_project")
                .build(app)?,
        )
        .separator()
        .items(&[&import_submenu])
        .item(&export)
        .separator()
        .item(&close_project)
        .separator()
        .item(&project_settings)
        .item(&kindling_settings)
        .separator()
        .item(&quit)
        .build()?;

    let find = MenuItemBuilder::new(label(zh, "Find in Scene…", "在场景中查找…"))
        .id("find")
        .accelerator("CmdOrCtrl+F")
        .build(app)?;
    let find_replace = MenuItemBuilder::new(label(zh, "Find and Replace…", "查找和替换…"))
        .id("find_replace")
        .accelerator("CmdOrCtrl+Alt+F")
        .build(app)?;
    let find_project = MenuItemBuilder::new(label(
        zh,
        "Find and Replace in Project…",
        "在项目中查找和替换…",
    ))
    .id("find_project")
    .accelerator("CmdOrCtrl+Shift+F")
    .build(app)?;

    // Build Edit submenu with standard items
    let edit_submenu = SubmenuBuilder::new(app, label(zh, "Edit", "编辑"))
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .separator()
        .items(&[&find, &find_replace, &find_project])
        .build()?;

    // Build Window submenu with standard items
    let window_submenu = SubmenuBuilder::new(app, label(zh, "Window", "窗口"))
        .minimize()
        .maximize()
        .separator()
        .close_window()
        .build()?;

    // View submenu
    let toggle_sidebar = MenuItemBuilder::new(label(zh, "Toggle Sidebar", "显示或隐藏侧边栏"))
        .id(menu_ids::TOGGLE_SIDEBAR)
        .accelerator("CmdOrCtrl+Backslash")
        .build(app)?;

    let toggle_references = MenuItemBuilder::new(label(
        zh,
        "Toggle References Panel",
        "显示或隐藏参考资料面板",
    ))
    .id(menu_ids::TOGGLE_REFERENCES)
    .accelerator("CmdOrCtrl+Shift+R")
    .build(app)?;

    let sync = MenuItemBuilder::new(label(zh, "Sync from Source", "从源文件同步"))
        .id(menu_ids::SYNC)
        .accelerator("CmdOrCtrl+Shift+S")
        .build(app)?;

    let view_submenu = SubmenuBuilder::new(app, label(zh, "View", "视图"))
        .item(&toggle_sidebar)
        .item(&toggle_references)
        .item(&sync)
        .build()?;

    // Help submenu
    let about = MenuItemBuilder::new(label(zh, "About Kindling...", "关于 Kindling..."))
        .id(menu_ids::ABOUT)
        .build(app)?;

    let command_palette = MenuItemBuilder::new(label(zh, "Command Palette...", "命令面板..."))
        .id(menu_ids::COMMAND_PALETTE)
        .accelerator("CmdOrCtrl+K")
        .build(app)?;

    let quick_start = MenuItemBuilder::new(label(zh, "Quick Start", "快速入门"))
        .id(menu_ids::QUICK_START)
        .accelerator("CmdOrCtrl+Shift+H")
        .build(app)?;

    let send_feedback = MenuItemBuilder::new(label(zh, "Send Feedback...", "发送反馈..."))
        .id(menu_ids::SEND_FEEDBACK)
        .build(app)?;

    let help_submenu = SubmenuBuilder::new(app, label(zh, "Help", "帮助"))
        .item(&about)
        .item(&send_feedback)
        .separator()
        .item(&command_palette)
        .item(&quick_start)
        .build()?;

    // Build the full menu
    let menu = MenuBuilder::new(app)
        .items(&[
            &file_submenu,
            &edit_submenu,
            &view_submenu,
            &window_submenu,
            &help_submenu,
        ])
        .build()?;

    app.set_menu(menu)?;

    Ok(())
}

#[tauri::command]
pub fn set_menu_locale(app: AppHandle<Wry>, locale: String) -> Result<(), String> {
    create_menu(&app, &locale).map_err(|error| error.to_string())
}

/// Set up menu event handling
pub fn setup_menu_events(app: &AppHandle<Wry>) {
    let app_handle = app.clone();

    app.on_menu_event(move |_app, event| {
        let id = event.id().0.as_str();

        // Emit event to frontend for handling
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.emit("menu-event", id);
        }
    });
}
