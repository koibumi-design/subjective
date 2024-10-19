use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetaData {
    pub id: Uuid,
    pub file_name: String,
    pub catalog: String,
    pub size: u64,
    pub mime_type: String,
    pub updated_at: NaiveDateTime,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadSource {
    pub file_id: Uuid,
    pub provider_type: String,
    pub provider_index: String,
    pub object_key: String,
}

impl FileDownloadSource {
    pub fn get_cache_uuid(&self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, serde_json::to_string(&self).unwrap().as_bytes())
    }
}