use std::collections::VecDeque;
use std::path::PathBuf;
use utils;

#[derive(Debug)]
pub struct Movie {
    path: PathBuf,
    errors: u8,
}

impl Movie {
    fn new(path: PathBuf) -> Movie {
        Movie {
            path,
            errors: 0,
        }
    }
}

pub struct State {
    queue: VecDeque<Movie>
}

impl State {
    pub fn new() -> State {
        State {
            queue: VecDeque::new()
        }
    }

    // TODO: move to cron in separate thread
    pub fn start_discovering(&mut self, path: String) {
        println!("Start discovering in {}", path);
        let files = utils::walk_dir(path);
        println!("Found {} files", files.len());
        files.iter()
            .filter(|it| utils::ext_not_in(it.to_path_buf(), &["mp4", "avi", "mkv"]))
            .filter(|it| utils::not_hidden(it.to_path_buf()))
            .filter(|it| utils::not_converted(it.to_path_buf()))
            .for_each(|it| self.add_path(it.clone()));
    }

    pub fn add_path(&mut self, path: PathBuf) {
        self.queue.push_back(Movie::new(path))
    }

    pub fn run(&self) {
        for m in self.queue.iter() {
            println!("{:?}", m)
        }
    }

    pub fn start_force() {
        // TODO
    }

    pub fn stop_force() {
        // TODO
    }
}
