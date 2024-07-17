use surrealdb::Error;

use crate::{
    domain::account::Account, infrastructure::repositories::account_repository::AccountRepository,
};

pub async fn create(content: Account) -> Result<Vec<Account>, Error> {
    let repository = AccountRepository::new();

    repository.create_account(content).await
}

pub async fn update(id: String, content: Account) -> Result<Account, Error> {
    let repository = AccountRepository::new();

    repository.update_account(id, content).await
}

pub async fn destroy(id: String) -> Result<Account, Error> {
    let repository = AccountRepository::new();

    repository.delete_account(id).await
}
