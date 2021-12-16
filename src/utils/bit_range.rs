use std::num::ParseIntError;

// Returns a tuple containing the version and type_id
pub fn header(mut packet: &mut BitRange) -> (isize, isize) {
  (version(&mut packet), type_id(&mut packet))
}

pub fn length_type_id(packet: &mut BitRange) -> isize {
  packet.slice(1).unwrap().as_isize().unwrap()
}

fn version(packet: &mut BitRange) -> isize {
  packet.slice(3).unwrap().as_isize().unwrap()
}

fn type_id(packet: &mut BitRange) -> isize {
  packet.slice(3).unwrap().as_isize().unwrap()
}

#[derive(Debug)]
pub struct BitRange {
  bitstring: Vec<char>,
  pos: usize
}

impl BitRange {
  pub fn from_hex(hex: &str) -> Self {
    let mut bitstring: Vec<char> = Vec::new();

    for hexdigit in hex.chars() {
      bitstring.append(&mut Self::lookup(hexdigit));
    }

    Self { bitstring, pos: 0 }
  }

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

  pub fn from(bits: &[char]) -> Self {
    Self { bitstring: bits.to_vec(), pos: 0 }
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
    self.bitstring.get(start..end).map(|slice| BitRange::from(slice))
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