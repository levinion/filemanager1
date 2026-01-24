use std::{error::Error, future::pending, path::PathBuf, process::Command};
use zbus::{connection, interface};

struct FileManager1 {}

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManager1 {
    fn show_folders(&mut self, uris: Vec<String>, _startup_id: String) {
        let dirs = uris
            .iter()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter(|path| PathBuf::from(path).is_dir())
            .collect::<Vec<_>>()
            .join(" ");
        let items = format!("setsid foot -e yazi {}", dirs);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).spawn();
    }

    fn show_items(&mut self, uris: Vec<String>, _startup_id: String) {
        let items = uris
            .iter()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .filter(|path| PathBuf::from(path).is_file())
            .collect::<Vec<_>>()
            .join(" ");
        let items = format!("setsid foot -e yazi {}", items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).spawn();
    }

    fn show_item_properties(&mut self, uris: Vec<String>, _startup_id: String) {
        let items = uris
            .iter()
            .filter_map(|uri| uri.strip_prefix("file://"))
            .collect::<Vec<_>>()
            .join(" ");
        let items = format!("setsid foot -e yazi {}", items);
        let args = vec!["-c", &items];
        let _ = Command::new("sh").args(args).spawn();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filemanager1 = FileManager1 {};
    let _conn = connection::Builder::session()?
        .name("org.freedesktop.FileManager1")?
        .serve_at("/org/freedesktop/FileManager1", filemanager1)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
