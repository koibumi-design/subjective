use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait DownloadPermitTrait<Credential: Sized>: Sized + Send + Sync {
    async fn get_signed_url(&self, credential: &Credential, path: &str) -> Result<String>;
}

pub trait IndexedAccount: Sized + Send + Sync {
    fn index_field(&self) -> String;
    fn match_index(&self, index: &str) -> bool {
        self.index_field() == index
    }
    fn build_hash_map(list: Vec<Self>) -> HashMap<String, Self> {
        list.into_iter().map(|x| (x.index_field(), x)).collect()
    }
}

#[async_trait]
pub trait DriverTrait<AccessToken: IndexedAccount>: DownloadPermitTrait<AccessToken>
{
    fn try_get_account(&self, index: &str) -> Option<&AccessToken>;
    fn driver_name() -> &'static str;
    fn link_expire_seconds() -> u64;
    async fn try_get_signed_url(&self, index: &str, path: &str) -> Result<String> {
        let account = self.try_get_account(index).ok_or(anyhow::anyhow!("Account not found"))?;
        self.get_signed_url(account, path).await
    }
}

pub trait ConfigurableDriverTrait<Config, AccessToken: IndexedAccount>: DriverTrait<AccessToken>
where Config: for<'de> serde::Deserialize<'de> + Sized
{
    fn from_config(config: Config) -> Result<Self>;
}

pub trait ClusterDriverTrait<Config, AccessToken: IndexedAccount>:
ConfigurableDriverTrait<Config, AccessToken> + CombinableDriverTrait<AccessToken>
where Config: PartialEq + Eq + Hash + for<'de> serde::Deserialize<'de>
{
    fn from_configs(config: HashSet<Config>) -> Result<Self>;
}

pub trait CombinableDriverTrait<Account: IndexedAccount>: DriverTrait<Account> {
    fn combine_drivers(a: Self, b: Self) -> Self;
}

#[async_trait]
pub trait DynamicAccessTokenDriver<AccessToken: IndexedAccount>: DriverTrait<AccessToken> {
    fn access_token_expire_seconds(&self) -> u64;
    async fn refresh_access_token(&self, old: &AccessToken) -> Result<AccessToken>;
    async fn refresh_all_access_tokens(&self) -> Result<()>;
}