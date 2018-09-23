use std::collections::VecDeque;
use std::option::Option;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
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

    fn errors_inc(&mut self) {
        self.errors += 1;
    }
}

pub enum Status {
    DONE,
    CANCELED,
    ERROR,
}

pub struct Converter {
    task: Option<Movie>
}

impl Converter {
    pub fn new() -> Converter {
        Converter {
            task: None
        }
    }

    pub fn process(&mut self, movie: &Movie) -> Status {
        let status = Command::new("sleep")
            .arg("1")
            .status()
            .expect("failed to convert movie");

        return match status.success() {
            true => Status::DONE,
            false => Status::ERROR,
        };
    }
}

pub struct State {
    queue: VecDeque<Movie>,
    converter: Converter,
}

impl State {
    pub fn new() -> State {
        State {
            queue: VecDeque::new(),
            converter: Converter::new(),
        }
    }

    // TODO: move to cron in separate thread
    pub fn start_discovering(&mut self, path: String) {
        println!("Start discovering in {}", path);
        let files = utils::walk_dir(path);
        println!("Found {} files", files.len());
        files.into_iter()
            .filter(|it| utils::ext_not_in(it, &["mp4", "avi", "mkv"]))
            .filter(|it| utils::not_hidden(it))
            .filter(|it| utils::not_converted(it))
            .for_each(|it| self.add_path(it));
    }

    pub fn add_path(&mut self, path: PathBuf) {
        self.queue.push_back(Movie::new(path))
    }

    pub fn run(&mut self) {
        for m in self.queue.iter() {
            println!("{:?}", m)
        }
        loop {
            let m_opt = self.queue.pop_front();
            if m_opt.is_some() {
                let mut movie = m_opt.unwrap();
                let status = self.converter.process(&movie);
                match status {
                    Status::DONE => println!("Converted {:?}", movie),
                    Status::CANCELED => {
                        println!("Canceled {:?}", movie);
                        self.queue.push_back(movie);
                    }
                    Status::ERROR => {
                        println!("Error {:?}", movie);
                        movie.errors_inc();
                        self.queue.push_back(movie);
                    }
                }
            } else {
                println!("Sleep");
                thread::sleep(utils::WAITE_TIME);
            }
        }
    }

    pub fn start_force() {
        // TODO
    }

    pub fn stop_force() {
        // TODO
    }
}
