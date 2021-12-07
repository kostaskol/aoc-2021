use crate::utils;

pub fn run(extra: bool) {
  let lines = utils::read_lines("inputs/7.txt");
  let input = parse_line(&lines[0]);

  if extra {
    run_two_stars(&input);
  } else {
    run_one_star(&input);
  }
}

fn run_two_stars(input: &Vec<i32>) {
  let max = *input.iter().max().unwrap() + 1;
  let mut fuel = vec![vec![0; max as usize]; input.len()];

  for (i, crab) in input.iter().enumerate() {
    for pos in 0..max {
      let distance = (pos - crab).abs();
      let sum = (distance.pow(2) + distance) / 2;
      fuel[i][pos as usize] = sum;
    }
  }

  let mut min = i32::MAX;
  for i in 0..fuel[0].len() {
    let mut sum = 0;
    for j in 0..fuel.len() {
      sum += fuel[j][i];
    }
    if sum < min {
      min = sum;
    }
  }

  println!("{}", min);
 }

fn run_one_star(input: &Vec<i32>) {
  let max = *input.iter().max().unwrap() + 1;
  let mut fuel = vec![vec![0; max as usize]; input.len()];

  for (i, crab) in input.iter().enumerate() {
    for pos in 0..max {
      fuel[i][pos as usize] = (pos - crab).abs();
    }
  }

  let mut min = i32::MAX;
  for i in 0..fuel[0].len() {
    let mut sum = 0;
    for j in 0..fuel.len() {
      sum += fuel[j][i];
    }
    if sum < min {
      min = sum;
    }
  }

  println!("{}", min);
}

fn parse_line(line: &str) -> Vec<i32> {
  line.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}