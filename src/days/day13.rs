use crate::utils::board::Point;
use crate::utils::input::read_file;
use regex::Regex;
use std::collections::HashSet;

pub fn run(extra: bool, test: bool) -> String {
    let lines = read_file("13", test);
    let (points, folds) = parse_input(lines);

    format!(
        "{}",
        match extra {
            false => p1::run(points, folds),
            true => p2::run(points, folds),
        }
    )
}

#[derive(Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}

/*
   0 1 2 3 4 5 6 7 8 9 10
 0 . . . # . . # . . # .
 1 . . . . # . . . . . .
 2 . . . . . . . . . . .
 3 # . . . . . . . . . .
 4 . . . # . . . . # . #
 5 . . . . . . . . . . .
 6 . . . . . . . . . . .
 7 ---------------------
 8 . . . . . . . . . . .
 9 . . . . . . . . . . .
10 . # . . . . # . # # .
11 . . . . # . . . . . .
12 . . . . . . # . . . #
13 # . . . . . . . . . .
14 # . # . . . . . . . .
 */

impl Fold {
    fn symmetrical(&self, p: &Point) -> Point {
        let mut ret = (0, 0);
        let f = |indx: usize, fold: usize| 2 * fold - indx;
        match self {
            Fold::Y(val) => {
                ret.1 = p.1;
                ret.0 = f(p.0, *val);
            }
            Fold::X(val) => {
                ret.0 = p.0;
                ret.1 = f(p.1, *val);
            }
        }

        ret
    }
}

fn parse_input(lines: Vec<String>) -> (HashSet<Point>, Vec<Fold>) {
    let mut points_section = true;
    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    let regex = Regex::new(r"^.*(?P<axis>.)=(?P<value>\d+)$").unwrap();
    for line in lines {
        if line.is_empty() {
            points_section = false;
            continue;
        }

        if points_section {
            let split: Vec<&str> = line.split(',').collect();
            points.insert((
                split[1].parse::<usize>().unwrap(),
                split[0].parse::<usize>().unwrap(),
            ));
        } else if let Some(x) = regex.captures(&line) {
            let val = x.name("value").unwrap().as_str().parse::<usize>().unwrap();
            let axis = x.name("axis").unwrap().as_str();
            let fold = match axis {
                "x" => Fold::X(val),
                "y" => Fold::Y(val),
                &_ => unreachable!(),
            };

            folds.push(fold)
        }
    }

    (points, folds)
}

mod p1 {
    use super::Fold;
    use crate::utils::board::Point;
    use std::collections::HashSet;

    pub fn run(points: HashSet<Point>, folds: Vec<Fold>) -> i32 {
        let mut cnt = 0;

        let fold = &folds[0];

        for point in points.iter() {
            match fold {
                Fold::X(val) => {
                    if point.1 < *val {
                        cnt += 1;
                    } else {
                        let sym = fold.symmetrical(point);
                        // Only count symmetrical points if they are not contained in the pointset
                        if sym.1 < *val && !points.contains(&sym) {
                            // Since only one fold occurs in p1, we don't need to add the symmetrical
                            // point to the original ones
                            cnt += 1;
                        }
                    }
                }
                Fold::Y(val) => {
                    if point.0 < *val {
                        cnt += 1;
                    } else {
                        let sym = fold.symmetrical(point);
                        // Only count symmetrical points if they are not contained in the pointset
                        if !points.contains(&sym) {
                            // Since only one fold occurs in p1, we don't need to add the symmetrical
                            // point to the original ones
                            cnt += 1;
                        }
                    }
                }
            }
        }
        cnt
    }
}

mod p2 {
    use super::Fold;
    use crate::utils::board::{Board, Point};
    use std::collections::HashSet;

    pub fn run(points: HashSet<Point>, folds: Vec<Fold>) -> i32 {
        let mut curr_points: HashSet<Point> = points;
        for fold in folds {
            let mut next_points: HashSet<Point> = HashSet::new();
            for point in curr_points.iter() {
                match fold {
                    Fold::Y(val) => {
                        if point.0 < val {
                            next_points.insert(*point);
                        } else {
                            let sym = fold.symmetrical(point);
                            next_points.insert(sym);
                        }
                    }
                    Fold::X(val) => {
                        if point.1 < val {
                            next_points.insert(*point);
                        } else {
                            let sym = fold.symmetrical(point);
                            next_points.insert(sym);
                        }
                    }
                }
            }
            curr_points = next_points.clone();
        }

        let board: Board<char> =
            Board::from_points(&(curr_points.into_iter().collect::<Vec<Point>>()), '.', '#');
        for row in board.expose() {
            println!("{}", row.iter().collect::<String>());
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_p1() {
        assert_eq!(run(false, true), "17")
    }

    // Cannot write tests for p2 since it
    // depends on reading the 2D array in the output
}
