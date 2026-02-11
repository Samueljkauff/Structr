use std::{sync::mpsc::channel};
use notify::{Config, RecommendedWatcher, Watcher};

fn start() {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config.default()).unwrap();

    watcher.watch(Path::new("~/Downloads")).unwrap();
    
    for res in rx {
        match res {
            Ok(_) => handle_change(),
            Err(_) => Err("ERROR"),
        }
    }
}

fn handle_change() {
    println!("hello");
}