use std::collections::VecDeque;
use std::marker::Sync;
use std::option::Option;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use subprocess::Exec;
use subprocess::ExitStatus;
use subprocess::Popen;
use utils;

#[derive(Debug)]
pub struct Movie {
    path: PathBuf,
    errors: u8,
}


#[derive(Debug)]
pub struct Msg {
    path: PathBuf
}


impl Msg {
    fn new(path: PathBuf) -> Msg {
        Msg {
            path,
        }
    }
}

unsafe impl Send for Msg {}

unsafe impl Sync for Msg {}

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
    process: Option<Popen>
}

impl Converter {
    pub fn new() -> Converter {
        Converter {
            process: None
        }
    }

    // TODO: Return result with Status and composite Error
    // TODO: Add Updating self.process
    pub fn process(&mut self, _movie: &Movie) -> Status {
        let p_res = Exec::cmd("sleep").arg("2").popen();
        match p_res {
            Ok(mut p) => {
                if let Some(status) = p.wait_timeout(utils::MOVIE_NAX_CONVERT_TIME).expect("Could not wait") {
                    match status {
                        ExitStatus::Exited(0) => {
                            return Status::DONE;
                        }
                        _ => {
                            return Status::ERROR;
                        }
                    }
                } else {
                    p.kill().expect("Could not kill");
                    p.wait().expect("Could not wait");
                    return Status::ERROR;
                }
            }
            Err(_e) => {
                return Status::ERROR;
            }
        }
    }

    pub fn cancel(&mut self) {}
}

pub struct State {
    queue: VecDeque<Movie>,
    sender: Sender<Msg>,
    receiver: Receiver<Msg>,
    converter: Converter,
}

impl State {
    pub fn new() -> State {
        let (tx, rx) = channel();
        State {
            queue: VecDeque::new(),
            sender: tx,
            receiver: rx,
            converter: Converter::new(),
        }
    }

    // TODO: move to cron in separate thread
    pub fn start_discovering(&mut self, path: String) {
        println!("Start discovering in {}", path);
        {
            let s = self.sender.clone();
            thread::spawn(move || {
                loop {
                    State::discovering(s.clone(), path.clone());
                    thread::sleep(utils::WAITE_TIME);
                }
            });
        }
    }

    fn discovering(sender: Sender<Msg>, path: String) {
        let files = utils::walk_dir(path);
        println!("Found {} files", files.len());
        files.into_iter()
            .filter(|it| utils::ext_not_in(it, &["mp4", "avi", "mkv"]))
            .filter(|it| utils::not_hidden(it))
            .filter(|it| utils::not_converted(it))
            .for_each(|it| {
                sender.send(Msg::new(it)).unwrap()
            });
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
