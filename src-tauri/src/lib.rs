pub mod application;
pub mod domain;
pub mod infrastructure;
use application::routes::accounts::{
    accounts, create_account, destroy_account, refresh_account, update_account,
};
use infrastructure::db::connect_db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async move {
        connect_db().await.unwrap();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            accounts,
            create_account,
            update_account,
            refresh_account,
            destroy_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
