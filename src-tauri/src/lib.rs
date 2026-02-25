// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use tauri::{AppHandle, Manager};
use std::sync::OnceLock;
type ConvertFn = unsafe extern "C" fn(
    path: *const c_char,
    width: *mut c_int,
    height: *mut c_int,
    output: *const c_char,
    quality: *mut c_int,
) -> *mut c_char;

static CORE_LIB: OnceLock<Library> = OnceLock::new();

fn get_convert_fn(handle: &AppHandle) -> Result<Symbol<'static, ConvertFn>, String> {
    let lib = CORE_LIB.get_or_init(|| {
        let lib_name = if cfg!(target_os = "windows") { "core.dll" } 
                       else if cfg!(target_os = "macos") { "core.dylib" } 
                       else { "core.so" };
        
        let resource_path = handle.path()
            .resolve(format!("libs/{}", lib_name), tauri::path::BaseDirectory::Resource)
            .expect("Failed to resolve lib path");

        unsafe { Library::new(resource_path).expect("Failed to load library") }
    });

    unsafe {
        // 这里 lib 是 &'static Library，所以 get 出来的 Symbol 也是 'static
        let func: Symbol<ConvertFn> = lib.get(b"Convert")
            .map_err(|e| format!("Symbol error: {}", e))?;
        Ok(func)
    }
}

#[tauri::command]
async fn convert(
    handle: tauri::AppHandle,
    path: String,
    mut width: i32,
    mut height: i32,
    output: String,
    mut quality: i32,
) -> Result<String, String> {
    // 获取已经加载好的单例库中的函数
    let convert_sym = get_convert_fn(&handle)?;

    unsafe {
        let c_path = CString::new(path).map_err(|_| "Path error")?;
        let c_output = CString::new(output).map_err(|_| "Output error")?;

        let result_ptr = convert_sym(
            c_path.as_ptr(),
            &mut width as *mut c_int,
            &mut height as *mut c_int,
            c_output.as_ptr(),
            &mut quality as *mut c_int,
        );

        if result_ptr.is_null() {
            return Err("NULL pointer from library".into());
        }

        Ok(CStr::from_ptr(result_ptr).to_string_lossy().into_owned())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(target_os = "macos")]
            {
                use tauri::menu::{Menu, PredefinedMenuItem, Submenu};

                let handle = _app.handle();
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
                _app.set_menu(menu)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
