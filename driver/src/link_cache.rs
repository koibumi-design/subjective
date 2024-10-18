use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::meta_data::FileDownloadSource;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedLinkCache {
    pub link: String,
    pub created_at: chrono::NaiveDateTime,
    pub is_locked: bool,
}

impl SignedLinkCache {
    pub fn get_uuid(source: FileDownloadSource) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, serde_json::to_string(&source).unwrap().as_bytes())
    }
}