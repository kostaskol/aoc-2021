use crate::utils;

const DIM_HOR: usize = 5;
const DIM_VER: usize = 5;

pub fn run(extra: bool) -> String {
  let lines = utils::read_lines("inputs/4.txt");

  let (inputs, mut boards) = parse_lines(&lines);

  format!("{}",
    match extra {
      true => run_two_stars(&inputs, &mut boards),
      false => run_one_star(&inputs, &mut boards)
    }
  )
}


#[derive(Debug)]
struct Board {
  board: [[(String, bool);DIM_HOR];DIM_VER],
  indx: i32
}

impl Board {
  fn new_draw(&mut self, draw: String) -> bool {
    for i in 0..DIM_HOR {
      for j in 0..DIM_VER {
        if self.board[i][j].0 == draw {
          return self.mark(i, j);
        }
      }
    }
    false
  }

  fn mark(&mut self, i: usize, j: usize) -> bool {
    self.board[i][j].1 = true;
    self.check_win(i, j)
  }

  fn check_win(&self, i: usize, j: usize) -> bool{
    let hor_win = self.board[i].iter().all(|(_, b)| *b);
    let mut ver_win = false;
    let mut cnt = 0;
    for i in 0..DIM_VER {
      if self.board[i][j].1 {
        cnt += 1;
      }
      ver_win = cnt == DIM_VER;
      if ver_win {
        break;
      }
    }

    hor_win || ver_win
  }

  fn score(&self, latest: String) -> i32 {
    let mut score = 0;
    for row in self.board.iter() {
      for col in row {
        if col.1 == false {
          score += col.0.parse::<i32>().unwrap();
        }
      }
    }

    score * latest.parse::<i32>().unwrap()
  }

  fn from_lines(lines: &[String], indx: i32) -> Self {
    let mut b: [[(String, bool);DIM_HOR];DIM_VER] = Default::default();

    for (i, line) in lines.iter().enumerate() {
      for (j, c) in line.split_whitespace().enumerate() {
        b[i][j] = (c.to_string(), false);
      }
    }

    Self {
      board: b,
      indx
    }
  }
}

fn parse_lines(lines: &Vec<String>) -> (Vec<String>, Vec<Board>) {
  let inputs: Vec<String> = lines[0].split(",").map(|s| s.to_string()).collect();

  let mut boards = Vec::<Board>::new();
  let mut cnt = 2;
  let mut indx = 0;

  loop {
    boards.push(Board::from_lines(&lines[cnt..cnt+DIM_VER], indx));
    indx += 1;
    cnt += DIM_VER + 1; // 1 blank line

    if cnt >= lines.len() {
      break;
    }
  }

  (inputs, boards)
}

fn run_one_star(inputs: &Vec<String>, boards: &mut Vec<Board>) -> i32 {
  let mut found = false;
  for input in inputs {
    for board in &mut *boards {
      if board.new_draw(input.clone()) {
        return board.score(input.clone());
      }
    }
  }

  -1
}

fn run_two_stars(inputs: &Vec<String>, boards: &mut Vec<Board>) -> i32 {
  let mut scores = Vec::new();
  for number in inputs {
    for board in &mut *boards {
      if scores.iter().map(|s: &(i32, i32)| s.1).collect::<Vec<i32>>().contains(&board.indx) {
        continue;
      }
      if board.new_draw(number.clone()) {
        let score = board.score(number.clone());
        scores.push((score, board.indx));
      }
    }
  }

  println!("{:?}", scores);
  1
}