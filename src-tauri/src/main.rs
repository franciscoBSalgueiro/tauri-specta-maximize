// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};

#[tauri::command]
#[specta::specta]
async fn close_splashscreen(window: Window) -> Result<(), String> {
    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .maximize()
        .unwrap();
    Ok(())
}

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands!(close_splashscreen,));

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.ts");
        specta_builder.into_plugin()
    };

    tauri::Builder::default()
        .plugin(specta_builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
