use crate::endpoint::BaseUrls;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub region: String,
    pub shard: String,
    pub port: u16,
    pub base_urls: BaseUrls,
    pub lockfile_password: String,
    pub entitlement_token: String,
    pub client_version: String,
    pub client_platform: String,
    pub auth_token: String,
    pub puuid: String,
}