use std::fs;
use std::fs::DirEntry;
use crate::domain::folder_node::FolderNode;

#[tauri::command]
pub fn load_children(root: String) -> Vec<FolderNode> {
    let mut nodes = Vec::new();

    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            if is_hidden(&entry) || is_file(&entry) {
                continue;
            }
            let path = entry.path();
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            nodes.push(FolderNode {
                name: entry.file_name().to_string_lossy().to_string(),
                path: path.to_string_lossy().to_string(),
                is_directory: metadata.is_dir(),
                parent: Some(root.clone()),
                children: Vec::new(),
            });
        }
    }
    nodes
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn is_file(entry: &DirEntry) -> bool {
    entry.metadata().map(|m| m.is_file()).unwrap_or(false)

}