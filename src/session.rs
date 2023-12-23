use crate::db::{DataStore, SessionItem};
use anyhow::{anyhow, Result};
use log::{error, info};

#[derive(Debug, Clone)]
pub struct Session {
    keep_alive: u64,
    db: DataStore,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    /// create a new session object
    pub fn new() -> Session {
        let db = DataStore::create();
        let keep_alive = crate::SESSION_TIMEOUT;

        Session { keep_alive, db }
    }

    /// generate session id code
    pub fn generate_code(&self) -> String {
        let range = 1_000_000_000_000..10_000_000_000_000;
        format!(
            "{:x}{:x}",
            fastrand::u64(range.clone()),
            fastrand::u64(range)
        )
    }

    /// create a user session and return the session code or error
    pub fn create_user_session(&mut self, user: &str) -> Result<String> {
        let code = self.generate_code();
        info!("user: {}, code: {}", user, &code);

        let ss = SessionItem::new(code.as_str(), user, self.keep_alive);
        match self.db.put(ss) {
            Ok(_) => Ok(code),
            Err(e) => {
                let msg = format!("error saving session item: {}", e);
                error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
    }

    /// return true if the session is still valid
    pub fn is_valid(&self, code: &str, user: &str) -> bool {
        let resp = self.db.get(code, user);
        resp.is_some()
    }

    /// remove the user session
    pub fn remove(&mut self, code: &str, user: &str) -> Option<String> {
        info!("remove user session: {}:{}", code, user);
        if self.db.remove(code, user) {
            Some(code.to_string())
        } else {
            None
        }
    }

    /// return the number of sessions currently in the database
    pub fn dbsize(&self) -> usize {
        self.db.dbsize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_session() -> Session {
        Session::new()
    }

    #[test]
    fn create_user_session() {
        let mut session = create_session();
        assert_eq!(session.dbsize(), 0);
        let user = "sally";
        let resp = session.create_user_session(user);
        assert!(resp.is_ok());
        let code = resp.unwrap();
        assert!(code.len() > 20);
        assert_eq!(session.dbsize(), 1);

        assert!(session.is_valid(&code, user));
    }

    #[test]
    fn remove_user_session() {
        let mut session = create_session();
        let user = "sally";
        let resp = session.create_user_session(user);
        assert!(resp.is_ok());
        let code = resp.unwrap();
        assert!(code.len() > 20);

        assert!(session.is_valid(&code, user));

        let resp = session.remove(&code, user);
        assert!(resp.is_some());
        assert_eq!(resp.unwrap(), code);

        assert!(!session.is_valid(&code, user));
        let resp = session.remove(&code, user);
        assert!(resp.is_none());
    }

    #[test]
    fn generate_code() {
        let session = create_session();
        let code = session.generate_code();
        println!("{}", code);
        assert!(code.len() == 22);
    }

    #[test]
    fn create() {
        let session = create_session();
        assert_eq!(session.db.dbsize(), 0);
    }
}
