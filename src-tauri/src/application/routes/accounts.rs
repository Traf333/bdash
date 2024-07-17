use crate::application::{commands, queries};
use crate::domain::account::Account;

#[tauri::command(async)]
pub async fn accounts() -> surrealdb::Result<Vec<Account>> {
    queries::all_accounts::select_accounts_query().await
}

#[tauri::command(async)]
pub async fn create_account(content: Account) -> surrealdb::Result<Vec<Account>> {
    commands::account::create(content).await
}

#[tauri::command(async)]
pub async fn update_account(id: String, content: Account) -> surrealdb::Result<Account> {
    commands::account::update(id, content).await
}

#[tauri::command(async)]
pub async fn destroy_account(id: String) -> surrealdb::Result<Account> {
    commands::account::destroy(id).await
}
