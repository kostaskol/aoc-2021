use std::collections::VecDeque;
use std::collections::HashMap;
extern crate maplit;

use maplit::hashmap;
use crate::utils;

type SymbolList = Vec<Vec<char>>;

pub fn run(extra: bool) -> String {
  let lines = utils::read_lines("inputs/10.txt");
  let symbols = parse_symbols(&lines);

  format!("{}",
    match extra {
      false => run_one_star(symbols),
      true => run_two_stars(symbols)
    }
  )
}

fn run_one_star(symbol_list: SymbolList) -> u64 {
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

fn run_two_stars(symbol_list: SymbolList) -> u64 {
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
        score += score_table.get(&matching_sym).unwrap_or(&0);
      }
      scores.push(score);
    }
  }
  scores.sort();
  scores[scores.len() / 2]
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

fn get_scores() -> HashMap<char, i32> {
  hashmap! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137
  }
}

fn get_autocomplete_score() -> HashMap<char, u64> {
  hashmap! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4
  }
}