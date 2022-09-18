use std::{fs, path::PathBuf, sync::mpsc::Sender};

pub fn run(root_path: PathBuf, depth: u8, sender: Sender<PathBuf>) {
    recurse(root_path, 1, depth, &sender);
}

fn recurse(path: PathBuf, depth: u8, max_depth: u8, sender: &Sender<PathBuf>) -> Vec<PathBuf> {
    let mut paths = vec![];
    let children = get_all_dirs(path);
	children.iter().for_each(|child| {
		sender.send(child.clone()).unwrap();
	});
    paths.append(&mut children.clone());

    if depth < max_depth {
        for child in children {
            paths.append(&mut recurse(child, depth + 1, max_depth, sender))
        }
    }

    return paths;
}

fn get_all_dirs(path: PathBuf) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.metadata().unwrap().is_dir() {
                    paths.push(entry.path());
                }
            }
        }
    }
    paths
}
