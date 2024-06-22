use warp::{reject::Rejection, Filter, Reply};

use crate::config::RingerConfig;

pub async fn run_http_server(config: &RingerConfig) {
    let router = create_warp_server();

    let (_addr, server_fut) = warp::serve(router).bind_with_graceful_shutdown(
        ([127, 0, 0, 1], config.http_server_port),
        async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("gotten shutdown signal");
        },
    );

    tokio::task::spawn(server_fut).await.unwrap();
}

fn create_warp_server() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().and(warp::path("status")).map(|| "OK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_have_status_route() {
        let router = create_warp_server();

        let res = warp::test::request()
            .method("GET")
            .path("/status")
            .reply(&router)
            .await;

        assert_eq!(res.status(), 200);
    }
}
