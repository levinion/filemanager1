use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{future::pending, os::unix::process::CommandExt, path::PathBuf, process::Command};
use vipera::{Configuration, Vipera};
use zbus::{connection, interface};

struct FileManager1 {
    config: Config,
}

#[derive(Serialize, Deserialize)]
struct Config {
    cmd: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cmd: "foot -e yazi {}".to_owned(),
        }
    }
}

impl Configuration for Config {
    fn vipera() -> Result<vipera::Vipera> {
        let vipera = Vipera::new()
            .set_config_name("config.toml")?
            .add_config_path("$XDG_CONFIG_HOME/filemanager1")?
            .add_config_path("~/.config/filemanager1")?
            .add_config_path("/usr/share/filemanager1")?;
        Ok(vipera)
    }
}

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManager1 {
    fn show_folders(&mut self, uris: Vec<String>, _startup_id: String) {
        let items = uris
            .iter()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter(|path| PathBuf::from(path).is_dir())
            .collect::<Vec<_>>()
            .join(" ");
        let items = self.config.cmd.replace("{}", &items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).process_group(0).spawn();
    }

    fn show_items(&mut self, uris: Vec<String>, _startup_id: String) {
        let items = uris
            .iter()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter(|path| PathBuf::from(path).is_file())
            .collect::<Vec<_>>()
            .join(" ");
        let items = self.config.cmd.replace("{}", &items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).process_group(0).spawn();
    }

    fn show_item_properties(&mut self, uris: Vec<String>, _startup_id: String) {
        let items = uris
            .iter()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .collect::<Vec<_>>()
            .join(" ");
        let items = self.config.cmd.replace("{}", &items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).process_group(0).spawn();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read_in_config().unwrap_or_default();
    let filemanager1 = FileManager1 { config };
    let _conn = connection::Builder::session()?
        .name("org.freedesktop.FileManager1")?
        .serve_at("/org/freedesktop/FileManager1", filemanager1)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
