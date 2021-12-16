use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use crate::utils::parsing::read_file;
use crate::utils::board::{Board, Point};

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("15", test);
  let board: Board<u32> = Board::from(parse_lines(lines));

  format!("{}", 
    match extra {
      false => p1::run(board),
      true => p2::run(board)
    }
  )
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<u32>> {
  let mut ret: Vec<Vec<u32>> = Vec::new();
  lines.into_iter().for_each(|line| {
    ret.push(
      line.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
    );
  });

  ret
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Element {
  p: Point,
  v: u32
}

impl Ord for Element {
  fn cmp(&self, other: &Self) -> Ordering {
    other.v.cmp(&self.v)
  }
}

impl PartialOrd for Element {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn dijkstra(board: Board<u32>) -> u32 {
  let mut visited: Board<u32> = Board::with_defaults(u32::MAX, board.dim());
  let mut to_visit: BinaryHeap<Element> = BinaryHeap::new();
  let mut parents: HashMap<Point, Point> = HashMap::new();

  to_visit.push(Element { p: (0, 0), v: *board.get((0, 0)).unwrap() });

  while let Some(e) = to_visit.pop() {
    let neighbours = board.get_neighbours(&e.p, false);
    *visited.get_mut(e.p).unwrap() = e.v;
    for n in neighbours {
      let curr_score = e.v + *board.get(n).unwrap();
      // If not in visited
      let visited_neighbour_score = visited.get_mut(n).unwrap();
      let &neighbour_score = board.get(n).unwrap();
      let cumulative_score = curr_score + neighbour_score;
      if *visited_neighbour_score == u32::MAX {
        to_visit.push(Element { p: n, v: cumulative_score});

        parents.insert(n, e.p);
        *visited.get_mut(n).unwrap() = neighbour_score;
      } else if cumulative_score < *visited_neighbour_score {
        *visited_neighbour_score = cumulative_score;
      }
    }
  }

  let dim = board.dim();
  let mut curr = (dim.0 - 1, dim.1 - 1);
  let mut sum = *board.get(curr).unwrap();
  while let Some(p) = parents.get(&curr) {
    if p == &(0, 0) {
      break;
    }
    sum += *board.get(*p).unwrap();
    curr = *p;
  }

  sum
}

mod p1 {
  use crate::utils::board::Board;
  use super::dijkstra;

  pub fn run(board: Board<u32>) -> u32 {
    dijkstra(board)
  }
}

mod p2 {
  use crate::utils::board::Board;
  use super::dijkstra;

  pub fn run(board: Board<u32>) -> u32 {
    dijkstra(enlarge(&board))
  }

  // Enlarge the board 5 times in each direction (25 times overall)
  fn enlarge(board: &Board<u32>) -> Board<u32> {
    let dim = board.dim();
    let mut new_boards: Vec<Board<u32>> = vec![Board::from(board.expose().clone())];

    /* We initially get all possible variants of the board.
     * Since copying the board to the left or the bottom is the (increased by 1),
     * while copying diagonally increases values by 2
     * e.g.
     * 1 1 ...
     * 1 2 2
     *.. 2 3
     *
     * We can only ever have 9 distinct versions of the board (1 original + 8 new).
     */
    for inc in 1..9 {
      let mut new_board = Board::<u32>::with_defaults(0, dim);
      for i in 0..dim.0 {
        for j in 0..dim.1 {
          let p = (i, j);
          let old_val = board.get(p).unwrap();
          let mut new_val = (old_val + inc) % 9;
          if new_val == 0 {
            new_val = 9;
          }
          *new_board.get_mut(p).unwrap() = new_val;
        }
      }
      new_boards.push(new_board);
    }

    arrange_boards(new_boards)
  }

  /* Given a vector of the 9 distinct boards, arranges and flattens them into one
   * The arrangement is as follows: (each board is denoted by its index in the vector)
   * 0 1 2 3 4
   * 1 2 3 4 5
   * 2 3 4 5 6
   * 3 4 5 6 7
   * 4 5 6 7 8
   */
  fn arrange_boards(boards: Vec<Board<u32>>) -> Board<u32> {
     let mut flat_board: Vec<Vec<u32>> = Vec::new();

     let dim = boards[0].dim();
     // We'll be copying the boards 5 times downwards
     for downwards_index in 0..5 {
      for board_row in 0..dim.0 {
        flat_board.push(Vec::<u32>::new());
        // This is indeed only used to loop
        // over the boards, however the order, while
        // linear, is somewhat erradic. It would be
        // interesting to check how this would be done with
        // iterators. Maybe some other time
        #[allow(clippy::needless_range_loop)]
        // Each new row of boards is one time larger than the previous one
        // e.g.
        // 1 2 3 4 5
        // 2 3 4 5 6
        // ... 
        // As such, we need increase the 5 indices of the boards we use by `board_row`
        for board_index in downwards_index..(downwards_index + 5) {
          let mut acc_row = boards[board_index].get_row(board_row).unwrap().clone();
          flat_board.last_mut().unwrap().append(&mut acc_row);
        }
      }
     }

     Board::from(flat_board)
  }
}

#[cfg(test)]
mod tests {
  use super::run;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "40")
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "315")
  }
}