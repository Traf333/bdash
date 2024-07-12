use crate::{domain::account::Account, infrastructure::db::DB};
use surrealdb::{error::Db::Thrown, Error};

pub struct AccountRepository {
    table: String,
}

impl AccountRepository {
    pub fn new() -> Self {
        AccountRepository {
            table: String::from("account"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Account>, Error> {
        let records = DB.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Account, Error> {
        if let Some(record) = DB.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        let error = Error::Db(Thrown(format!("Account with id {} not found", id)));
        Err(error)
    }

    pub async fn get_by_name(&self, name: String) -> Result<Account, Error> {
        if let Some(record) = DB
            .query("SELECT * FROM account WHERE name = $name")
            .bind(("name", name.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        let error = Error::Db(Thrown(format!("Account with name {} not found", name)));
        Err(error)
    }

    pub async fn create_account(&self, content: Account) -> Result<Vec<Account>, Error> {
        let record = DB.create(&self.table).content(content).await?;
        Ok(record)
    }

    pub async fn update_account(&self, id: String, content: Account) -> Result<Account, Error> {
        let record = DB
            .update((&self.table, id))
            .content(content)
            .await?
            .unwrap();
        Ok(record)
    }

    pub async fn delete_account(&self, id: String) -> Result<Account, Error> {
        let result = DB.delete((&self.table, id)).await?.unwrap();
        Ok(result)
    }
}
