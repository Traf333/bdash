use crate::{
    domain::account::Account, infrastructure::repositories::account_repository::AccountRepository,
};

// might consider some sort,filter, pagination later
pub async fn select_accounts_query() -> Result<Vec<Account>, surrealdb::Error> {
    let repository = AccountRepository::new();
    // let mut accounts = vec![];
    // if let Ok(result) = repository.get_all().await {
    //     accounts = result;
    // }

    // Ok(accounts)
    repository.get_all().await
}
