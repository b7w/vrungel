use std::collections::HashSet;
use std::collections::VecDeque;
use std::option::Option;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
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
                println!("{:?}", self.process);
                return Status::ERROR;
            }
        }
    }

    #[allow(unused)]
    pub fn cancel(&mut self) {
        unimplemented!()
    }
}

pub struct State {
    queue: Arc<Mutex<VecDeque<Movie>>>,
    converter: Arc<Mutex<Converter>>,
}

impl State {
    pub fn new() -> State {
        State {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            converter: Arc::new(Mutex::new(Converter::new())),
        }
    }

    pub fn start_discovering(&mut self, path: String) {
        println!("Start discovering in {}", path);
        {
            let queue = self.queue.clone();
            thread::spawn(move || {
                let mut state = HashSet::new();
                loop {
                    let mut q = queue.lock().unwrap();
                    State::discovering(&mut state, &mut q, path.clone());
                    thread::sleep(utils::WAITE_TIME);
                }
            });
        }
    }

    fn discovering(state: &mut HashSet<PathBuf>, queue: &mut MutexGuard<VecDeque<Movie>>, path: String) {
        let files = utils::walk_dir(path);
        files.into_iter()
            .filter(|it| utils::ext_not_in(it, &["mp4", "avi", "mkv"]))
            .filter(|it| utils::not_hidden(it))
            .filter(|it| utils::not_converted(it))
            .for_each(|it| {
                if state.insert(it.clone()) {
                    queue.push_back(Movie::new(it.clone()));
                    println!("Discover: {:?}", it)
                }
            });
    }

    pub fn start_conveter(&mut self) {
        println!("Start converter");
        {
            let queue = self.queue.clone();
            let converter = self.converter.clone();
            thread::spawn(move || {
                loop {
                    let mut q = queue.lock().unwrap();
                    let mut c = converter.lock().unwrap();
                    let m_opt = q.pop_front();
                    if m_opt.is_some() {
                        let mut movie = m_opt.unwrap();
                        let status = c.process(&movie);
                        match status {
                            Status::DONE => println!("Converted {:?}", movie),
                            Status::CANCELED => {
                                println!("Canceled {:?}", movie);
                                q.push_back(movie);
                            }
                            Status::ERROR => {
                                println!("Error {:?}", movie);
                                movie.errors_inc();
                                q.push_back(movie);
                            }
                        }
                    } else {
                        println!("No work");
                        thread::sleep(utils::WAITE_TIME);
                    }
                }
            });
        }
    }

    #[allow(unused)]
    pub fn queue_size(&self) -> String {
        match self.queue.lock() {
            Ok(q) => format!("{}", q.len()),
            Err(e) => e.to_string()
        }
    }

    #[allow(unused)]
    pub fn start_force() {
        unimplemented!()
    }

    #[allow(unused)]
    pub fn stop_force() {
        unimplemented!()
    }
}
