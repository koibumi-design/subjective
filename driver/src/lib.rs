use std::collections::HashMap;
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
pub trait DriverTrait<Account: IndexedAccount>: DownloadPermitTrait<Account>
{
    fn try_get_account(&self, index: &str) -> Option<&Account>;
    fn driver_name() -> &'static str;
}

pub trait DriverCombinableTrait<Account: IndexedAccount>: DriverTrait<Account> {
    fn combine_drivers(a: Self<Account>, b: Self<Account>) -> Self<Account>;
}