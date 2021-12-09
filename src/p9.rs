use std::collections::{HashSet, VecDeque};
use crate::utils;

type Board = Vec<Vec<(u8, bool)>>;
type Point = (usize, usize);

pub fn run(extra: bool) -> String {
  let lines = utils::read_lines("inputs/9.txt");
  let board = parse_board(lines);

  format!("{}",
    match extra {
      false => run_one_star(board),
      true => run_two_stars(board)
    }
  )
}

fn run_one_star(mut board: Board) -> i32 {
  mark_low_points(&mut board);

  let mut risk_level: i32 = 0;
  for row in board {
    for cell in row {
      if cell.1 {
        risk_level += (cell.0 as i32) + 1;
      }
    }
  }
  risk_level
}

fn run_two_stars(mut board: Board) -> i32 {
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
  lengths.sort();
  let lengths_len = lengths.len();
  lengths[lengths_len - 3..lengths_len].iter().map(|&e| e as i32).product()
}

fn get_to_visit(board: &Board, p: &Point, visited: &HashSet<Point>) -> Vec<Point> {
  let mut to_visit = Vec::new();
  let neighb = neighbours(&board, p.0, p.1);
  let low_point_val = board[p.0][p.1].0;
  for n in neighb {
    let neighb_val = board[n.0][n.1].0;
    if !visited.contains(&n)
        && neighb_val != 9
        && neighb_val > low_point_val {
      to_visit.push(n);
    }
  }
  to_visit
}

fn find_low_points(board: &Board) -> Vec<(usize, usize)> {
  let mut low_points = Vec::new();
  for (i, row) in board.iter().enumerate() {
    for (j, cell) in row.iter().enumerate() {
      if cell.1 {
        low_points.push((i, j));
      }
    }
  }
  low_points
}

fn mark_low_points(board: &mut Board) {
  for i in 0..board.len() {
    for j in 0..board[i].len() {
      let neighbours = neighbours(&board, i, j);
      let curr = board[i][j];
      let mut lower_neighbour = false;
      for n in neighbours.iter() {
        if board[n.0][n.1].0 <= curr.0 {
          lower_neighbour = true;
        }
      }
      board[i][j].1 = !lower_neighbour;
    }
  }
}

fn neighbours(board: &Board, i: usize, j: usize) -> Vec<Point> {
  let mut neighbours = vec![];

  if i > 0 {
    neighbours.push((i - 1, j));
  }

  if i < board.len() - 1 {
    neighbours.push((i + 1, j));
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
  let mut board = vec![];

  for line in lines {
    let mut row = vec![];

    for c in line.chars() {
      row.push((c.to_digit(10).unwrap() as u8, false));
    }

    board.push(row);
  }

  board
}