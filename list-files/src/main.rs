use std::{fs, path::PathBuf};

fn main() {
    list_contents_of_path(PathBuf::from("/"));
}

fn list_contents_of_path(path: PathBuf) {
    println!("{}", path.to_str().unwrap());
    if let Ok(paths) = fs::read_dir(path) {
        for path in paths {
            let Ok(path) = path else { continue };
            list_contents_of_path(path.path());
        }
    }
}
