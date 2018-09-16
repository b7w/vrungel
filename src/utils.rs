use std::ffi::OsStr;
use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

trait PathBufMix {
    fn file_name_safe<'a>(&'a self, default: &'a str) -> &'a str;
}

impl PathBufMix for PathBuf {
    fn file_name_safe<'a>(&'a self, default: &'a str) -> &'a str {
        let mut opt_name = self.file_name().unwrap_or(OsStr::new(default));
        let opt_str = opt_name.to_str();
        return opt_str.unwrap_or(default);
    }
}

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


pub fn ext_not_in(path: PathBuf, check: &[&str]) -> bool {
    let mut ext_opt = path.extension();
    let ext = ext_opt.unwrap_or("".as_ref());
    return check.into_iter().any(|it| it == &ext.to_str().unwrap_or(""));
}

pub fn not_hidden(path: PathBuf) -> bool {
    let mut name = path.file_name_safe(".");
    return !name.starts_with(".");
}

pub fn not_converted(path: PathBuf) -> bool {
    let name = path.file_name_safe(".");
    return !name.contains(".ipad.");
}
