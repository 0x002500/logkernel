use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio::task;

use crate::structs::write_request::WriteRequest;

pub fn start_global_write_system() -> mpsc::Sender<WriteRequest> {
    //create channel
    let (global_tx, mut global_rx) = mpsc::channel::<WriteRequest>(2048);

    tokio::spawn(async move {

        while let Some(req) = global_rx.recv().await {
            let path: PathBuf = PathBuf::from(".")
                .join("data")
                .join(&req.app_name)
                .join("main.log");

            let _ = task::spawn_blocking(move || {
                use std::fs::OpenOptions;
                use std::io::Write;

                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                    let _ = file.write_all(req.data.as_bytes());
                }
            })
            .await;
        }
    });

    global_tx
}
