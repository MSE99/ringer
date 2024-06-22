use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, path::PathBuf, time::Duration};

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
        payload: Value,
        authorization: Option<String>,
    },
}

pub fn load_config_from(p: &PathBuf) -> Result<RingerConfig> {
    let reader = File::open(p)?;
    let config: RingerConfig = serde_json::from_reader(reader)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    struct TempFile {
        path: PathBuf,
        file: File,
    }

    impl TempFile {
        fn new(path: PathBuf) -> Self {
            let file = File::create_new(path.clone())
                .expect(format!("could not create file {}", path.display()).as_str());

            TempFile { path, file }
        }

        fn add_content(&mut self, s: &str) -> Result<()> {
            self.file.write(s.as_bytes())?;
            Ok(())
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            std::fs::remove_file(&self.path)
                .expect(&format!("could not create file {}", self.path.display()));
        }
    }

    #[test]
    fn file_does_not_exist() {
        let path_that_does_not_exist = PathBuf::from("./path-that-does-not-exist.json");
        let result = load_config_from(&path_that_does_not_exist);

        assert!(result.is_err());
    }

    #[test]
    fn file_with_bad_json() {
        let path = PathBuf::from("test__bad_json.json");

        let mut temp = TempFile::new(path.clone());
        temp.add_content("As string").unwrap();

        let result = load_config_from(&path);
        assert!(result.is_err());
    }

    #[test]
    fn happy_path() {
        let path = PathBuf::from("test__happy_path.json");
        let mut temp = TempFile::new(path.clone());

        let app: Application = Application {
            name: String::from("hello world"),
            cool_down: None,
            interval: Duration::from_secs(10),
            status_url: String::from("https://salem.com"),
        };

        let apps: Vec<Application> = vec![app];
        let config = RingerConfig { apps };
        let serialized = serde_json::to_string(&config).unwrap();
        temp.add_content(&serialized).unwrap();

        let config_from_file = load_config_from(&path).unwrap();

        assert_eq!(config_from_file.apps.len(), config.apps.len());

        assert_eq!(
            config_from_file.apps.iter().nth(0).unwrap().name,
            String::from("hello world")
        );
    }
}
