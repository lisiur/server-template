use std::{path::PathBuf, sync::OnceLock};

use serde::{Deserialize, Serialize};

use crate::result::ServerResult;

static SETTING: OnceLock<Settings> = OnceLock::new();

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
    pub private_key: String,
    pub public_key: String,
    pub upload_dir: PathBuf,
    pub debug: Option<bool>,
}

impl Settings {
    pub fn init() -> ServerResult<&'static Settings> {
        let setting = config::Config::builder()
            .add_source(config::File::with_name("settings"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?
            .try_deserialize::<Settings>()?;

        SETTING.set(setting).unwrap();

        Ok(Settings::get())
    }

    pub fn get() -> &'static Settings {
        SETTING.get().unwrap()
    }

    pub fn json_pretty(&self, extra_indent: usize) -> String {
        serde_json::to_string_pretty(self)
            .unwrap()
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i > 0 {
                    format!("{}{}", " ".repeat(extra_indent), line)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
