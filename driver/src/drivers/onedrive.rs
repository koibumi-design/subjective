use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::Deserialize;
use anyhow::Result;
use async_trait::async_trait;
use crate::{DriverTrait, IndexedLiveAccessToken, LiveAccessToken};

#[derive(Debug, Deserialize, Clone)]
pub struct OnedriveAccountConfig {
    /// The refresh token for the onedrive account.
    /// *For further information, please refer to the official documentation of Microsoft OAuth 2.0 authorization flow.*
    pub refresh_token: String,

    /// The client id for the application.
    /// You can get it from the Azure portal with the client secret.
    pub client_id: String,

    /// The client secret for the application.
    /// You can get it from the Azure portal with the client id.
    pub client_secret: String,
}

struct OnedriveLiveAccessToken {
    pub config: OnedriveAccountConfig,
    pub access_token: Arc<RwLock<String>>,
    pub expires_at: Arc<RwLock<i64>>,
    pub my_drive_id: String,
}

#[async_trait]
impl LiveAccessToken for OnedriveLiveAccessToken {
    async fn get_signed_url(&self, path: &str) -> Result<String> {
        todo!()
    }
}

impl IndexedLiveAccessToken for OnedriveLiveAccessToken {
    fn index_field(&self) -> String {
        self.config.client_id.clone()
    }
}

impl OnedriveLiveAccessToken {
    pub async fn from_config(config: OnedriveAccountConfig) -> Result<Self> {
        let access_token = fetch_access_token(&config).await?.access_token;
        let expires_at = fetch_access_token(&config).await?.expires_in;
        let my_drive_id = get_my_od_id(&access_token).await?;
        Ok(Self {
            config,
            access_token: Arc::new(RwLock::new(access_token)),
            expires_at: Arc::new(RwLock::new(expires_at)),
            my_drive_id,
        })
    }

    pub async fn update_self(&self) -> Result<()> {
        let res = fetch_access_token(&self.config).await?;
        let mut access_token = self.access_token.write().await;
        let mut expires_at = self.expires_at.write().await;
        *access_token = res.access_token;
        *expires_at = res.expires_in;
        Ok(())
    }
}

// auth
const AUTH_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

#[allow(unused_variables)]
#[derive(Debug, Deserialize)]
/// The response json when request `AUTH_URL`.
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: String,
}

async fn fetch_access_token(config: &OnedriveAccountConfig) -> Result<AccessTokenResponse> {
    let res = reqwest::Client::new()
        .post(AUTH_URL)
        .form(&[
            ("client_id", &config.client_id),
            ("refresh_token", &config.refresh_token),
            ("requested_token_use", &"on_behalf_of".to_owned()),
            ("client_secret", &config.client_secret),
            ("grant_type", &"refresh_token".to_owned()),
        ])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;
    Ok(res)
}

// get my drive id
const MY_DRIVE_URL: &str = "https://graph.microsoft.com/v1.0/me/drive";
async fn get_my_od_id(access_token: &str) -> Result<String> {
    #[derive(Debug, Deserialize)]
    /// Response json when request `MY_DRIVE_URL`.
    struct MyDrive {
        id: String,
    }
    let client = reqwest::Client::new();
    let res = client
        .get(MY_DRIVE_URL)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<MyDrive>()
        .await?;
    Ok(res.id)
}

pub struct OnedriveDriver(HashMap<String, Arc<OnedriveLiveAccessToken>>);

const ONEDRIVE_DRIVER_NAME: &str = "onedrive";

