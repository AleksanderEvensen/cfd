use std::{collections::VecDeque, fs, path::PathBuf, sync::mpsc::Sender};

struct PathEntry {
    path: PathBuf,
    depth: u8,
}

pub fn run(root_path: PathBuf, max_depth: u8, sender: Sender<PathBuf>) {
    let mut search_queue: VecDeque<PathEntry> = get_all_dirs(&root_path, 1);

    while let Some(next_dir) = search_queue.pop_front() {
        if next_dir.depth < max_depth {
            search_queue.append(&mut get_all_dirs(&next_dir.path, next_dir.depth + 1));
        }
        sender.send(next_dir.path).unwrap();
    }
    drop(sender);
}

fn get_all_dirs(path: &PathBuf, depth: u8) -> VecDeque<PathEntry> {
    let mut paths: VecDeque<PathEntry> = VecDeque::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.metadata().unwrap().is_dir() {
                    paths.push_back(PathEntry {
                        path: entry.path(),
                        depth,
                    });
                }
            }
        }
    }
    paths
}
