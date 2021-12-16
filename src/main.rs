extern crate clap;
use clap::{Arg, App};

mod days;
mod utils;
mod parsing;

use crate::days::*;

fn main() {
    let matches = App::new("Advent of code!")
        .arg(Arg::new("extra")
                .short('e')
                .long("extra")
                .takes_value(false))
        .arg(Arg::new("problem_number")
                .short('p')
                .long("problem")
                .takes_value(true))
        .arg(Arg::new("test_input")
                .short('t')
                .long("test")
                .takes_value(false))
        .get_matches();

    let extra = matches.is_present("extra");
    let test = matches.is_present("test_input");
    let problem = matches.value_of("problem_number").unwrap_or("16");
    let answer: String =
        match problem {
            "1" => day1::run(extra, test),
            "2" => day2::run(extra, test),
            "3" => day3::run(extra, test),
            "4" => day4::run(extra, test),
            "5" => day5::run(extra, test),
            "6" => day6::run(extra, test),
            "7" => day7::run(extra, test),
            "8" => day8::run(extra, test),
            "9" => day9::run(extra, test),
            "10" => day10::run(extra, test),
            "11" => day11::run(extra, test),
            "12" => day12::run(extra, test),
            "13" => day13::run(extra, test),
            "14" => day14::run(extra, test),
            "15" => day15::run(extra, test),
            "16" => day16::run(extra, test),
            &_ => format!("Only know how to solve #{:?} for now :(", (1..=16))
        };

    println!("{}", answer);
}
