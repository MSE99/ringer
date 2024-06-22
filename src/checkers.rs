use std::time::Duration;

use crate::config::Application;

pub fn deploy_checkers(apps: &Vec<Application>) {
    for app in apps.iter() {
        let app_settings_def = app.clone();

        tokio::task::spawn(async move {
            let dur = Duration::from_millis(app_settings_def.interval);
            let mut intr = tokio::time::interval(dur);

            println!("starting checker for {}", app_settings_def.name);

            loop {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        println!("shutting checker for {}", app_settings_def.name);
                        break
                    },

                    _ = intr.tick() => {
                        check_app(app_settings_def.clone()).await;
                    },
                }
            }
        });
    }
}

async fn check_app(app: Application) {
    println!("checking on {}", &app.name);

    let req_app = app.clone();
    let fut = tokio::spawn(async move { reqwest::get(req_app.status_url).await });

    tokio::select! {
        res = fut => {
            match res {
                Err(e) => {
                    println!("error while checking {}: {}", &app.name, e.to_string());
                },
                Ok(v) => {
                    match v {
                        Err(e) => {
                            println!("error while checking {}: {}", &app.name, e.to_string());
                            report_error(&app).await;
                        },
                        Ok(res) => {
                            println!("{} respond with {}", &app.name, res.status());
                        },
                    };
                },
            };
        },
        _ = tokio::signal::ctrl_c() => {
            println!("cancelling check for {}", app.name);
        },
    }
}

async fn report_error(app: &Application) {
    println!("reporting error for {}", app.name);

    for alerter in app.alerters.iter() {
        let req_alerter = alerter.clone();
        let fut = tokio::spawn(async move {
            let client = reqwest::Client::new();
            let serialized_body = serde_json::to_string(&req_alerter.payload).unwrap();

            client
                .post(reqwest::Url::parse(&req_alerter.url).unwrap())
                .body(serialized_body)
                .send()
                .await
        });

        tokio::select! {
            res = fut => {
                match res {
                    Err(_) => {},

                    Ok(res) => {
                        match res {
                            Err(e) => {
                                println!("error while reporting {}: {}", &app.name, e.to_string());
                            },
                            Ok(res) => {
                                println!("{} alerter {} respond with {}", &app.name, alerter.url, res.status());
                            },
                        };

                    },
                }
            },

            _ = tokio::signal::ctrl_c() => {
                println!("cancelling error report for {}", app.name);
            },
        }
    }
}
