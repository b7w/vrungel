extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

mod utils;
mod core;

const USAGE: &'static str = "
Vrungel.

Usage:
  vrungel <path>
  vrungel (-h | --help)
  vrungel --version

Options:
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Searching in {}", args.arg_path);

    let res = utils::walk_dir(args.arg_path);
    println!("Found {} files", res.len());
    let mut state = core::State::new();
    res.iter()
        .for_each(|it| println!("{:?}", it.clone().file_name()));
    res.iter()
        .filter(|it| utils::ext_not_in(it.to_path_buf(), &["mp4", "avi", "mkv"]))
        .filter(|it| utils::not_hidden(it.to_path_buf()))
        .filter(|it| utils::not_converted(it.to_path_buf()))
        .for_each(|it| state.add_path(it.clone()));
    state.run();
}
