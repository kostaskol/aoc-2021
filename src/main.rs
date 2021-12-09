extern crate clap;
use clap::{Arg, App};

mod utils;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;

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
    let problem = matches.value_of("problem_number").unwrap_or("9");
    let answer: String =
        match problem {
            "1" => p1::run(extra_star),
            "2" => p2::run(extra_star),
            "3" => p3::run(extra_star),
            "4" => p4::run(extra_star),
            "5" => p5::run(extra_star),
            "6" => p6::run(extra_star),
            "7" => p7::run(extra_star),
            "8" => p8::run(extra_star),
            "9" => p9::run(extra_star),
            &_ => format!("Only know how to solve #{:?} for now :(", (1..=9))
        };

    println!("{}", answer);
}
