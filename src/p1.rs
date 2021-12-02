use crate::utils;

pub fn run(extra: bool) {
  let lines = utils::read_lines("inputs/1.txt");
  let input: Vec<i32> = utils::convert_to_ints(&lines);

  if extra {
    run_two_stars(input);
  } else {
    run_one_star(input);
  }
}

// Count increases in values
fn run_one_star(input: Vec<i32>) {
  let mut cnt: u32 = 0;

  for i in 1..input.len() {
    if input[i] > input[i - 1] {
      cnt += 1;
    }
  }

  println!("{}", cnt);
}

// Count increases in values in windows of 3
fn run_two_stars(input: Vec<i32>) {
  let mut current_window: i32 = input[0..=2].iter().sum();
  let mut cnt: u32 = 0;

  for i in 3..input.len() {
    let previous_window = current_window;
    current_window = current_window + input[i] - input[i - 3];

    if current_window > previous_window {
      cnt += 1;
    }
  }

  println!("{}", cnt)
}