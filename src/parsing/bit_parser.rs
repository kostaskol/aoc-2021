use crate::parsing::bit_range::BitRange;

pub struct Parser(BitRange);

impl Parser {
  pub fn execute(&mut self) -> isize {
    let (_, type_id) = self.header();

    if type_id == 4 {
      // Literal packet
      self.decode_literal()
    } else {
      self.decode_operator(Operator::from(type_id))
    }
  }

  pub fn versions(&mut self) -> isize {
    let (version, type_id) = self.header();

    let mut version_sum = version;

    if type_id == 4 {
      self.decode_literal();
    } else {
      version_sum += self.decode_operator_version()
    }

    version_sum
  }

  fn decode_operator_version(&mut self) -> isize {
    let length_type_id = self.length_type_id();
    let mut version_sum = 0;

    if length_type_id == 1 {
      let sub_packets = self.0.slice(11).unwrap().as_isize().unwrap();
      for _ in 0..sub_packets {
        version_sum += self.versions();
      }
    } else {
      let num_bits = self.0.slice(15).unwrap().as_isize().unwrap() as usize;
      let target = self.0.pos() + num_bits;
      while self.0.pos() < target {
        version_sum += self.versions();
      }
    }

    version_sum
  }

  fn decode_operator(&mut self, op: Operator) -> isize {
    let length_type_id = self.length_type_id();

    let mut sub_packet_results: Vec<isize> = Vec::new();

    if length_type_id == 1 {
      let sub_packets = self.0.slice(11).unwrap().as_isize().unwrap();
      for _ in 0..sub_packets {
        sub_packet_results.push(self.execute())
      }
    } else {
      let num_bits = self.0.slice(15).unwrap().as_isize().unwrap() as usize;
      let target = self.0.pos() + num_bits;
      while self.0.pos() < target {
        sub_packet_results.push(self.execute());
      }
    }

    match op {
      Operator::Sum => sub_packet_results.iter().sum(),
      Operator::Product => sub_packet_results.iter().product(),
      Operator::Min => *sub_packet_results.iter().min().unwrap(),
      Operator::Max => *sub_packet_results.iter().max().unwrap(),
      Operator::GreaterThan => {
        if sub_packet_results[0] > sub_packet_results[1] {
          1
        } else {
          0
        }
      },
      Operator::LessThan => {
        if sub_packet_results[0] < sub_packet_results[1] {
          1
        } else {
          0
        }
      },
      Operator::Equal => {
        if sub_packet_results[0] == sub_packet_results[1] {
          1
        } else {
          0
        }
      }
    }
  }

  fn decode_literal(&mut self) -> isize {
    // Literal
    let mut literal: Vec<char> = Vec::new();

    loop {
      let mut is_last_batch = false;
      let mut batch: BitRange = self.0.slice(5).unwrap();
      if batch.next() == '0' {
        is_last_batch = true
      }

      let mut next_4 = batch.take(4);
      literal.append(&mut next_4);

      if is_last_batch {
        break;
      }
    }

    BitRange::from(&literal[..]).as_isize().unwrap()
  }

  // Returns a tuple containing the version and type_id
  fn header(&mut self) -> (isize, isize) {
    (self.version(), self.type_id())
  }

  fn length_type_id(&mut self) -> isize {
    self.0.slice(1).unwrap().as_isize().unwrap()
  }

  fn version(&mut self) -> isize {
    self.0.slice(3).unwrap().as_isize().unwrap()
  }

  fn type_id(&mut self) -> isize {
    self.0.slice(3).unwrap().as_isize().unwrap()
  }
}

impl From<&str> for Parser {
  fn from(s: &str) -> Self {
    Parser(BitRange::from(s))
  }
}

enum Operator {
  Sum,
  Product,
  Min,
  Max,
  GreaterThan,
  LessThan,
  Equal
}

impl From<isize> for Operator {
  fn from(i: isize) -> Self {
    match i {
      0 => Operator::Sum,
      1 => Operator::Product,
      2 => Operator::Min,
      3 => Operator::Max,
      5 => Operator::GreaterThan,
      6 => Operator::LessThan,
      7 => Operator::Equal,
      _ => panic!("Unrecognized operator: {}", i)
    }
  }
}