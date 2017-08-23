#[macro_use]
extern crate clap;
extern crate pbr;
use clap::App;
use pbr::ProgressBar;
use std::process::Command;
use std::{thread, time};

const DEFAULT_TIME: u64 = 25;
const DEFAULT_BREAK_TIME: u64 = 5;
const DEFAULT_LOOP_NUMBER: u32 = 1;
const DEFAULT_FORMAT: &'static str = "[=>-]";
const DEFAULT_MESSAGE: &'static str = "Pomodoro complete! Please take a break.";

fn main() {
    // Load options using clap cli generator (yaml)
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Set matches to values
    let time = value_t!(matches, "time", u64).unwrap_or(DEFAULT_TIME);
    let break_time = value_t!(matches, "break", u64).unwrap_or(DEFAULT_BREAK_TIME);
    let loop_number = value_t!(matches, "loop", u32).unwrap_or(DEFAULT_LOOP_NUMBER);
    let format = matches.value_of("format").unwrap_or(DEFAULT_FORMAT);
    let message = matches.value_of("message").unwrap_or(DEFAULT_MESSAGE);
    let is_loop = matches.is_present("loop");
    let verbose = matches.is_present("verbose");

    let mut pb = ProgressBar::new(time);
    pb.format(format);
    pb.show_speed = false;
    pb.show_percent = false;
    pb.show_time_left = false;
    pb.show_tick = false;
    pb.set(0);
    for _ in 0..time {
        thread::sleep(time::Duration::from_secs(60));
        pb.inc();
    }
    pb.finish_print(message);
    Command::new("notify-send").arg(message).output();
}
