use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use config::{load_config_from, Application};

mod config;
mod http;

#[tokio::main]
async fn main() {
    let config_path = get_config_path_at_cwd();
    let config = load_config_from(&config_path).unwrap_or_else(|_| config::default_config());

    deploy_checkers(&config.apps);
    run_http_server(&config).await;
}

fn get_config_path_at_cwd() -> PathBuf {
    let mut current_path = std::env::current_dir().unwrap();
    current_path.push(Path::new("config.json"));
    current_path
}

async fn run_http_server(config: &config::RingerConfig) {
    let router = http::create_warp_server();

    let (_addr, server_fut) = warp::serve(router).bind_with_graceful_shutdown(
        ([127, 0, 0, 1], config.http_server_port),
        async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("gotten shutdown signal");
        },
    );

    tokio::task::spawn(server_fut).await.unwrap();
}

fn deploy_checkers(apps: &Vec<Application>) {
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
