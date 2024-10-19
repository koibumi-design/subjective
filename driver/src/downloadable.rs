use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use async_trait::async_trait;
use anyhow::Result;
use crate::FileDownloadSource;

#[async_trait]
/// A `LiveAccessToken` can request a signed url from the provider.
pub trait LiveAccessToken: Sized + Send + Sync {
    async fn get_signed_url(&self, path: &str) -> Result<String>;
}

/// The trait for access token that can be indexed by a string.
pub trait IndexedLiveAccessToken: LiveAccessToken {
    fn index_field(&self) -> String;
    fn match_index(&self, index: &str) -> bool {
        self.index_field() == index
    }
    fn build_hash_map(list: Vec<Self>) -> HashMap<String, Self> {
        list.into_iter().map(|x| (x.index_field(), x)).collect()
    }
}

/// A container that can store access tokens.
pub trait AccessTokenContainer<AccessToken: IndexedLiveAccessToken> {
    fn try_get_account(&self, index: &str) -> Option<&AccessToken>;
}

#[async_trait]
pub trait DriverTrait {
    /// All drivers will be stored in a `HashMap<String, Arc<dyn DriverTrait>>` with their identifier as the key.
    fn identifier(&self) -> &str;
    fn link_expire_seconds() -> u64;
    async fn try_get_signed_url(&self, source: &FileDownloadSource) -> Result<String>;
}

#[async_trait]
pub trait ConfigurableDriverTrait<Config>: Sized
where Config: for<'de> serde::Deserialize<'de> + Sized
{
    async fn from_config(config: Config) -> Result<Self>;
}

pub trait ConfigurablePoolTrait<Config>: ConfigurableDriverTrait<Config>
where Config: PartialEq + Eq + Hash + for<'de> serde::Deserialize<'de>
{
    fn from_configs(config: HashSet<Config>) -> Result<Self>;
}

#[async_trait]
/// Some drivers need to refresh their access tokens.
///
/// This trait is for the access token itself.
pub trait DynamicAccessToken: LiveAccessToken {
    async fn refresh_self(&self) -> Result<Self>;
}

#[async_trait]
/// Some drivers need to refresh their access tokens.
pub trait DynamicAccessTokenDriver: DriverTrait {
    /// this can help manager to decide how often to refresh the access token.
    fn access_token_expire_seconds(&self) -> &'static u64;

    /// refresh all access tokens. Usually by calling [refresh_access_token](DynamicAccessToken::refresh_access_token) for each access token.
    async fn refresh_all_access_tokens(&self) -> Result<()>;
}