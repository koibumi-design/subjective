use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedLinkCache {
    pub link: String,
    pub created_at: chrono::NaiveDateTime,
    pub lock_until: Option<chrono::NaiveDateTime>,
}

impl SignedLinkCache {
    pub fn new(link: String) -> Self {
        Self {
            link,
            created_at: chrono::Utc::now().naive_utc(),
            lock_until: None,
        }
    }
    pub fn lock(mut self, ms: u64) -> Self {
        let now = chrono::Utc::now().naive_utc();
        let lock_until = now + chrono::Duration::milliseconds(ms as i64);
        self.lock_until = Some(lock_until);
        self
    }
    pub fn unlock(mut self) -> Self {
        self.lock_until = None;
        self
    }
}