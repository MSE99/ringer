use std::path::{Path, PathBuf};

mod checkers;
mod config;
mod http;

#[tokio::main]
async fn main() {
    let config_path = get_config_path_at_cwd();

    let config =
        config::load_config_from(&config_path).unwrap_or_else(|_| config::default_config());

    checkers::deploy_checkers(&config.apps);
    http::run_http_server(&config).await;
}

fn get_config_path_at_cwd() -> PathBuf {
    let mut current_path = std::env::current_dir().expect("could not get cwd");
    current_path.push(Path::new("config.json"));
    current_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_config_path() {
        let result = get_config_path_at_cwd();
        assert!(result.ends_with("config.json"));
    }
}
