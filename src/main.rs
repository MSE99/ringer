mod config;
mod http;

#[tokio::main]
async fn main() {
    let router = http::create_warp_server();

    let (_addr, server_fut) =
        warp::serve(router).bind_with_graceful_shutdown(([127, 0, 0, 1], 3000), async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("gotten shutdown signal");
        });

    tokio::task::spawn(server_fut).await.unwrap();
}
