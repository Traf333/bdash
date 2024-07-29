use crate::application::client;
use log::info;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub play_passes: u64,
    pub total_balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    Blum,
    Doge,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Option<Thing>,
    pub name: String,
    pub init_data: String,
    pub status: bool,
    pub account_type: AccountType,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub data: Option<Data>,
}

impl Account {
    pub fn new(
        name: &str,
        init_data: &str,
        account_type: AccountType,
        refresh_token: &str,
        access_token: &str,
    ) -> Account {
        Account {
            name: name.to_string(),
            init_data: init_data.to_string(),
            account_type,
            refresh_token: Some(refresh_token.to_string()),
            access_token: Some(access_token.to_string()),
            status: false,
            data: None,
            id: None,
        }
    }

    pub async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        dbg!(&self.name);
        dbg!(&self.access_token);
        if let Some(token) = &self.refresh_token {
            info!("Обновляем токен для: {}", self.name);
            let (refresh_token, access_token) = client::refresh_token(token).await?;
            self.refresh_token = Some(refresh_token);
            self.access_token = Some(access_token);
        } else {
            info!("Создаем новый токен для: {}", self.name);
            self.initialise().await?;
        }
        info!("Новый токен: {:?}", &self.access_token);

        Ok(())
    }

    pub async fn balance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        client::balance(self).await?;
        if let Some(data) = &self.data {
            info!(
                "Текущий баланс: {}, Количество игр: {}",
                data.total_balance, data.play_passes
            );
        } else {
            info!("No data has been retrieved");
        }

        Ok(())
    }

    pub async fn daily(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Вход каждый день: {}", self.name);

        client::daily_claim(self).await?;
        Ok(())
    }

    pub async fn eight(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Собираем монетки при входе для {}", &self.name);
        client::eight_hours_claim(self).await?;
        Ok(())
    }

    pub async fn initialise(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Account init: {}", self.name);
        let data: serde_json::Value = serde_json::from_str(&self.init_data)?;
        if let Some(query) = data.get("tgWebAppData") {
            dbg!(query);
            let (refresh_token, access_token) = client::init(query).await?;
            self.refresh_token = Some(refresh_token);
            self.access_token = Some(access_token);
        } else {
            let message = format!("can not obtain data {}", self.init_data);
            panic!("{}", message);
        }
        Ok(())
    }
}
