extern crate clap;
use clap::{Arg, App};

mod utils;
mod board;

mod day1; mod day2; mod day3; mod day4;
mod day5; mod day6; mod day7; mod day8;
mod day9; mod day10; mod day11;

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

    let extra_star = matches.is_present("extra");
    let test_input = matches.is_present("test_input");
    let problem = matches.value_of("problem_number").unwrap_or("11");
    let answer: String =
        match problem {
            "1" => day1::run(extra_star, test_input),
            "2" => day2::run(extra_star, test_input),
            "3" => day3::run(extra_star, test_input),
            "4" => day4::run(extra_star, test_input),
            "5" => day5::run(extra_star, test_input),
            "6" => day6::run(extra_star, test_input),
            "7" => day7::run(extra_star, test_input),
            "8" => day8::run(extra_star, test_input),
            "9" => day9::run(extra_star, test_input),
            "10" => day10::run(extra_star, test_input),
            "11" => day11::run(extra_star, test_input),
            &_ => format!("Only know how to solve #{:?} for now :(", (1..=11))
        };

    println!("{}", answer);
}
