use crate::utils;

type Board = Vec<Vec<Octopus>>;

struct Octopus {
  energy: u8,
  flashed: bool
}

#[derive(Debug)]
enum RunType {
  Increase,
  Flash,
  Reset
}

impl Octopus {
  fn new(energy: u8) -> Octopus {
    Octopus {
      energy,
      flashed: false
    }
  }

  fn flashed(&mut self) {
    self.flashed = true;
  }

  fn reset(&mut self) {
    self.flashed = false;
    if self.energy > 9 {
      self.energy = 0;
    }
  }

  fn gain_energy(&mut self) {
    self.energy += 1;
  }

  fn is_ready(&self) -> bool {
    self.energy > 9 && !self.flashed
  }
}

fn next_type(runtype: &mut RunType) {
  match runtype {
    RunType::Increase => *runtype = RunType::Flash,
    RunType::Flash => *runtype = RunType::Reset,
    RunType::Reset => *runtype = RunType::Increase
  }
}

pub fn run(extra: bool) -> String {
  let lines = utils::read_lines("inputs/11.txt");
  let board = parse_board(lines);

  format!("{}",
    match extra {
      false => run_one_star(board),
      true => run_two_stars(board)
    }
  )
}

fn run_one_star(mut board: Board) -> i32 {
  let mut total_flashes = 0;

  for _ in 0..100 {
    let mut runtype = RunType::Increase;
    for _ in 0..3 {
      for i in 0..board.len() {
        for j in 0..board[i].len() {
          match runtype {
            RunType::Increase => {
              board[i][j].gain_energy()
            },
            RunType::Flash => total_flashes += flash(&mut board, i, j),
            RunType::Reset => reset(&mut board)
          }
        }
      }
      next_type(&mut runtype);
    }
  }

  total_flashes
}

fn run_two_stars(mut board: Board) -> i32 {
  let mut runtype = RunType::Increase;
  let mut step = 0;
  loop {
    step += 1;
    for _ in 0..3 {
      for i in 0..board.len() {
        for j in 0..board[i].len() {
          match runtype {
            RunType::Increase => board[i][j].gain_energy(),
            RunType::Flash => {
              if flash(&mut board, i, j) == 100 {
                return step;
              }
            },
            RunType::Reset => reset(&mut board)
          }
        }
      }
      next_type(&mut runtype);
    }
  }
}

fn flash(board: &mut Board, i: usize, j: usize) -> i32 {
  if !board[i][j].is_ready() {
    return 0;
  }
  board[i][j].flashed();

  let mut total_flashes = 1;
  let neighbours = get_neighbours(board, i, j);

  for n in neighbours {
    board[n.0][n.1].gain_energy();
    total_flashes += flash(board, n.0, n.1);
  }

  total_flashes
}

fn reset(board: &mut Board) {
  for i in 0..board.len() {
    for j in 0..board[i].len() {
      board[i][j].reset();
    }
  }
}

fn get_neighbours(board: &Board, i: usize, j: usize) -> Vec<(usize, usize)> {
  let mut neighbours = vec![];

  if i > 0 {
    neighbours.push((i - 1, j));

    if j > 0 {
      neighbours.push((i - 1, j - 1));
    }

    if j < board[i].len() - 1 {
      neighbours.push((i - 1, j + 1));
    }
  }

  if i < board.len() - 1 {
    neighbours.push((i + 1, j));

    if j < board[i].len() - 1 {
      neighbours.push((i + 1, j + 1));
    }

    if j > 0 {
      neighbours.push((i + 1, j - 1));
    }
  }

  if j > 0 {
    neighbours.push((i, j - 1));
  }

  if j < board[i].len() - 1 {
    neighbours.push((i, j + 1));
  }

  neighbours
}

fn parse_board(lines: Vec<String>) -> Board {
  let mut board: Board = Vec::new();
  for line in lines {
    let mut row: Vec<Octopus> = Vec::new();
    for c in line.chars() {
      row.push(Octopus::new(c.to_digit(10).unwrap() as u8));
    }
    board.push(row);
  }
  board
}