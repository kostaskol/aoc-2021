use crate::utils::read_file;
use crate::board::{Point, Board};

const BOARD_LENGTH: usize = 5;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("4", test);

  let (inputs, boards) = parse_lines(lines);

  format!("{}",
    match extra {
      false => p1::run(inputs, boards),
      true => p2::run(inputs, boards)
    }
  )
}

pub struct BingoBoard {
  board: Board<(String, bool)>,
  indx: usize
}

impl BingoBoard {
  fn new_draw(&mut self, draw: &str) -> bool {
    let dim = self.board.dim();
    for i in 0..dim.0 {
      for j in 0..dim.1 {
        let p = (i, j);
        if self.board.get(p).unwrap().0 == draw {
          return self.mark(p);
        }
      }
    }
    false
  }

  fn mark(&mut self, p: Point) -> bool {
    self.board.get_mut(p).unwrap().1 = true;
    self.check_win(p)
  }

  fn check_win(&self, p: Point) -> bool{
    let hor_win = self.board.expose()[p.0].iter().all(|(_, b)| *b);
    let mut ver_win = false;
    let mut cnt = 0;
    let dim = self.board.dim();
    for _ in 0..dim.0 {
      if self.board.get(p).unwrap().1 {
        cnt += 1;
      }
      ver_win = cnt == dim.1;
      if ver_win {
        break;
      }
    }

    hor_win || ver_win
  }

  fn score(&self, latest: &str) -> i32 {
    let mut score = 0;
    for row in self.board.expose().iter() {
      for col in row {
        if !col.1 {
          score += col.0.parse::<i32>().unwrap();
        }
      }
    }

    score * latest.parse::<i32>().unwrap()
  }

  fn from_lines(lines: &[String], indx: usize) -> Self {
    let mut b = Vec::<Vec::<(String, bool)>>::new();

    for row in lines {
      let data: Vec<(String, bool)> = row.split_whitespace().into_iter().
                                        map(|e| (e.to_string(), false)).collect();
      b.push(data);
    }

    Self {
      board: Board::<(String, bool)>::from(b),
      indx
    }
  }
}

fn parse_lines(lines: Vec<String>) -> (Vec<String>, Vec<BingoBoard>) {
  let inputs: Vec<String> = lines[0].split(',').map(|s| s.to_string()).collect();

  let mut boards = Vec::<BingoBoard>::new();
  let mut cnt = 2;
  let mut indx = 0;

  loop {
    boards.push(BingoBoard::from_lines(&lines[cnt..cnt+BOARD_LENGTH], indx));
    indx += 1;
    cnt += BOARD_LENGTH + 1; // 1 blank line

    if cnt >= lines.len() {
      break;
    }
  }

  (inputs, boards)
}

mod p1 {
  use super::BingoBoard;

  pub fn run(inputs: Vec<String>, mut boards: Vec<BingoBoard>) -> i32 {
    for input in inputs.iter() {
      for board in &mut boards.iter_mut() {
        if board.new_draw(input) {
          return board.score(input);
        }
      }
    }

    -1
  }
}

mod p2 {
  use super::BingoBoard;

  pub fn run(inputs: Vec<String>, mut boards: Vec<BingoBoard>) -> i32 {
    let mut scores: Vec<(i32, usize)> = Vec::new();
    for number in inputs {
      for board in boards.iter_mut() {
        if scores.iter().map(|s: &(i32, usize)| s.1).any(|e| e == board.indx) {
          continue;
        }
        if board.new_draw(&number) {
          let score = board.score(&number);
          scores.push((score, board.indx));
        }
      }
    }

    1
  }
}

// TODO: Fix and write tests