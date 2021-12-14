use crate::utils::read_file;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("7", test);
  let input = parse_line(&lines[0]);

  format!("{}",
    match extra {
      false => p1::run(input),
      true => p2::run(input)
    }
  )
}

fn parse_line(line: &str) -> Vec<i32> {
  line.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

mod p1 {
  pub fn run(input: Vec<i32>) -> i32 {
    let max = *input.iter().max().unwrap() + 1;
    let mut fuel = vec![vec![0; max as usize]; input.len()];

    for (i, crab) in input.iter().enumerate() {
      for pos in 0..max {
        fuel[i][pos as usize] = (pos - crab).abs();
      }
    }

    let mut min = i32::MAX;
    // This isn't actually needless range looping since
    // we want to loop all rows for each column at a time.
    // This cannot be done with iterators unless
    // we transpose the matrix but this isn't trivial in rust
    #[allow(clippy::needless_range_loop)]
    for i in 0..fuel[0].len() {
      let mut sum = 0;
      // See comment above for why we allow this
      #[allow(clippy::needless_range_loop)]
      for j in 0..fuel.len() {
        sum += fuel[j][i];
      }
      if sum < min {
        min = sum;
      }
    }

    min
  }
}

mod p2 {
  pub fn run(input: Vec<i32>) -> i32 {
    let max = *input.iter().max().unwrap() + 1;
    let mut fuel = vec![vec![0; max as usize]; input.len()];

    for (i, crab) in input.iter().enumerate() {
      for pos in 0..max {
        let distance = (pos - crab).abs();
        let sum = (distance.pow(2) + distance) / 2;
        fuel[i][pos as usize] = sum;
      }
    }

    // This isn't actually needless range looping since
    // we want to loop all rows for each column at a time.
    // This cannot be done with iterators unless
    // we transpose the matrix but this isn't trivial in rust
    #[allow(clippy::needless_range_loop)]
    let mut min = i32::MAX;
    // See comment above for why we allow this
    #[allow(clippy::needless_range_loop)]
    for i in 0..fuel[0].len() {
      let mut sum = 0;
      for j in 0..fuel.len() {
        sum += fuel[j][i];
      }
      if sum < min {
        min = sum;
      }
    }

    min
  }
}

#[cfg(test)]
mod tests {
  use super::run;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "37");
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "168");
  }
}