use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Domain {
    pub name: String,
    pub enable: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DomainsConfig {
    pub email: String,
    pub token: String,
    pub interval_minutes: Option<i32>,
    pub domains: Vec<Domain>,
}

pub const CONFIG_FILENAME: &str = "config.json";
pub const LOGIN_URL: &str = "https://www.duckdns.org/";
