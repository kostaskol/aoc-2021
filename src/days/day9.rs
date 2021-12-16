use crate::utils::input::read_file;
use crate::utils::board::Board;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("9", test);
  let board = parse_board(lines);

  format!("{}",
    match extra {
      false => p1::run(board),
      true => p2::run(board)
    }
  )
}

fn parse_board(lines: Vec<String>) -> Board<(u8, bool)> {
  let mut board = vec![];

  for line in lines {
    let mut row = vec![];

    for c in line.chars() {
      row.push((c.to_digit(10).unwrap() as u8, false));
    }

    board.push(row);
  }

  Board::from(board)
}

fn mark_low_points(board: &mut Board<(u8, bool)>) {
  let dim = board.dim();
  for i in 0..dim.0 {
    for j in 0..dim.1 {
      let p = (i, j);
      let neighbours = board.get_neighbours(&p, false);
      let curr = board.get(p).unwrap();
      let mut lower_neighbour = false;
      for n in neighbours {
        if board.get(n).unwrap().0 <= curr.0 {
          lower_neighbour = true;
        }
      }
      board.get_mut(p).unwrap().1 = lower_neighbour;
    }
  }
}

mod p1 {
  use crate::utils::board::Board;
  use super::mark_low_points;

  pub fn run(mut board: Board<(u8, bool)>) -> i32 {
    mark_low_points(&mut board);

    let mut risk_level: i32 = 0;
    let dim = board.dim();
    for i in 0..dim.0 {
      for j in 0..dim.1 {
        let cell = board.get((i, j)).unwrap();
        if cell.1 {
          risk_level += (cell.0 as i32) + 1;
        }
      }
    }
    risk_level
  }
}

mod p2 {
  use std::collections::{HashSet, VecDeque};
  use crate::utils::board::{Board, Point};
  use super::mark_low_points;

  pub fn run(mut board: Board<(u8, bool)>) -> i32 {
    mark_low_points(&mut board);
    let low_points = find_low_points(&board);
    let mut res = vec![Vec::<Point>::new(); low_points.len()];

    for (i, low_point) in low_points.iter().enumerate() {
      let mut visited: HashSet<Point> = HashSet::new();
      let mut to_visit: VecDeque<Point> = VecDeque::new();

      // Initialize with first neighbours
      for neighb in get_to_visit(&board, low_point, &visited) {
        to_visit.push_front(neighb);
      }
      to_visit.push_front(*low_point);
      while let Some(curr) = to_visit.pop_front() {
        if visited.contains(&curr) {
          continue;
        }
        visited.insert(curr);
        for neighb in get_to_visit(&board, &curr, &visited) {
          to_visit.push_front(neighb);
        }
      }
      res[i] = visited.into_iter().collect();
    }

    let mut lengths = res.iter().map(|e| e.len()).collect::<Vec<usize>>();
    lengths.sort_unstable();
    let lengths_len = lengths.len();
    lengths[lengths_len - 3..lengths_len].iter().map(|&e| e as i32).product()
  }

  fn find_low_points(board: &Board<(u8, bool)>) -> Vec<(usize, usize)> {
    let mut low_points: Vec<Point> = Vec::new();
    let dim = board.dim();
    for i in 0..dim.0 {
      for j in 0..dim.1 {
        let cell = board.get((i, j)).unwrap();
        if cell.1 {
          low_points.push((i, j));
        }
      }
    }

    low_points
  }

  fn get_to_visit(
    board: &Board<(u8, bool)>,
    p: &Point,
    visited: &HashSet<Point>
  ) -> Vec<Point> {
    let mut to_visit = Vec::new();
    let neighb = board.get_neighbours(p, false);
    let low_point_val = board.get(*p).unwrap().0;
    for n in neighb {
      let neighb_val = board.get(n).unwrap().0;
      if !visited.contains(&n)
          && neighb_val != 9
          && neighb_val > low_point_val {
        to_visit.push(n);
      }
    }
    to_visit
  }
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::run;

  #[test]
  fn test_p1() {
    // TODO: Fix the module + tests
    // assert_eq!(run(false, true), "15");
  }

  #[test]
  fn test_p2() {
    // assert_eq!(run(true, true), "1134");
  }
}