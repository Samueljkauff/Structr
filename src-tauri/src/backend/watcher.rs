use std::{sync::mpsc::channel};
use notify::{Config, RecommendedWatcher, Watcher};
fn start() {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config.default()).unwrap();
}