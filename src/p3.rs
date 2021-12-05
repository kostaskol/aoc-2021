use crate::utils;

pub fn run(extra: bool) {
  let lines = utils::read_lines("inputs/3.txt");
  let binary = parse_binary(&lines);

  if extra {
    solve_two_stars(&binary);
  } else {
    solve_one_star(&binary);
  }
}

fn solve_two_stars(binary: &Vec<Vec<bool>>) {
  println!("{}", solve_1s(&binary) * solve_0s(&binary));
}

fn solve_0s(binary: &Vec<Vec<bool>>) -> isize {
  let byte_size = binary[0].len();
  let mut prev_binary = binary.clone();
  let mut answer = Vec::new();

  for i in 0..byte_size {
    let mut next_binary = Vec::new();
    let (ones, zeroes) = count_bits(&prev_binary, i);
    let min_bit = ones < zeroes;

    for byte in prev_binary {
      if byte[i] == min_bit {
        next_binary.push(byte.clone());
      }
    }
    prev_binary = next_binary.clone();

    if i == byte_size - 1 || prev_binary.len() == 1 {
      answer = next_binary;
      break;
    }
  }

  if answer.len() != 1 {
    panic!("Invalid binary: {:?}", answer);
  }

  bin_vec_to_dec(&answer[0])
}

fn solve_1s(binary: &Vec<Vec<bool>>) -> isize {
  let byte_size = binary[0].len();
  let mut prev_binary = binary.clone();
  let mut answer = Vec::new();
  for i in 0..byte_size {
    let mut next_binary = Vec::new();
    let (ones, zeroes) = count_bits(&prev_binary, i);
    let max_bit = ones >= zeroes;

    for byte in prev_binary {
      if byte[i] == max_bit {
        next_binary.push(byte.clone());
      }
    }
    prev_binary = next_binary.clone();

    if i == byte_size - 1 {
      answer = next_binary;
    }
  }

  if answer.len() != 1 {
    panic!("Invalid binary: {:?}", answer);
  }

  bin_vec_to_dec(&answer[0])
}

fn count_bits(binary: &Vec<Vec<bool>>, indx: usize) -> (usize, usize) {
  let mut ones = 0;
  let mut zeroes = 0;
  for row in binary {
    if row[indx] {
      ones += 1;
    } else {
      zeroes += 1;
    }
  }
  (ones, zeroes)
}

fn solve_one_star(binary: &Vec<Vec<bool>>) {
  let binary_size = binary[0].len();
  let mut cnt: Vec<i32> = vec![0; binary_size];

  for line in binary {
    for (i, bit) in line.iter().enumerate() {
      if *bit {
        cnt[i] += 1;
      }
    }
  }

  let mut gamma = vec![false; binary_size];
  let mut epsilon = vec![false; binary_size];

  for (i, el) in cnt.iter().enumerate() {
    let cond = *el > (binary.len() / 2) as i32;
    gamma[i] = cond;
    epsilon[i] = !cond;
  }

  let gamma_dec = bin_vec_to_dec(&gamma);
  let epsilon_dec = bin_vec_to_dec(&epsilon);

  println!("G: {} E: {}. Answer: {}", gamma_dec, epsilon_dec, gamma_dec * epsilon_dec);
}

fn bin_vec_to_dec(vec: &Vec<bool>) -> isize {
  let mut res = String::new();
  for bit in vec {
    if *bit {
      res.push_str("1");
    } else {
      res.push_str("0");
    }
  }

  isize::from_str_radix(&res, 2).unwrap()
}

fn parse_binary(lines: &Vec<String>) -> Vec<Vec<bool>> {
  let mut binary = Vec::new();
  for line in lines {
    let digits = line.chars().map(|s| s.to_string());
    binary.push(digits.map(|d| d == "1").collect());
  }

  binary
}