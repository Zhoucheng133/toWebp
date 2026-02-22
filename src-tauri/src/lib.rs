// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                use tauri::menu::{Menu, PredefinedMenuItem, Submenu};

                let handle = app.handle();
                let locale = "zh";
                let (app_menu_name, quit_label, edit_label, copy_label, paste_label, select_all_label, undo_label, redo_label, window_label, minimize_label, fullscreen_label, about_label, hide_label) = 
                if locale == "zh" {
                    ("应用", "退出 toWebp", "编辑", "复制", "粘贴", "全选", "撤销", "重做", "窗口", "最小化", "进入全屏幕", "关于 toWebp", "隐藏 toWebp")
                } else {
                    ("App", "Quit toWebp", "Edit", "Copy", "Paste", "Select All", "Undo", "Redo", "Window", "Minimize", "Fullscreen", "About toWebp", "Hide toWebp")
                };
                
                let app_menu = Submenu::with_items(
                    handle, 
                    app_menu_name, 
                    true, 
                    &[
                        &PredefinedMenuItem::about(handle, Some(about_label), None)?,
                        &PredefinedMenuItem::separator(handle)?,
                        &PredefinedMenuItem::hide(handle, Some(hide_label))?,
                        &PredefinedMenuItem::quit(handle, Some(quit_label))?
                    ]
                )?;

                let edit_menu = Submenu::with_items(
                    handle,
                    edit_label,
                    true,
                    &[
                        &PredefinedMenuItem::undo(handle, Some(undo_label))?,
                        &PredefinedMenuItem::redo(handle, Some(redo_label))?,
                        &PredefinedMenuItem::separator(handle)?,
                        &PredefinedMenuItem::copy(handle, Some(copy_label))?,
                        &PredefinedMenuItem::paste(handle, Some(paste_label))?,
                        &PredefinedMenuItem::select_all(handle, Some(select_all_label))?,
                    ],
                )?;

                let window_menu = Submenu::with_items(
                    handle,
                    window_label,
                    true,
                    &[
                        &PredefinedMenuItem::minimize(handle, Some(minimize_label))?,
                        &PredefinedMenuItem::fullscreen(handle, Some(fullscreen_label))?,
                    ],
                )?;

                let menu = Menu::with_items(handle, &[&app_menu, &edit_menu, &window_menu])?;
                app.set_menu(menu)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
