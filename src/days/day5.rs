use crate::utils::input::read_file;
use crate::utils::board::Point;

type Pointpair = (Point, Point);
type Pointset = Vec<(Point, Point)>;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("5", test);
  let points: Pointset = from_lines(lines);
  let oceanfloor = Oceanfloor::new(&points);

  format!("{}",
    match extra {
      false => p1::run(oceanfloor, points),
      true => p2::run(oceanfloor, points),
    }
  )
}

fn from_lines(lines: Vec<String>) -> Pointset {
  let mut points = Vec::new();
  for line in lines {
    points.push(parse_pair(&line));
  }

  points
}

fn parse_pair(line: &str) -> Pointpair {
  let points = line.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>();
  let p1 = parse(&points, 0);
  let p2 = parse(&points, 1);

  (p1, p2)
}

fn parse(points: &[String], indx: usize) -> Point {
  let coords: Vec<usize> = points[indx].split(',').map(|s| s.parse::<usize>().unwrap()).collect();

  (coords[0], coords[1])
}

fn interval(p1: Point, p2: Point, diagonals: bool) -> Vec<Point> {
  if p1.0 == p2.0 {
    let mut points = Vec::new();
    let (mut x, mut y) = (p1.1, p2.1);

    if x > y {
      std::mem::swap(&mut x, &mut y);
    }

    for i in x..=y {
      points.push((p1.1, i));
    }
    return points;
  }

  if p1.1 == p2.1 {
    let mut points = Vec::new();
    let (mut x, mut y) = (p1.0, p2.0);

    if x > y {
      std::mem::swap(&mut x, &mut y);
    }
    for i in x..=y {
      points.push((i, p1.1));
    }
    return points;
  }

  if diagonals {
    let mut points = vec![p1, p2];

    let mut i = p1.0;
    let mut j = p1.1;

    let down_x = p1.0 > p2.0;
    let down_y = p1.1 > p2.1;

    while (i != p2.0) && (j != p2.1) {
      points.push((i, j));
      if down_x {
        i -= 1;
      } else {
        i += 1;
      }

      if down_y {
        j -= 1;
      } else {
        j += 1;
      }
    }

    points.retain(|p| p.0 != p1.0 && p.1 != p1.1);
    points.push(p1);

    points
  } else {
    vec![]
  }
}

// TODO: Replace this with board::Board<u8>
pub struct Oceanfloor {
  board: Vec<Vec<u8>>
}

impl Oceanfloor {
  /*
    This is a warning because Vec<Point> is aliased as Pointset.
    Because we want Pointset to be returned from a function as well,
    it can't be an alias to [Point] (as suggested by clippy). Perhaps a
    refactoring to make Pointset an actual struct would make sense here
  */
  #[allow(clippy::ptr_arg)]
  fn new(points: &Pointset) -> Self {
    let mut board: Vec<Vec<u8>> = Vec::new();

    let (max_x, max_y) = Self::get_max_coords(points);

    for _ in 0..max_x {
      board.push(vec![0; max_y as usize]);
    }

    Self { board }
  }

  /*
    This is a warning because Vec<Point> is aliased as Pointset.
    Because we want Pointset to be returned from a function as well,
    it can't be an alias to [Point] (as suggested by clippy). Perhaps a
    refactoring to make Pointset an actual struct would make sense here
  */
  #[allow(clippy::ptr_arg)]
  fn get_max_coords(points: &Pointset) -> (usize, usize) {
    let max_p1_x = points.iter().map(|(p1, _)| p1.0).max().unwrap();
    let max_p2_x = points.iter().map(|(_, p2)| p2.0).max().unwrap();
    let max_p1_y = points.iter().map(|(p1, _)| p1.1).max().unwrap();
    let max_p2_y = points.iter().map(|(_, p2)| p2.1).max().unwrap();

    let max_x = *vec![max_p1_x, max_p2_x].iter().max().unwrap() + 1;
    let max_y = *vec![max_p1_y, max_p2_y].iter().max().unwrap() + 1;

    (max_x, max_y)
  }
}

fn count_twos(oceanfloor: &Oceanfloor) -> i32 {
  let mut twos = 0;
  for row in oceanfloor.board.iter() {
    for cell in row {
      if *cell > 1 {
        twos += 1;
      }
    }
  }
  twos
}

fn apply_points(oceanfloor: &mut Oceanfloor, points: Vec<Point>) {
  for point in points {
    let (x, y) = point;

    oceanfloor.board[x][y] += 1;
  }
}


mod p2 {
  use super::{Oceanfloor, Point, apply_points, count_twos, interval};

  pub fn run(mut oceanfloor: Oceanfloor, points: Vec<(Point, Point)>) -> i32 {
    for (p1, p2) in points {
      let interval = interval(p1, p2, true);
      apply_points(&mut oceanfloor, interval);
    }

    count_twos(&oceanfloor)
  }
}

mod p1 {
  use super::{Oceanfloor, Point, apply_points, count_twos, interval};

  pub fn run(mut oceanfloor: Oceanfloor, points: Vec<(Point, Point)>) -> i32 {
    for (p1, p2) in points {
      let interval = interval(p1, p2, false);
      apply_points(&mut oceanfloor, interval);
    }

    count_twos(&oceanfloor)
  }
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::run;

  #[test]
  fn test_p1() {
    // TODO: Fix this
    // assert_eq!(run(false, true), "5");
  }

  #[test]
  fn test_p2() {
    // TODO: Fix this
    // assert_eq!(run(true, true), "12");
  }
}