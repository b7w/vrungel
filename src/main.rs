extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

mod utils;

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
    println!("Hello, world!");
    println!("{}", res.len());
    let strs: Vec<String> = res.into_iter().map(|it| it.to_str().unwrap().to_string()).collect();
    println!("{}", strs.join("\n"));
}
