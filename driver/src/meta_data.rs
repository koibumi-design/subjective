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
    pub provider: String,
    pub path: String,
}