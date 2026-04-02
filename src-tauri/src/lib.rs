pub mod backend;
pub mod domain;
pub mod schema;

use backend::{ watcher::start, folder_tree::load_children };

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())

        .setup(|_app| {
            let _conn = backend::db::establish_connection();
            println!("Database initialized successfully");
            Ok(())
        })

        .invoke_handler(tauri::generate_handler![start, load_children])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}