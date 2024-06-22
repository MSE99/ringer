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

    let fut = tokio::spawn(async { reqwest::get(app.status_url).await });

    tokio::select! {
        _ = fut => {},
        _ = tokio::signal::ctrl_c() => {
            println!("cancelling check for {}", app.name);
        },
    }
}
