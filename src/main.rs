use std::path::Path;

mod config;
mod http;

#[tokio::main]
async fn main() {
    let mut current_path = std::env::current_dir().unwrap();
    current_path.push(Path::new("config.json"));

    let config =
        config::load_config_from(&current_path).unwrap_or_else(|_| config::default_config());

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
