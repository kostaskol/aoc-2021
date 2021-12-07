extern crate clap;
use clap::{Arg, App};

mod utils;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;

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
        .get_matches();

    let extra_star = matches.is_present("extra");
    let problem = matches.value_of("problem_number").unwrap_or("5");

    match problem {
        "1" => p1::run(extra_star),
        "2" => p2::run(extra_star),
        "3" => p3::run(extra_star),
        "4" => p4::run(extra_star),
        "5" => p5::run(extra_star),
        &_ => println!("Only know how to solve #{:?} for now :(", (1..=5))
    }
}
