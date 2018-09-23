use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

pub const WAITE_TIME: Duration = Duration::from_secs(1);

trait PathExt {
    fn file_name_safe<'a>(&'a self, default: &'a str) -> &'a str;
}

impl PathExt for Path {
    fn file_name_safe<'a>(&'a self, default: &'a str) -> &'a str {
        self.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(default)
    }
}

pub enum WalkErr {
    Io(io::Error)
}

#[allow(unused)]
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


pub fn ext_not_in<P: AsRef<Path>>(path: P, check: &[&str]) -> bool {
    let ext_opt = path.as_ref().extension();
    let ext = ext_opt.unwrap_or("".as_ref());
    return check.into_iter().any(|it| it == &ext.to_str().unwrap_or(""));
}

pub fn not_hidden<P: AsRef<Path>>(path: P) -> bool {
    let name = path.as_ref().file_name_safe(".");
    return !name.starts_with(".");
}

pub fn not_converted<P: AsRef<Path>>(path: P) -> bool {
    let name = path.as_ref().file_name_safe(".");
    return !name.contains(".ipad.");
}
