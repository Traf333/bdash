use crate::application::client;
use log::info;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub play_passes: u64,
    pub total_balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitData {
    pub tgWebAppData: String,
    pub tgWebAppVersion: String,
    pub tgWebAppThemeParams: String,
    pub tgWebAppPlatform: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    Blum,
    Doge,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Option<Thing>,
    #[serde(deserialize_with = "deserialize_name")]
    pub name: String,
    pub init_data: InitData,
    pub status: Option<bool>,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub data: Option<Data>,
}

fn deserialize_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> Visitor<'de> for StringOrNumberVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}

impl Account {
    // pub fn new(name: &str, init_data: &str, refresh_token: &str, access_token: &str) -> Account {
    //     Account {
    //         name: name.to_string(),
    //         init_data: init_data.to_string(),
    //         refresh_token: Some(refresh_token.to_string()),
    //         access_token: Some(access_token.to_string()),
    //         status: None,
    //         data: None,
    //         id: None,
    //     }
    // }

    pub async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(token) = &self.refresh_token {
            info!("Обновляем токен для: {}", self.name);

            let (refresh_token, access_token) = client::refresh_token(token).await?;
            if access_token.is_empty() {
                self.initialise().await?;
            } else {
                self.refresh_token = Some(refresh_token);
                self.access_token = Some(access_token);
            }
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
        info!("Вход каждый день: {}", &self.name);
        client::daily_claim(self).await?;
        Ok(())
    }

    pub async fn eight(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Собираем монетки при входе для {}", &self.name);
        client::eight_hours_claim(self).await?;
        Ok(())
    }

    pub async fn initialise(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Account init: {}", &self.name);
        let (refresh_token, access_token) =
            client::init(self.init_data.tgWebAppData.clone()).await?;
        self.refresh_token = Some(refresh_token);
        self.access_token = Some(access_token);
        Ok(())
    }
}
