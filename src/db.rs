/// db common to otp and session
use anyhow::Result;
use hashbrown::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct SessionItem {
    pub code: String,
    pub user: String,
    pub expires: u64,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    db: HashMap<String, u64>,
}

impl SessionItem {
    pub fn new(code: &str, user: &str, keep_alive: u64) -> SessionItem {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expires = now.as_secs() + keep_alive;

        SessionItem {
            code: code.to_string(),
            user: user.to_string(),
            expires,
        }
    }

    /// return true if the session has expired
    pub fn has_expired(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        self.expires <= now.as_secs()
    }
}

impl DataStore {
    /// create the data store
    pub fn create() -> DataStore {
        DataStore { db: HashMap::new() }
    }

    // create the db key
    fn create_key(&self, code: &str, user: &str) -> String {
        format!("{}:{}", code, user)
    }

    /// return the number of items in the data store
    pub fn dbsize(&self) -> usize {
        self.db.len()
    }

    /// store this in the database
    pub fn put(&mut self, item: SessionItem) -> Result<()> {
        let key = self.create_key(&item.code, &item.user);
        let _resp = self.db.insert(key, item.expires);

        Ok(())
    }

    /// return the session item if it exists and has not expired
    pub fn get(&self, code: &str, user: &str) -> Option<SessionItem> {
        let key = self.create_key(code, user);
        let value = self.db.get(&key);
        if value.is_none() {
            value?;
        }

        let value = *value.unwrap();
        let item = SessionItem {
            code: code.to_string(),
            user: user.to_string(),
            expires: value,
        };

        if item.has_expired() {
            None
        } else {
            Some(item)
        }
    }

    /// remove the item; return true if it was removed, false if not found
    pub fn remove(&mut self, code: &str, user: &str) -> bool {
        let key = self.create_key(code, user);
        let v = self.db.remove(&key);
        v.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::otp::Otp;

    fn create_otp() -> Otp {
        Otp::new()
    }

    #[test]
    fn create() {
        let store = DataStore::create();
        assert_eq!(store.db.len(), 0);
        assert_eq!(store.dbsize(), 0);
    }

    #[test]
    fn otp_item() {
        let otp = create_otp();
        let code = otp.generate_code();
        let user = "jack";

        let item = SessionItem::new(&code, user, 60u64);

        assert_eq!(item.code.len(), 6);
        assert_eq!(item.user, user);
    }

    #[test]
    fn remove_item() {
        let otp = create_otp();
        let code = otp.generate_code();
        let user = "jack";

        let item = SessionItem::new(&code, user, 60u64);
        let mut store = DataStore::create();
        let resp = store.put(item);
        assert!(resp.is_ok());
        assert_eq!(store.dbsize(), 1);
        let resp = store.remove(&code, user);
        assert!(resp);
        let resp = store.remove(&code, user);
        assert!(!resp);
    }

    #[test]
    fn put_get() {
        let otp = create_otp();
        let code = otp.generate_code();
        let user = "jack";
        let keep_alive = 60u64;

        let item = SessionItem::new(&code, user, keep_alive);
        let mut store = DataStore::create();
        assert_eq!(store.dbsize(), 0);

        let _resp = store.put(item).unwrap();
        assert_eq!(store.dbsize(), 1);

        let copy_item = store.get(&code, user);
        assert!(copy_item.is_some());

        let non_item = store.get(&code, "john");
        assert!(non_item.is_none());

        let code = otp.generate_code();
        let user = "sammy";
        let item = SessionItem::new(&code, user, 0u64);
        let _resp = store.put(item).unwrap();
        assert_eq!(store.dbsize(), 2);

        let non_item = store.get(&code, user);
        assert!(non_item.is_none());
    }

    #[test]
    fn has_expired() {
        let otp = create_otp();
        let code = otp.generate_code();
        let user = "jack";
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expires = now + 60;

        let item = SessionItem {
            code: code.to_string(),
            user: user.to_string(),
            expires,
        };
        assert!(!item.has_expired());

        let item = SessionItem {
            code: code.to_string(),
            user: user.to_string(),
            expires: now - 10,
        };
        assert!(item.has_expired());
    }

    #[test]
    fn create_key() {
        let store = DataStore::create();
        let code = "100000";
        let user = "jack";

        let key = store.create_key(code, user);
        assert_eq!(key, "100000:jack");
    }
}
