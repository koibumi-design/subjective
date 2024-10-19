use serde::{Deserialize, Serialize};
use anyhow::Result;
use async_trait::async_trait;
use crate::cache::FileDownloadSource;
use crate::drivers::{AccessToken, DynamicAccessTokenAccount, ProviderAccount};

#[derive(Debug, Deserialize, Clone)]
pub struct OnedriveConfig {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OnedriveLiveAccessToken {
    pub access_token: String,
    pub my_drive_id: String,
}

#[async_trait]
impl AccessToken for OnedriveLiveAccessToken {
    async fn get_signed_url(&self, source: &FileDownloadSource) -> Result<String> {
        // we use item-id as `object_key` in onedrive.
        fn request_url(my_drive_id: &str, item_id: &str) -> String {
            format!("https://graph.microsoft.com/v1.0/drives/{}/items/{}", my_drive_id, item_id)
        }
        #[derive(Debug, Deserialize)]
        struct ResponseFile {
            #[serde(rename = "mimeType")]
            _mime_type: String,
        }
        #[derive(Debug, Deserialize)]
        struct Response {
            #[serde(rename = "file")]
            _file: ResponseFile,
            #[serde(rename = "@microsoft.graph.downloadUrl")]
            download_url: String
        }
        let client = reqwest::Client::new();
        let res = client
            .get(request_url(&self.my_drive_id, &source.object_key))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(res.download_url)
    }
}

#[async_trait]
impl ProviderAccount<OnedriveLiveAccessToken> for OnedriveConfig {
    async fn get_access_token(&self) -> Result<OnedriveLiveAccessToken> {
        let access_token = fetch_access_token(self).await?;
        let my_drive_id = fetch_my_od_id(&access_token).await?;
        Ok(OnedriveLiveAccessToken {
            access_token,
            my_drive_id,
        })
    }
}

impl DynamicAccessTokenAccount<OnedriveLiveAccessToken> for OnedriveConfig {
    fn general_lifetime() -> u64 {
        60 * 60
    }
}

async fn fetch_access_token(config: &OnedriveConfig) -> Result<String> {
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
    Ok(res.access_token)
}

async fn fetch_my_od_id(access_token: &str) -> Result<String> {
    const MY_DRIVE_URL: &str = "https://graph.microsoft.com/v1.0/me/drive";
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