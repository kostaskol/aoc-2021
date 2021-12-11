use crate::utils;

pub fn run(extra: bool, test: bool) -> String {
  let lines = utils::read_lines(&utils::inp_file("1", test));
  let input: Vec<i32> = utils::convert_to_ints(&lines);

  format!("{}",
    match extra {
      true => p1::run(input),
      false => p2::run(input)
    }
  )
}

mod p1 {
  // Count increases in values
  pub fn run(input: Vec<i32>) -> u32 {
    let mut cnt: u32 = 0;

    for i in 1..input.len() {
      if input[i] > input[i - 1] {
        cnt += 1;
      }
    }

    cnt
  }
}

mod p2 {
  // Count increases in values in windows of 3
  pub fn run(input: Vec<i32>) -> u32 {
    let mut current_window: i32 = input[0..=2].iter().sum();
    let mut cnt: u32 = 0;

    for i in 3..input.len() {
      let previous_window = current_window;
      current_window = current_window + input[i] - input[i - 3];

      if current_window > previous_window {
        cnt += 1;
      }
    }

    cnt
  }
}
