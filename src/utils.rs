use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::fmt::Display;

pub const WAITE_TIME: Duration = Duration::from_secs(2);

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

fn _walk_dir<P: AsRef<Path>>(acc: &mut Vec<PathBuf>, path: P) {
    ok_or_display(read_dir(path))
        .into_iter()
        .flatten()
        .flat_map(ok_or_display)
        .map(|it| it.path())
        .for_each(|it| {
            if it.is_dir() {
                _walk_dir(acc, it);
            } else if it.is_file() {
                acc.push(it);
            }
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

fn ok_or_display<T, E: Display>(res: Result<T, E>) -> Option<T> {
    match res {
        Ok(val) => Some(val),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}
