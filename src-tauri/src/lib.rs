mod command;
use std::sync::{Arc, Mutex};
mod utils;
use command::model::ServerState;
use tauri::menu::*;

pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(ServerState {
            server_handle: None,
        })))
        .menu(|handle| {
            let menu = Menu::with_items(
                handle,
                &[
                    #[cfg(target_os = "macos")]
                    &Submenu::with_items(
                        handle,
                        "Edit",
                        true,
                        &[
                            &PredefinedMenuItem::undo(handle, None)?,
                            &PredefinedMenuItem::redo(handle, None)?,
                            &PredefinedMenuItem::cut(handle, None)?,
                            &PredefinedMenuItem::copy(handle, None)?,
                            &PredefinedMenuItem::paste(handle, None)?,
                            &PredefinedMenuItem::select_all(handle, None)?,
                        ],
                    )?,
                ],
            );
            menu
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            utils::init::show_window(app);
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            command::cmds::open_window,
            command::cmds::preview_from_config,
            command::cmds::update_build_file,
            command::cmds::update_config_file,
            command::cmds::update_cargo_file,
            command::cmds::update_main_rust,
            command::cmds::rust_lib_window,
            command::cmds::update_custom_js,
            command::cmds::get_custom_js,
            command::cmds::content_to_base64,
            command::cmds::update_config_json,
            command::cmds::rust_main_window,
            command::cmds::open_url,
            command::cmds::open_devtools,
            command::cmds::update_init_rs,
            command::cmds::start_server,
            command::cmds::stop_server,
            command::cmds::support_pp,
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let _ = utils::init::resolve_setup(app).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
