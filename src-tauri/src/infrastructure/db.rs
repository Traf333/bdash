use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    Result, Surreal,
};

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db() -> Result<()> {
    let _ = DB.connect::<Wss>("bdash-db.fly.dev").await?;
    let _ = DB
        .signin(Root {
            username: "bobr",
            password: "bobrrulesforever",
        })
        .await;
    let _ = DB.use_ns("bdash").use_db("bdash").await?;
    Ok(())
}
