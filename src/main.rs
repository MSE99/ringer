mod config;

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| "foo");

    let (_addr, server_fut) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3000), async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("gotten shutdown signal");
        });

    tokio::task::spawn(server_fut).await.unwrap();
}
