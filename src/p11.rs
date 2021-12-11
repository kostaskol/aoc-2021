use crate::utils;
use crate::board::{Point, Board};

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

fn run_one_star(mut board: Board<Octopus>) -> i32 {
  let mut total_flashes = 0;

  for _ in 0..100 {
    let mut runtype = RunType::Increase;
    let board_dim = board.dim();
    for _ in 0..3 {
      for i in 0..board_dim.0 {
        for j in 0..board_dim.1 {
          let p = (i, j);
          match runtype {
            RunType::Increase => {
              board.get_mut(p).unwrap().gain_energy();
            },
            RunType::Flash => total_flashes += flash(&mut board, p),
            RunType::Reset => reset(&mut board)
          }
        }
      }
      next_type(&mut runtype);
    }
  }

  total_flashes
}

fn run_two_stars(mut board: Board<Octopus>) -> i32 {
  let mut runtype = RunType::Increase;
  let mut step = 0;
  let board_dim = board.dim();
  loop {
    step += 1;
    for _ in 0..3 {
      for i in 0..board_dim.0 {
        for j in 0..board_dim.1 {
          let p = (i, j);
          match runtype {
            RunType::Increase => board.get_mut(p).unwrap().gain_energy(),
            RunType::Flash => {
              if flash(&mut board, p) == 100 {
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

fn flash(board: &mut Board<Octopus>, p: Point) -> i32 {
  if !board.get(p).unwrap().is_ready() {
    return 0;
  }
  board.get_mut(p).unwrap().flashed();

  let mut total_flashes = 1;
  let neighbours = board.get_neighbours(p, true);

  for n in neighbours {
    board.get_mut(n).unwrap().gain_energy();
    total_flashes += flash(board, n);
  }

  total_flashes
}

fn reset(board: &mut Board<Octopus>) {
  let dim = board.dim();
  for i in 0..dim.0 {
    for j in 0..dim.1 {
      let p = (i, j);
      board.get_mut(p).unwrap().reset();
    }
  }
}

fn parse_board(lines: Vec<String>) -> Board<Octopus> {
  let mut board: Vec<Vec<Octopus>> = Vec::new();
  for line in lines {
    let mut row: Vec<Octopus> = Vec::new();
    for c in line.chars() {
      row.push(Octopus::new(c.to_digit(10).unwrap() as u8));
    }
    board.push(row);
  }

  Board::<Octopus>::from(board)
}