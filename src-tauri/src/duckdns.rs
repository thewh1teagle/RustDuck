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
    tracing::debug!("GET {}", url);
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
                    tracing::debug!("Response: {:?}", result);
                }
                Err(report) => {
                    tracing::error!("error while updating: {:?}", report);
                }
            }

            let duration = config
                .interval_minutes
                .unwrap_or(crate::config::DEFAULT_INTERVAL_MINUTES)
                * 60;
            tracing::debug!("Sleeping for {:?} seconds", duration);
            sleep(Duration::from_secs(duration)).await;
        } else {
            // sleep for 10 seconds and check again
            // config may be updated
            sleep(Duration::from_secs(10)).await;
        }
    }
}
