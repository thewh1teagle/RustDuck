use std::{sync::Arc, time::Duration};

use crate::config::DomainsConfig;
use eyre::{bail, Result};
use tokio::{sync::Mutex, time::sleep};

pub async fn update_domains(config: DomainsConfig) -> Result<Option<String>> {
    let client = reqwest::Client::new();
    let domains = config
        .domains
        .iter()
        .filter(|d| {
            if let Some(enable) = d.enable {
                return enable;
            }
            false
        })
        .map(|d| d.name.clone())
        .collect::<Vec<String>>()
        .join(",");
    if domains.is_empty() {
        return Ok(None);
    }
    let url = format!(
        "https://www.duckdns.org/update?domains={}&token={}",
        domains, config.token
    );
    log::debug!("GET {}", url);
    let res = client.get(url).send().await?.text().await?;
    if !res.contains("OK") {
        bail!("Bad response: {}", res);
    }
    Ok(Some(res))
}

pub async fn updater_task(config: Arc<Mutex<Option<DomainsConfig>>>) {
    loop {
        let config = config.lock().await.clone();
        if let Some(config) = config {
            let result = update_domains(config.to_owned()).await;

            match result {
                Ok(result) => {
                    log::debug!("Response: {:?}", result);
                }
                Err(report) => {
                    log::error!("error while updating: {:?}", report);
                }
            }
            log::debug!("Sleeping for {:?}", config.interval_minutes);
            let sleep_duration = config
                .interval_minutes
                .unwrap_or(crate::config::DEFAULT_INTERVAL);
            sleep(Duration::from_secs(sleep_duration)).await;
        } else {
            sleep(Duration::from_secs(10)).await;
        }
    }
}
