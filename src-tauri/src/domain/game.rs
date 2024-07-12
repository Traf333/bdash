use crate::domain::account::Account;
use csv::{ReaderBuilder, WriterBuilder};
use log::info;
pub struct Game {
    pub accounts: Vec<Account>,
    file_path: String,
}

impl Game {
    pub fn new(file_path: &str) -> Game {
        Game {
            accounts: vec![],
            file_path: file_path.to_string(),
        }
    }

    pub fn read_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&self.file_path)?;
        for result in reader.records() {
            let record = result?;
            let account = Account::new(&record[0], &record[1], &record[2], &record[3]);
            self.accounts.push(account);
        }
        Ok(())
    }
    pub async fn refresh_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for account in &mut self.accounts {
            account.refresh().await?;
        }

        Ok(())
    }

    pub fn save_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_path(&self.file_path)?;
        if let Some(_first_account) = self.accounts.first() {
            wtr.write_record(["name", "init_data", "refresh_token", "access_token"])?;
        }
        for account in &self.accounts {
            wtr.write_record([
                &account.name,
                &account.init_data,
                &account.refresh_token,
                &account.access_token,
            ])?;
        }
        Ok(())
    }

    pub async fn daily(&self) -> Result<(), Box<dyn std::error::Error>> {
        for account in &self.accounts {
            account.daily().await?;
        }

        Ok(())
    }

    pub async fn eight_hours(&self) -> Result<(), Box<dyn std::error::Error>> {
        for account in &self.accounts {
            account.eight().await?;
        }

        Ok(())
    }

    pub async fn fetch_balance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for account in &mut self.accounts {
            account.balance().await?;
        }
        Ok(())
    }

    pub fn summary(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut total_balance = 0.0;
        let mut total_passes = 0;
        for account in &self.accounts {
            if let Some(ref data) = account.data {
                total_balance += data.total_balance;
                total_passes += data.play_passes;
            }
        }

        info!(
            "Total balance: {} \n Total Passes: {}",
            total_balance, total_passes
        );
        Ok(())
    }
}
