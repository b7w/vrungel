use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub enum WalkErr {
    Io(io::Error)
}

fn _walk_dir<P: AsRef<Path>>(acc: &mut Vec<PathBuf>, path: P) {
    read_dir(path)
        .map_err(|it| println!("{}", it))
        .map(|files| {
            files.into_iter().for_each(|p| {
                p
                    .map_err(|it| println!("{}", it))
                    .map(|it| {
                        if it.path().is_dir() {
                            _walk_dir(acc, it.path())
                        }
                        if it.path().is_file() {
                            acc.push(it.path())
                        }
                    });
            })
        });
}

pub fn walk_dir<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut acc: Vec<PathBuf> = Vec::new();
    _walk_dir(&mut acc, path);
    return acc;
}
