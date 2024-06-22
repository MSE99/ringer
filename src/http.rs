use warp::{reject::Rejection, Filter, Reply};

pub fn create_warp_server() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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
