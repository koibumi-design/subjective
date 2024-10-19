mod onedrive;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::cache::FileDownloadSource;
use anyhow::Result;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum DriversConfig {
    Onedrive(onedrive::OnedriveConfig),
}

#[async_trait]
pub trait AccessToken: Sized + Send + Sync + Serialize + for<'de> Deserialize<'de> {
    async fn get_signed_url(&self, source: &FileDownloadSource) -> Result<String>;
}

#[async_trait]
pub trait ProviderAccount<A: AccessToken>: Sized + Send + Sync + for<'de> Deserialize<'de> {
    async fn get_access_token(&self) -> Result<A>;
}

#[async_trait]
pub trait DynamicAccessTokenAccount<A: AccessToken>: ProviderAccount<A> {
    /// how long the token will be valid in seconds.
    fn general_lifetime() -> u64;
}