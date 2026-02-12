use std::{sync::mpsc::channel, thread};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn start(app: AppHandle) {
    let downloads = app
        .path()
        .download_dir()
        .expect("Failed to resolve downloads directory");

    println!("Watching: {:?}", downloads);

    thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher =
            RecommendedWatcher::new(tx, Config::default())
                .expect("Failed to create watcher");

        watcher
            .watch(&downloads, RecursiveMode::Recursive)
            .expect("Failed to watch directory");

        println!("Watcher started");

        for res in rx {
            match res {
                Ok(event) => {
                    println!("File event: {:?}", event);
                }
                Err(e) => {
                    println!("Watch error: {:?}", e);
                }
            }
        }
    });
}

