use crate::utils;

type InputLine = Vec<(Vec<String>, Vec<String>)>;

pub fn run(extra: bool, test: bool) -> String {
  let input = parse_lines(
    utils::read_lines(
      &utils::inp_file("8", test)
    )
  );

  format!("{}",
    match extra {
      false => p1::run(input),
      true => p2::run(input)
    }
  )
}

/**
 * Parses input lines into the following structure:
 * [
 *  ([input signals], [output signals]),
 *  ...
 * ]
 * 
 * input signals are 10 elements long
 * output signals are 4 element long
 * 
 * # Examples
 * 
 *  ```
 *  assert_eq!(
 *  parse_lines(vec!["1 2 3 4 5 6 7 8 9 10 | 1 2 3 4", "10 9 8 7 6 5 4 3 2 1 | 4 3 2 1"]),
 *  vec![
 *    (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
 *     vec![1, 2, 3, 4]),
 *    (vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
 *     vec![4, 3, 2, 1])
 *  ]
 * )
 * ```
 */ 
fn parse_lines(input: Vec<String>) -> InputLine {
  let mut res = Vec::new();

  for line in input {
    let split: Vec<&str> = line.split(" | ").collect();
    let left_split: Vec<String> = split[0].split_whitespace().map(|s| s.to_string()).collect();
    let right_split: Vec<String> = split[1].split_whitespace().map(|s| s.to_string()).collect();

    res.push((left_split, right_split));
  }

  res
}

mod p1 {
  use super::InputLine;

  pub fn run(input: InputLine) -> u32 {
    let mut unique: u32 = 0;
    for line in input {
      for digits in line.1 {
        match digits.len() {
          2 | 3 | 4 | 7 => unique += 1,
          _ => ()
        }
      }
    }

    unique
  }
}

mod p2 {
  use super::InputLine;

  /*
  * This is uber confusing but it's pretty simple:
  * 1. `numbers` holds the decoded numbers where the index is the actual number
  *    while the value is the encoded string
  * 2. `digits` is an 8-element array where the index is the length of the encoded
  *    string (as such some indices are empty) and the value is the *sorted* encoded string
  * 3. The encoded string -> number mapping is statically known for some strings based
  *    on their length (e.g. The string with a length of 2 is the number 1,
  *    the one with a length of 3 is 7, etc).
  *    Based one the above, the named variables (one & four) hold the encoded string
  *    representing the numbers (1 & 4) respectively.
  * 4. The `l` variable is a bit trickier: It's the difference between the encoded string
  *    representing 1 and the one representing 4:
  *    Let's say 4 is 'abcd' and 1 is 'ab'. The top left and center bars of 4 (forming and L)
  *    have to be `cd`, since `ab` are the top right and bottom right bars (of 1).
  * 5. Based on all the above, we can deduce all the encoded digits by simply
  *    checking which encoded strings have one of the known numbers as a substring
  *    (not necessarily consecutive) and their length.
  *    e.g. 
  *    Considering the following mapping of string length to possible numbers:
  *    (length -> digit)
  *    2 -> 1
  *    3 -> 7
  *    4 -> 4
  *    5 -> [2, 3, 5]
  *    6 -> [6, 9, 0]
  *    7 -> 8
  *    we know that out of the 3 possible 5-length encoded strings, only one of them
  *    contains all characters of the number 1 (3). Out of the remaining 2, only one
  *    contains the L we calculated before (5). As such, we've mapped all 5-length encoded
  *    strings to the correct number (the specific mapping of {a,f} is unimportant due
  *    to sorting the encoded strings).
  *    Doing the same for the 6-length encoded strings
  *    (this time 9 containing the entirety of 4, 6 containing the L and 0 being
  *    the remaining one), we have a mapping of all numbers.
  */
  pub fn run(input: InputLine) -> u32 {
    let mut sum = 0;
    for line in input {
      let digits = deep_sort(line.0);
      let mut numbers = [
        &digits[1][0], // 1
        "", // 2
        "", // 3
        &digits[3][0], // 4
        "", // 5
        "", // 6
        &digits[2][0], // 7
        &digits[6][0], // 8
        "", // 9
        "", // 0
      ];
      // There is only one 2 & one 4 length digit (1 & 4 respectively)
      let one = &digits[1][0];
      let four = &digits[3][0];
      let l = remove(four, one);

      // Five refers to digit length and not the number
      let fives = digits[4].clone();
      for five in fives.iter() {
        if contains(five, one) {
          // 3
          numbers[2] = five;
        } else if contains(five, &l) {
          // 5
          numbers[4] = five;
        } else {
          // 2
          numbers[1] = five;
        }
      }

      let sixes = digits[5].clone();
      for six in sixes.iter() {
        if contains(six, four) {
          // 9
          numbers[8] = six;
        } else if contains(six, &l) {
          // 6
          numbers[5] = six;
        } else {
          // 0
          numbers[9] = six;
        }
      }

      let mut res_digit = String::new();
      for digit in line.1 {
        for (i, num) in numbers.iter().enumerate() {

          if sort(digit.to_string()) == *num {
            if i == 9 {
              res_digit.push('0');
            } else {
              res_digit.push_str(&(i + 1).to_string());
            }
          }
        }
      }

      sum += res_digit.parse::<u32>().unwrap();
    }
    sum
  }

  fn remove(a: &str, b: &str) -> String {
    let mut res = a.chars().collect::<Vec<char>>();
    res.retain(|e| !b.contains(*e));
    res.iter().collect::<String>()
  }

  fn contains(a: &str, b: &str) -> bool {
    for c in b.chars() {
      if !a.contains(c) {
        return false;
      }
    }
    true
  }

  fn sort(digit: String) -> String {
    let mut digit = digit.chars().collect::<Vec<char>>();
    digit.sort_unstable();
    digit.iter().collect::<String>()
  }

  /*
  * Sorts the array by element length, then each element by alphabetical order.
  */
  fn deep_sort(digits: Vec<String>) -> Vec<Vec<String>> {
    let mut ret = vec![Vec::new(); 7];

    for digit in digits {
      let sorted_digit = sort(digit);
      let indx = sorted_digit.len() - 1;
      if ret[indx].contains(&sorted_digit) {
        continue;
      }

      ret[indx].push(sorted_digit);
    }

    ret
  }
}