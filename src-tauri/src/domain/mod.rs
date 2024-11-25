use account::Account;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

pub mod account;
impl TryFrom<Account> for String {
    type Error = Error;
    fn try_from(val: Account) -> Result<String> {
        Ok(val.name)
    }
}
