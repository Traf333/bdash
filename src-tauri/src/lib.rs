use domain::account::Account;
use infrastructure::db::connect_db;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
pub mod application;
pub mod domain;
pub mod infrastructure;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn accounts() -> surrealdb::Result<Vec<Account>> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;
    dbg!("creating new record");
    // Create a new person with a random id
    // Create a new person with a random id
    let created: Vec<Account> = db
        .create("account")
        .content(Account::new(
            "bob",
            "init data",
            "some atoken",
            "another token",
        ))
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    let updated: Option<Account> = db
        .update(("account", "jaime"))
        .merge(Account::new("jameee", "iiiiinit", "some", "token"))
        .await?;
    dbg!(updated);

    // Select all people records
    let people: Vec<Account> = db.select("account").await?;
    dbg!(&people);

    Ok(people)
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::async_runtime::spawn(async move {
        connect_db().await.unwrap();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, accounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
