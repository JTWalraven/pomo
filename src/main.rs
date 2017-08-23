#[macro_use]
extern crate clap;
extern crate pbr;
extern crate term;
extern crate shlex;
use clap::App;
use pbr::ProgressBar;
use std::{thread, time};
use std::io;
use std::io::prelude::*;
use std::process::Command;

const DEFAULT_TIME: u64 = 25;
const DEFAULT_BREAK_TIME: u64 = 5;
const DEFAULT_UNITS: &'static str = "m";
const DEFAULT_LOOP_NUMBER: u32 = 1;
const DEFAULT_FORMAT: &'static str = "[=>-]";
const DEFAULT_MESSAGE: &'static str = "Pomodoro complete!";
const DEFAULT_BREAK_MESSAGE: &'static str = "Break complete. Focus time!";

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = term::stdout().unwrap();
    stdout.reset().unwrap();

    print!("Press any key to continue...");
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

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
        let mut execute_args: Vec<String> = shlex::split(execute).unwrap_or(Vec::new());
        let execute_command: String = execute_args.remove(0);
        let _ = Command::new(execute_command)
            .args(execute_args)
            .spawn()
            .expect("execute command failed to start");
    }
}

fn run_pomodoro(time: &u64, units: &str, format: &str, message: &str, execute: &str) {
    create_progress_bar(&time, &units, &format, &message);

    run_execute(&execute);

    pause();
}

fn run_pomodoro_cycle(time: &u64, break_time: &u64, units: &str, format: &str, message: &str, break_message: &str, execute: &str, is_color: &bool) {
    // Run pomodoro
    let mut t = term::stdout().unwrap();
    if *is_color { t.fg(term::color::RED).unwrap(); }
    run_pomodoro(&time, &units, &format, &message, &execute);

    // Run break
    if *is_color { t.fg(term::color::BLUE).unwrap(); }
    run_pomodoro(&break_time, &units, &format, &break_message, &execute);

    // Clear formating
    if *is_color { t.reset().unwrap(); }
}

fn main() {
    // Load options using clap cli generator (yaml)
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Set matches to values
    let time = value_t!(matches, "time", u64).unwrap_or(DEFAULT_TIME);
    let break_time = value_t!(matches, "break", u64).unwrap_or(DEFAULT_BREAK_TIME);
    let loop_number = value_t!(matches, "number", u32).unwrap_or(DEFAULT_LOOP_NUMBER);
    let units = matches.value_of("units").unwrap_or(DEFAULT_UNITS);
    let format = matches.value_of("format").unwrap_or(DEFAULT_FORMAT);
    let message = matches.value_of("message").unwrap_or(DEFAULT_MESSAGE);
    let break_message = matches.value_of("break-message").unwrap_or(DEFAULT_BREAK_MESSAGE);
    let execute = matches.value_of("execute").unwrap_or("");
    let is_color = matches.is_present("color");
    let is_loop = matches.is_present("loop");

    // Execute loop
    if is_loop {
        loop {
            run_pomodoro_cycle(&time, &break_time, &units, &format, &message, &break_message, &execute, &is_color);
        }
    } else {
        for _ in 0..loop_number {
            run_pomodoro_cycle(&time, &break_time, &units, &format, &message, &break_message, &execute, &is_color);
        }
    }

}
