use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Domain {
    pub name: String,
    pub enable: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DomainsConfig {
    pub email: String,
    pub token: String,
    pub interval_minutes: Option<u64>,
    pub domains: Vec<Domain>,
}

pub const CONFIG_FILENAME: &str = "config.json";
pub const LOGIN_URL: &str = "https://www.duckdns.org/";
pub const DEFAULT_INTERVAL_MINUTES: u64 = 10;
