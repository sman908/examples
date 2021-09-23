use std::{
    collections::VecDeque,
    env,
    fs::{self, DirEntry},
    io::{self, Write},
    iter,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

fn recurse_dir<P: AsRef<Path>>(dir: P) -> impl Iterator<Item = io::Result<DirEntry>> {
    let mut unvisited = VecDeque::new();
    // unvisited.push_back(dir.as_ref().to_path_buf());
    let mut maybe_entries = fs::read_dir(dir);
    iter::from_fn(move || loop {
        match &mut maybe_entries {
            Ok(entries) => {
                match entries.next() {
                    Some(Ok(entry)) => {
                        match entry.file_type() {
                            Ok(filetype) => {
                                if filetype.is_dir() {
                                    unvisited.push_back(entry.path())
                                }
                                return Some(Ok(entry));
                            },
                            Err(e) => return Some(Err(e)),
                        }
                    },
                    Some(Err(e)) => return Some(Err(e)),
                    None => {
                        let dir = unvisited.pop_front()?;
                        maybe_entries = fs::read_dir(dir);
                    },
                }
            },
            Err(_) => return Some(Err(io::Error::last_os_error())),
        };
    })
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let dir = args.next().unwrap_or(".".to_string());
    let mut out = io::stdout();
    let mut err = io::stderr();
    for entry in recurse_dir(dir) {
        match entry {
            Ok(entry) => {
                out.write_all(entry.path().as_os_str().as_bytes())?;
            }
            Err(e) => {
                write!(err, "{}", e)?;
            }
        }
    }
    Ok(())
}
