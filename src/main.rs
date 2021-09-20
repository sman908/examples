use std::{
    collections::VecDeque,
    env,
    fs::{self, DirEntry},
    io, iter,
    path::{Path, PathBuf},
};

fn recurse_dir<P: AsRef<Path>>(dir: P) -> impl Iterator<Item = io::Result<PathBuf>> {
    let mut unvisited = VecDeque::new();
    unvisited.push_back(dir.as_ref().to_path_buf());
    let mut entries: Box<dyn Iterator<Item = io::Result<DirEntry>>> = Box::new(iter::empty());
    iter::from_fn(move || loop {
        match entries.next() {
            Some(Ok(entry)) => match entry.file_type() {
                Ok(filetype) => {
                    let path = entry.path();
                    if filetype.is_dir() {
                        unvisited.push_back(path)
                    } else if filetype.is_file() {
                        return Some(Ok(path));
                    }
                }
                Err(err) => return Some(Err(err)),
            },
            Some(Err(err)) => return Some(Err(err)),
            None => {
                let dir = unvisited.pop_front()?;
                match fs::read_dir(dir) {
                    Ok(new_entries) => {
                        entries = Box::new(new_entries);
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
        }
    })
}

fn main() {
    let mut args = env::args().skip(1);
    let dir = args.next().unwrap_or(".".to_string());
    for entry in recurse_dir(dir) {
        match entry {
            Ok(path) => {
                println!("{}", path.to_string_lossy())
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}
