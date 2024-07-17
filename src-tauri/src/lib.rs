pub mod application;
pub mod domain;
pub mod infrastructure;
use application::routes::accounts;
use infrastructure::db::connect_db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async move {
        connect_db().await.unwrap();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![accounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
