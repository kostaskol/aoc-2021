extern crate maplit;
use std::collections::HashMap;
use maplit::hashmap;
use crate::utils::input::read_file;

type SymbolList = Vec<Vec<char>>;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("10", test);
  let symbols = parse_symbols(&lines);

  format!("{}",
    match extra {
      false => p1::run(symbols),
      true => p2::run(symbols)
    }
  )
}

fn get_pairs() -> (HashMap<char, char>, HashMap<char, char>) {
  let pairs = hashmap!{
    '<' => '>',
    '{' => '}',
    '[' => ']',
    '(' => ')',
  };

  let reverse_pairs = hashmap!{
    '>' => '<',
    '}' => '{',
    ']' => '[',
    ')' => '(',
  };

  (pairs, reverse_pairs)
}

mod p1 {
  extern crate maplit;
  use std::collections::{VecDeque, HashMap};
  use maplit::hashmap;
  use super::{SymbolList, get_pairs};

  pub fn run(symbol_list: SymbolList) -> u64 {
    let mut stack: VecDeque<char> = VecDeque::new();
    let (pairs, reverse_pairs) = get_pairs();
    let scores = get_scores();
    let mut score = 0;

    for line in symbol_list {
      for symbol in line {
        if pairs.contains_key(&symbol) {
          stack.push_front(symbol);
        } else {
          let popped_symbol = stack.pop_front().unwrap();
          match reverse_pairs.get(&symbol) {
            None => panic!("Huh?"),
            Some(matching_sym) => {
              if *matching_sym != popped_symbol {
                score += scores.get(&symbol).unwrap_or(&0);
              }
            }
          }
        }
      }
    }
    score as u64
  }

  fn get_scores() -> HashMap<char, i32> {
    hashmap! {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137
    }
  }
}

mod p2 {
  extern crate maplit;
  use std::collections::{VecDeque, HashMap};
  use maplit::hashmap;
  use super::{SymbolList, get_pairs};

  pub fn run(symbol_list: SymbolList) -> u64 {
    let (pairs, reverse_pairs) = get_pairs();
    let score_table = get_autocomplete_score();
    let mut scores: Vec<u64> = Vec::new();

    for line in symbol_list {
      let mut corrupted = false;
      let mut stack: VecDeque<char> = VecDeque::new();
      for symbol in line {
        if pairs.contains_key(&symbol) {
          stack.push_front(symbol);
        } else {
          let popped_symbol = stack.pop_front().unwrap();
          match reverse_pairs.get(&symbol) {
            None => panic!("Symbol is neither opening nor closing. Bad input"),
            Some(matching_sym) => {
              if *matching_sym != popped_symbol {
                corrupted = true;
                break;
              }
            }
          }
        }
      }

      if !corrupted {
        let mut score: u64 = 0;
        while let Some(popped_symbol) = stack.pop_front() {
          let matching_sym = pairs.get(&popped_symbol).unwrap();
          score *= 5;
          score += score_table.get(matching_sym).unwrap_or(&0);
        }
        scores.push(score);
      }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
  }

  fn get_autocomplete_score() -> HashMap<char, u64> {
    hashmap! {
      ')' => 1,
      ']' => 2,
      '}' => 3,
      '>' => 4
    }
  }
}


fn parse_symbols(input: &[String]) -> SymbolList {
  let mut lines: SymbolList = Vec::new();

  for line in input {
    let mut symbols = Vec::new();
    for c in line.chars() {
      symbols.push(c);
    }
    lines.push(symbols);
  }
  lines
}

#[cfg(test)]
mod tests {
  use super::run;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "26397");
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "288957");
  }
}