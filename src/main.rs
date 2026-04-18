use anyhow::Result;
use itertools::Itertools;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::{
    future::pending,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};
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
            cmd: "foot -a yazi -e yazi {}".to_owned(),
        }
    }
}

impl Configuration for Config {
    fn vipera() -> Result<vipera::Vipera> {
        let vipera = Vipera::new()
            .set_config_name("config.toml")?
            .add_config_path("$XDG_CONFIG_HOME/filemanager1")?
            .add_config_path("~/.config/filemanager1")?
            .add_config_path("/etc/filemanager1")?;
        Ok(vipera)
    }
}

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManager1 {
    fn show_folders(&mut self, uris: Vec<String>, _startup_id: String) -> zbus::fdo::Result<()> {
        let raw_items = uris
            .iter()
            .unique()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter_map(|path| urlencoding::decode(path).ok())
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        let items = raw_items
            .iter()
            .filter_map(|path| {
                let path = Path::new(path);
                if path.is_dir() {
                    Some(path.to_path_buf())
                } else {
                    path.parent().map(|parent| parent.to_path_buf())
                }
            })
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        if items.is_empty() {
            let err = zbus::fdo::Error::FileNotFound(raw_items.join(" "));
            let _ = Notification::new()
                .summary("FileManager1")
                .body(&err.to_string())
                .show();
            return Err(err);
        }
        let items = self.config.cmd.replace("{}", &items.join(" "));
        eprintln!("show_folders: {}", &items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).process_group(0).spawn();
        Ok(())
    }

    fn show_items(&mut self, uris: Vec<String>, _startup_id: String) -> zbus::fdo::Result<()> {
        let raw_items = uris
            .iter()
            .unique()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter_map(|path| urlencoding::decode(path).ok())
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        let items = raw_items
            .iter()
            .filter(|path| PathBuf::from(path).exists())
            .cloned()
            .collect::<Vec<_>>();
        if items.is_empty() {
            let err = zbus::fdo::Error::FileNotFound(raw_items.join(" "));
            let _ = Notification::new()
                .summary("FileManager1")
                .body(&err.to_string())
                .show();
            return Err(err);
        }
        let items = self.config.cmd.replace("{}", &items.join(" "));
        eprintln!("show_items: {}", &items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).process_group(0).spawn();
        Ok(())
    }

    fn show_item_properties(
        &mut self,
        uris: Vec<String>,
        _startup_id: String,
    ) -> zbus::fdo::Result<()> {
        let raw_items = uris
            .iter()
            .unique()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter_map(|path| urlencoding::decode(path).ok())
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        let items = raw_items
            .iter()
            .filter(|path| PathBuf::from(path).exists())
            .cloned()
            .collect::<Vec<_>>();
        if items.is_empty() {
            let err = zbus::fdo::Error::FileNotFound(raw_items.join(" "));
            let _ = Notification::new()
                .summary("FileManager1")
                .body(&err.to_string())
                .show();
            return Err(err);
        }
        let items = self.config.cmd.replace("{}", &items.join(" "));
        eprintln!("show_item_properties: {}", &items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).process_group(0).spawn();
        Ok(())
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
