use crate::utils::input::read_file;
use std::collections::HashMap;

pub fn run(extra: bool, test: bool) -> String {
    let lines = read_file("14", test);
    let polymer = Polymer::from_string(&lines[0]);
    let rules = parse_rules(&lines[2..]);

    format!(
        "{}",
        match extra {
            false => p1::run(polymer, rules),
            true => p2::run(polymer, rules),
        }
    )
}

type Rules = HashMap<(u8, u8), u8>;

pub struct Polymer {
    segments: HashMap<(u8, u8), usize>,
    first: u8,
    last: u8,
}

impl Polymer {
    pub fn from_string(input: &str) -> Self {
        let mut segments: HashMap<(u8, u8), usize> = HashMap::new();
        let bytes = input.as_bytes();
        bytes
            .windows(2)
            .for_each(|win| *segments.entry((win[0], win[1])).or_insert(0) += 1);
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];

        Polymer {
            segments,
            first,
            last,
        }
    }

    pub fn apply_rules(&mut self, rules: &Rules) {
        let mut new_segments: HashMap<(u8, u8), usize> = HashMap::new();

        for (segment, count) in self.segments.iter() {
            if let Some(&res) = rules.get(segment) {
                *new_segments.entry((segment.0, res)).or_insert(0) += count;
                *new_segments.entry((res, segment.1)).or_insert(0) += count;
            } else {
                // With the specific input provided, this branch never seems
                // to be taken. It's still here for completeness.
                *new_segments.entry(*segment).or_insert(0) += count;
            }
        }

        self.segments = new_segments;
    }

    pub fn score(&self) -> usize {
        let mut scores: HashMap<u8, usize> = HashMap::new();

        // Due to the way we store the segments (pairs of 2),
        // We count every byte twice (e.g. in ['AB BC'] 'B' is counted twice)
        for ((a, b), count) in self.segments.iter() {
            *scores.entry(*a).or_insert(0) += count;
            *scores.entry(*b).or_insert(0) += count;
        }

        // However, we only count the first and last digit once. To be able
        // to simply divide by 2, we must increase these counts by 1.
        *scores.entry(self.last).or_insert(0) += 1;
        *scores.entry(self.first).or_insert(0) += 1;

        let max = scores.values().max().unwrap();
        let min = scores.values().min().unwrap();
        (max - min) / 2
    }
}

fn parse_rules(lines: &[String]) -> Rules {
    let mut pairs: Rules = HashMap::new();

    for line in lines {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let (left, right) = (parts[0].as_bytes(), parts[1].as_bytes()[0]);
        pairs.insert((left[0], left[1]), right);
    }

    pairs
}

mod p1 {
    use super::*;

    pub fn run(mut poly: Polymer, rules: Rules) -> usize {
        for _ in 0..10 {
            poly.apply_rules(&rules);
        }
        poly.score()
    }
}

mod p2 {
    use super::*;

    pub fn run(mut poly: Polymer, rules: Rules) -> usize {
        for _ in 0..40 {
            poly.apply_rules(&rules);
        }
        poly.score()
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_p1() {
        assert_eq!(run(false, true), "1588");
    }

    #[test]
    fn test_p2() {
        assert_eq!(run(true, true), "2188189693529")
    }
}
