/// otp generator
use crate::db::{DataStore, SessionItem};
use anyhow::Result;
use log::info;

#[derive(Debug, Clone)]
pub struct Otp {
    keep_alive: u64,
    db: DataStore,
}

impl Default for Otp {
    fn default() -> Self {
        Self::new()
    }
}

impl Otp {
    /// create a new Otp struct
    pub fn new() -> Otp {
        let db = DataStore::create();
        let keep_alive = crate::OTP_TIMEOUT;

        Otp { keep_alive, db }
    }

    /// generate the 6 digit otp code
    pub fn generate_code(&self) -> String {
        let range = 100_000..1_000_000_u64;
        format!("{}", fastrand::u64(range))
    }

    /// create a new user otp and store it with standard expiration timestamp
    pub fn create_user_otp(&mut self, user: &str) -> Result<String> {
        let code = self.generate_code();
        info!("user: {}, code: {}", user, &code);

        let ss = SessionItem::new(code.as_str(), user, self.keep_alive);
        self.db.put(ss)?;

        Ok(code)
    }

    /// validate this otp for the given user
    pub fn is_valid(&self, code: &str, user: &str) -> bool {
        info!("validate: {}:{}", code, user);
        let resp = self.db.get(code, user);
        resp.is_some()
    }

    /// remove the code for this user
    pub fn remove(&mut self, code: &str, user: &str) -> Option<String> {
        info!("remove otp {}:{}", code, user);
        if self.db.remove(code, user) {
            Some(code.to_string())
        } else {
            None
        }
    }

    /// return the number of otp sessions in the database
    pub fn dbsize(&self) -> usize {
        self.db.dbsize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_otp() -> Otp {
        Otp::new()
    }

    #[test]
    fn create_user_otp() {
        let mut otp = create_otp();
        let user = "sally";
        assert_eq!(otp.dbsize(), 0);
        let resp = otp.create_user_otp(user);
        assert!(resp.is_ok());
        let code = resp.unwrap();
        assert_eq!(code.len(), 6);
        assert_eq!(otp.dbsize(), 1);

        assert!(otp.is_valid(&code, user));
    }

    #[test]
    fn remove_user_otp() {
        let mut otp = create_otp();
        let user = "sally";
        let resp = otp.create_user_otp(user);
        assert!(resp.is_ok());
        let code = resp.unwrap();
        assert_eq!(code.len(), 6);

        assert!(otp.is_valid(&code, user));

        let resp = otp.remove(&code, user);
        assert!(resp.is_some());
        assert_eq!(resp.unwrap(), code);

        assert!(!otp.is_valid(&code, user));
        let resp = otp.remove(&code, user);
        assert!(resp.is_none());
    }

    #[test]
    fn generate_code() {
        let otp = create_otp();
        let code = otp.generate_code();

        assert_eq!(code.len(), 6);
    }

    #[test]
    fn create() {
        let otp = create_otp();
        assert_eq!(otp.db.dbsize(), 0);
    }
}
