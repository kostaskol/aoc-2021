use crate::utils;

type Pointpair = (Point, Point);
type Pointset = Vec<(Point, Point)>;

pub fn run(extra: bool) -> String {
  let lines = utils::read_lines("inputs/5.txt");
  let points: Pointset = Point::from_lines(&lines);
  let mut oceanfloor = Oceanfloor::new(&points);

  format!("{}",
    match extra {
      true => run_two_stars(&mut oceanfloor, &points),
      false => run_one_star(&mut oceanfloor, &points),
    }
  )
}

#[derive(Debug, Copy, Clone)]
struct Point {
  x: i32,
  y: i32
}

impl Point {
  fn from_lines(lines: &Vec<String>) -> Pointset {
    let mut points = Vec::new();
    for line in lines {
      points.push(Self::parse_pair(&line));
    }

    points
  }

  fn parse_pair(line: &str) -> Pointpair {
    let points = line.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>();
    let p1 = Self::parse(&points, 0);
    let p2 = Self::parse(&points, 1);

    (p1, p2)
  }

  fn parse(points: &Vec<String>, indx: usize) -> Self {
    let coords = points[indx].split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    Self {
      y: coords[0],
      x: coords[1]
    }
  }

  fn interval(p1: &Self, p2: &Self, diagonals: bool) -> Vec<Self> {
    if p1.x == p2.x {
      let mut points = Vec::new();
      let (mut x, mut y) = (p1.y, p2.y);

      if x > y {
        let c = x;
        x = y;
        y = c;
      }

      for i in x..=y {
        points.push( Self { x: p1.x, y: i });
      }
      return points;
    }

    if p1.y == p2.y {
      let mut points = Vec::new();
      let (mut x, mut y) = (p1.x, p2.x);

      if x > y {
        let c = x;
        x = y;
        y = c;
      }
      for i in x..=y {
        points.push(Self { x: i, y: p1.y });
      }
      return points;
    }

    if diagonals {
      /*
      (0, 8) -> (8, 0)
      ===
      [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8)]

      (2, 8) -> (8, 2)
      ===
      [(2, 8), (3, 7), (4, 6), (5, 5), (6, 4), (7, 3), (8, 2)]

      (0, 2) -> (4, 6)
      (0, 2), (1, 3), (2, 4), (3, 5), (4, 6)

      0 1 2 3 4 5 6 7 8
    0 . . . . . . . . .
    1 . . . . . . . . .
    2 . . . . . . . . 1
    3 . . . . . . . . .
    4 . . . . . . . . .
    5 . . . . . 1 . . .
    6 . . . . . . . . .
    7 . . . . . . . . .
    8 . . . . . . . . .

    (1, 0) -> (4, 3)
    ===
    (1, 0), (2, 1), (3, 2), (4, 3)
       */
      let mut points = vec![*p1, *p2];

      let mut i = p1.x;
      let mut j = p1.y;

      let down_x = p1.x > p2.x;
      let down_y = p1.y > p2.y;

      while (i != p2.x) && (j != p2.y) {
        points.push(Self { x: i, y: j });
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

      points.retain(|p| p.x != p1.x && p.y != p1.y);
      points.push(*p1);

      return points;
    } else {
      vec![]
    }
  }
}

struct Oceanfloor {
  board: Vec<Vec<u8>>
}

impl Oceanfloor {
  fn new(points: &Pointset) -> Self {
    let mut board: Vec<Vec<u8>> = Vec::new();

    let (max_x, max_y) = Self::get_max_coords(&points);

    for _ in 0..max_x {
      board.push(vec![0; max_y as usize]);
    }

    Self { board }
  }

  fn get_max_coords(points: &Pointset) -> (i32, i32) {
    let max_p1_x = points.iter().map(|(p1, _)| p1.x).max().unwrap();
    let max_p2_x = points.iter().map(|(_, p2)| p2.x).max().unwrap();
    let max_p1_y = points.iter().map(|(p1, _)| p1.y).max().unwrap();
    let max_p2_y = points.iter().map(|(_, p2)| p2.y).max().unwrap();

    let max_x = *vec![max_p1_x, max_p2_x].iter().max().unwrap() + 1;
    let max_y = *vec![max_p1_y, max_p2_y].iter().max().unwrap() + 1;

    (max_x, max_y)
  }

  fn print(&self) {
    for row in &self.board {
      println!("{:?}", row);
    }
  }
}

fn run_two_stars(oceanfloor: &mut Oceanfloor, points: &Vec<(Point, Point)>) -> i32 {
  for (p1, p2) in points {
    let interval = Point::interval(p1, p2, true);
    apply_points(oceanfloor, interval);
  }

  count_twos(oceanfloor)
}

fn run_one_star(oceanfloor: &mut Oceanfloor, points: &Vec<(Point, Point)>) -> i32 {
  for (p1, p2) in points {
    let interval = Point::interval(p1, p2, false);
    apply_points(oceanfloor, interval);
  }

  count_twos(oceanfloor)
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
    let Point {x, y} = point;

    oceanfloor.board[x as usize][y as usize] += 1;
  }
}

