#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // Load options using clap cli generator (yaml)
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Set matches to values
    let time = value_t!(matches, "time", u32).unwrap_or(25);
    let break_time = value_t!(matches, "break", u32).unwrap_or(5);
    let loop_number = value_t!(matches, "loop", u32).unwrap_or(1);
    let is_loop = matches.is_present("loop");
    let verbose = matches.is_present("verbose");
}
