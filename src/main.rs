#[macro_use]
extern crate clap;
extern crate pbr;
use clap::App;
use pbr::ProgressBar;
use std::process::Command;
use std::{thread, time};

const DEFAULT_TIME: u64 = 25;
const DEFAULT_BREAK_TIME: u64 = 5;
const DEFAULT_UNITS: &'static str = "m";
const DEFAULT_LOOP_NUMBER: u32 = 1;
const DEFAULT_FORMAT: &'static str = "[=>-]";
const DEFAULT_MESSAGE: &'static str = "Pomodoro complete! Please take a break.";

fn create_progress_bar(time: &u64, units: &str, format: &str, message: &str) {
    let time_scale = get_time_scale_for_units(units);
    let mut pb = ProgressBar::new(*time);
    pb.format(format);
    pb.show_speed = false;
    pb.show_percent = false;
    pb.show_time_left = false;
    pb.show_tick = false;
    pb.set(0);
    for _ in 0..*time {
        thread::sleep(time::Duration::from_millis(time_scale));
        pb.inc();
    }
    pb.finish_print(message);
}

fn get_time_scale_for_units(units: &str) -> u64 {
    match Some(units) {
        Some("ms") => 1,
        Some("s") => 1000,
        Some("m") => 60000,
        Some("h") => 3600000,
        _ => 60000
    }
}

fn run_execute(execute: &str) {
    if execute.len() > 0 {
        let mut execute_args: Vec<&str> = execute.split_whitespace().collect();
        let execute_command: &str = execute_args.remove(0);
        Command::new(execute_command)
            .args(execute_args)
            .spawn()
            .expect("execute command failed to start");
    }
}

fn main() {
    // Load options using clap cli generator (yaml)
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Set matches to values
    let time = value_t!(matches, "time", u64).unwrap_or(DEFAULT_TIME);
    let break_time = value_t!(matches, "break", u64).unwrap_or(DEFAULT_BREAK_TIME);
    let loop_number = value_t!(matches, "loop", u32).unwrap_or(DEFAULT_LOOP_NUMBER);
    let units = matches.value_of("units").unwrap_or(DEFAULT_UNITS);
    let format = matches.value_of("format").unwrap_or(DEFAULT_FORMAT);
    let message = matches.value_of("message").unwrap_or(DEFAULT_MESSAGE);
    let execute = matches.value_of("execute").unwrap_or("");
    let is_loop = matches.is_present("loop");
    let verbose = matches.is_present("verbose");

    create_progress_bar(&time, &units, &format, &message);

    run_execute(&execute);
}
