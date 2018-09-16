use std::collections::VecDeque;
use std::path::PathBuf;

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

    pub fn add_path(&mut self, path: PathBuf) {
        self.queue.push_back(Movie::new(path))
    }

    pub fn run(&self) {
        for m in self.queue.iter() {
            println!("{:?}", m)
        }
    }
}
