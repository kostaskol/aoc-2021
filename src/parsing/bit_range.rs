use std::num::ParseIntError;


#[derive(Debug)]
pub struct BitRange {
  bitstring: Vec<char>,
  pos: usize
}

impl BitRange {
  pub fn slice(&mut self, length: usize) -> Option<BitRange> {
    let old_pos = self.pos;
    self.pos = old_pos + length;
    self.range(old_pos, old_pos + length)
  }

  pub fn take(&mut self, length: usize) -> Vec<char> {
    self.slice(length).unwrap().bitstring
  }

  // Non inclusive end
  pub fn next(&mut self) -> char {
    self.take(1)[0]
  }

  pub fn as_str(&self) -> String {
    self.bitstring.iter().collect()
  }

  pub fn as_isize(&self) -> Result<isize, ParseIntError> {
    isize::from_str_radix(&self.as_str(), 2)
  }

  pub fn pos(&self) -> usize {
    self.pos
  }

  fn range(&self, start: usize, end: usize) -> Option<BitRange> {
    self.bitstring.get(start..end).map(BitRange::from)
  }

  fn lookup(hex: char) -> Vec<char> {
    match hex {
      '0' => "0000",
      '1' => "0001",
      '2' => "0010",
      '3' => "0011",
      '4' => "0100",
      '5' => "0101",
      '6' => "0110",
      '7' => "0111",
      '8' => "1000",
      '9' => "1001",
      'A' => "1010",
      'B' => "1011",
      'C' => "1100",
      'D' => "1101",
      'E' => "1110",
      'F' => "1111",
      x => panic!("Unrecognized hex digit: {}", x)
    }.chars().collect()
  }
}

impl From<&str> for BitRange {
  fn from(hex: &str) -> Self {
    let mut bitstring: Vec<char> = Vec::new();

    for hexdigit in hex.chars() {
      bitstring.append(&mut Self::lookup(hexdigit));
    }

    Self { bitstring, pos: 0 }
  }
}

impl From<&[char]> for BitRange {
  fn from(bits: &[char]) -> Self {
    Self { bitstring: bits.to_vec(), pos: 0 }
  }
}
