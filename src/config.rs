use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RingerConfig {
    apps: Vec<Application>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    name: String,
    status_url: String,
    interval: Duration,
    cool_down: Option<Duration>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Alerter {
    HttpAlerter {
        url: String,
        payload: HashMap<String, String>,
        authorization: Option<String>,
    },
}
