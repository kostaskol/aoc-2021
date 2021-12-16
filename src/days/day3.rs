use crate::utils::input::read_file;

type Binset = Vec<Bin>;

pub fn run(extra: bool, test: bool) -> String {
    let lines = read_file("3", test);
    let binary = Bin::from_input(lines);

    format!(
        "{}",
        match extra {
            false => p1::run(binary),
            true => p2::run(binary),
        }
    )
}

#[derive(Debug, Clone)]
pub struct Bin {
    bits: Vec<bool>,
}

impl Bin {
    fn from_input(input: Vec<String>) -> Binset {
        let mut bitset = Vec::new();
        for line in input {
            let digits = line.chars().map(|s| s.to_string());
            bitset.push(Self {
                bits: digits.map(|d| d == "1").collect(),
            });
        }

        bitset
    }

    fn bit_at(&self, indx: usize) -> bool {
        self.bits[indx]
    }

    fn to_decimal(&self) -> isize {
        let res = self
            .bits
            .iter()
            .map(|&b| {
                match b {
                    true => "1",
                    false => "0",
                }
                .to_string()
            })
            .collect::<Vec<String>>()
            .join("");

        isize::from_str_radix(&res, 2).unwrap()
    }

    fn len(&self) -> usize {
        self.bits.len()
    }
}

/*
 This is a warning because Vec<Bin> is aliased as Binset.
 Because we want Binset to be returned from a function as well,
 it can't be an alias to [Bin] (as suggested by clippy). Perhaps a
 refactoring to make Binset an actual struct would make sense here
*/
#[allow(clippy::ptr_arg)]
fn count_bits(binary: &Binset, indx: usize) -> (usize, usize) {
    let mut ones = 0;
    let mut zeroes = 0;
    for row in binary {
        if row.bit_at(indx) {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }
    (ones, zeroes)
}

mod p1 {
    use super::{Bin, Binset};

    pub fn run(binset: Binset) -> isize {
        let binary_size = binset[0].len();
        let mut cnt: Vec<i32> = vec![0; binary_size];

        for bin in binset.iter() {
            for (i, bit) in bin.bits.iter().enumerate() {
                if *bit {
                    cnt[i] += 1;
                }
            }
        }

        let mut gamma = vec![false; binary_size];
        let mut epsilon = vec![false; binary_size];

        for (i, el) in cnt.iter().enumerate() {
            let cond = *el > (binset.len() / 2) as i32;
            gamma[i] = cond;
            epsilon[i] = !cond;
        }

        let gamma_dec = Bin { bits: gamma }.to_decimal();
        let epsilon_dec = Bin { bits: epsilon }.to_decimal();
        gamma_dec * epsilon_dec
    }
}

mod p2 {
    use super::{count_bits, Bin, Binset};

    pub fn run(bitset: Binset) -> isize {
        solve_1s(&bitset) * solve_0s(&bitset)
    }

    /*
      This is a warning because Vec<Bin> is aliased as Binset.
      Because we want Binset to be returned from a function as well,
      it can't be an alias to [Bin] (as suggested by clippy). Perhaps a
      refactoring to make Binset an actual struct would make sense here
    */
    #[allow(clippy::ptr_arg)]
    fn solve_0s(binary: &Binset) -> isize {
        let byte_size = binary[0].len();
        let mut prev_binary = binary.clone();
        let mut answer = Vec::new();

        for i in 0..byte_size {
            let mut next_binary = Vec::new();
            let (ones, zeroes) = count_bits(&prev_binary, i);
            let min_bit = ones < zeroes;

            for byte in prev_binary {
                if byte.bit_at(i) == min_bit {
                    next_binary.push(byte.clone());
                }
            }
            prev_binary = next_binary.clone();

            if i == byte_size - 1 || prev_binary.len() == 1 {
                answer = next_binary;
                break;
            }
        }

        if answer.len() != 1 {
            panic!("Invalid binary: {:?}", answer);
        }

        answer[0].to_decimal()
    }

    /*
      This is a warning because Vec<Bin> is aliased as Binset.
      Because we want Binset to be returned from a function as well,
      it can't be an alias to [Bin] (as suggested by clippy). Perhaps a
      refactoring to make Binset an actual struct would make sense here
    */
    #[allow(clippy::ptr_arg)]
    fn solve_1s(binary: &Binset) -> isize {
        let byte_size = binary[0].len();
        let mut prev_binary = binary.clone();
        let mut answer = Vec::new();
        for i in 0..byte_size {
            let mut next_binary: Vec<Bin> = Vec::new();
            let (ones, zeroes) = count_bits(&prev_binary, i);
            let max_bit = ones >= zeroes;

            for byte in prev_binary {
                if byte.bit_at(i) == max_bit {
                    next_binary.push(byte.clone());
                }
            }
            prev_binary = next_binary.clone();

            if i == byte_size - 1 {
                answer = next_binary;
            }
        }

        if answer.len() != 1 {
            panic!("Invalid binary: {:?}", answer);
        }

        answer[0].to_decimal()
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_p1() {
        assert_eq!(run(false, true), "198");
    }

    #[test]
    fn test_p2() {
        assert_eq!(run(true, true), "230");
    }
}
