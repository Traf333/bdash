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

pub async fn obtain_balance(id: String) -> Result<Account, Error> {
    let repository = AccountRepository::new();

    let mut account = repository.get_by_id(id.clone()).await?;
    account.refresh().await.unwrap();
    account.daily().await.unwrap();
    account.balance().await.unwrap();

    repository.update_account(id, account).await
}

pub async fn destroy(id: String) -> Result<Account, Error> {
    let repository = AccountRepository::new();

    repository.delete_account(id).await
}
