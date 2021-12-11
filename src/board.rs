pub type Point = (usize, usize);

pub struct Board<T>(Vec<Vec<T>>);

impl<T> Board<T> {
  pub fn from(vec: Vec<Vec<T>>) -> Self {
    Board(vec)
  }

  pub fn dim(&self) -> (usize, usize) {
    (self.0.len(), self.0[0].len())
  }

  pub fn get(&self, point: Point) -> Option<&T> {
    self.0.get(point.0).and_then(|row| row.get(point.1))
  }

  pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
    self.0.get_mut(point.0).and_then(|row| row.get_mut(point.1))
  }

  pub fn expose(&self) -> &Vec<Vec<T>> {
    &self.0
  }

  pub fn get_neighbours(&self, p: Point, diagonals: bool) -> Vec<Point> {
    let mut neighbours = vec![];
    let board = &self.0;

    if p.0 > 0 {
      neighbours.push((p.0 - 1, p.1));

      if diagonals {
        if p.1 > 0 {
          neighbours.push((p.0 - 1, p.1 - 1));
        }

        if p.1 < board[p.0].len() - 1 {
          neighbours.push((p.0 - 1, p.1 + 1));
        }
      }
    }

    if p.0 < board.len() - 1 {
      neighbours.push((p.0 + 1, p.1));

      if diagonals {
        if p.1 < board[p.0].len() - 1 {
          neighbours.push((p.0 + 1, p.1 + 1));
        }

        if p.1 > 0 {
          neighbours.push((p.0 + 1, p.1 - 1));
        }
      }
    }

    if p.1 > 0 {
      neighbours.push((p.0, p.1 - 1));
    }

    if p.1 < board[p.0].len() - 1 {
      neighbours.push((p.0, p.1 + 1));
    }

    neighbours
  }
}

#[cfg(test)]
mod test {
  use super::Board;
  use super::Point;

  #[test]
  fn test_neighbours_no_diagonals() {
    let board = Board::<char>(vec![
      vec!['a', 'b', 'c'],
      vec!['d', 'e', 'f'],
      vec!['g', 'h', 'i'],
    ]);

    let neighbours: Vec<Point> = board.get_neighbours((1, 1), false);
    let expected_neighbours: Vec<Point> = vec![(0, 1), (1, 0), (1, 2), (2, 1)];

    assert_eq!(neighbours.len(), expected_neighbours.len());
    for elem in neighbours {
      assert!(expected_neighbours.contains(&elem));
    }
  }

  #[test]
  fn test_neighbours_diagonals() {
    let board = Board::<char>(vec![
      vec!['a', 'b', 'c'],
      vec!['d', 'e', 'f'],
      vec!['g', 'h', 'i'],
    ]);

    let neighbours: Vec<Point> = board.get_neighbours((1, 1), true);
    let expected_neighbours: Vec<Point> = vec![
      (0, 0), (0, 1), (0, 2),
      (1, 0), (1, 2),
      (2, 0), (2, 1), (2, 2)
    ];

    assert_eq!(neighbours.len(), expected_neighbours.len());
    for elem in neighbours {
      assert!(expected_neighbours.contains(&elem));
    }
  }
}