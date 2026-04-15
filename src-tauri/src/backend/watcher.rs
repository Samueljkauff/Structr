use std::{
    fs::{self},
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread,
    time::Duration,
};

use notify::{event::CreateKind, Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Manager};

use crate::backend::db;
use crate::{backend::file_manager::move_file, domain::file_meta::FileMeta};

#[tauri::command]
pub fn start(app: AppHandle) {
    let downloads = initialize_download_directory(&app);
    println!("Watching: {:?}", downloads);
    let app_clone = app.clone();

    thread::spawn(move || {
        run_watcher(app_clone, &downloads);
    });
}

fn initialize_download_directory(app: &AppHandle) -> PathBuf {
    app.path()
        .download_dir()
        .expect("Failed to resolve downloads directory")
}

fn run_watcher(app: AppHandle, downloads: &Path) {
    let (tx, rx) = channel();

    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher");

    watcher
        .watch(&downloads, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("Watcher started");

    for res in rx {
        let event = match res {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                continue;
            }
        };

        for path in event.paths {
            if !is_file_done_downloading(&path, event.kind) {
                continue;
            }

            println!("File download detected: {:?}, {:?}", event.kind, path);
            let file_path = path.clone();

            match FileMeta::new(&file_path) {
                Ok(data) => {
                    let app_handle = app.clone();
                    let meta = data.clone();

                    tauri::async_runtime::spawn(async move {
                        let service =
                            crate::backend::classification_service::ClassificationService {
                                classifier: crate::backend::ml_classifier::MLClassifier {
                                    model: "llama3".into(),
                                },
                            };

                        let result = service.classify(&app_handle, &meta).await;

                        println!("Classification result: {:?}", result);

                        let destination = &result.suggested_path;
                        if destination.as_os_str().is_empty() {
                            eprintln!("Invalid classification → skipping");
                            return;
                        }
                        let file_name = file_path.file_name().unwrap();
                        let full_destination = Path::new(&destination).join(file_name);

                        if let Err(e) = move_file(&path, &full_destination) {
                            eprintln!("Move failed: {:?}", e);
                        } else {
                            println!("Moved file to: {:?}", &full_destination);
                            let mut conn = db::establish_connection(&app_handle).expect("Database Error");

                            let _ = db::insert_file_move(&mut conn, 
                                file_path.to_str().unwrap(),
                                full_destination.to_str().unwrap(),
                            );
                        }
                    });
                }
                Err(e) => eprintln!("Metadata error: {:?}", e),
            }
        }
    }
}

fn is_file_done_downloading(path: &Path, event_kind: EventKind) -> bool {
    if !path.exists() {
        return false;
    }

    if !path.is_file() {
        return false;
    }

    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e,
        None => return false,
    };

    if ext == "crdownload" || ext == "part" || ext == "tmp" {
        return false;
    }

    let file_size1 = match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => return false,
    };

    thread::sleep(Duration::from_millis(500));

    let file_size2 = match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => return false,
    };

    if file_size1 != file_size2 {
        return false;
    }

    if event_kind != EventKind::Create(CreateKind::File) {
        return false;
    }

    if is_hidden(&path) {
        return false;
    }

    true
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}
