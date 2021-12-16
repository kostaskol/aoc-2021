use crate::utils::parsing::read_file;
use crate::utils::bit_range::BitRange;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("16", test);
  let non_comment = lines.iter().find(|&line| !line.starts_with('#')).unwrap();
  let packet = BitRange::from_hex(non_comment);

  format!("{}",
    match extra {
      false => p1::run(packet),
      true => p2::run(packet)
    }
  )
}

mod p1 {
  use crate::utils::bit_range::*;

  pub fn run(mut packet: BitRange) -> isize {
    parse_packet(&mut packet)
  }

  fn parse_packet(mut packet: &mut BitRange) -> isize {
    let (version, type_id) = header(&mut packet);

    let mut version_sum = version;

    if type_id == 4 {
      // Literal packet
      as_literal(&mut packet);
    } else {
      version_sum += as_op(&mut packet);
    }

    version_sum
  }

  fn as_op(mut packet: &mut BitRange) -> isize {
    let length_type_id = length_type_id(&mut packet);
    let mut version_sum = 0;

    if length_type_id == 1 {
      let sub_packets = packet.slice(11).unwrap().as_isize().unwrap();
      for _ in 0..sub_packets {
        version_sum += parse_packet(&mut packet);
      }
    } else {
      let num_bits = packet.slice(15).unwrap().as_isize().unwrap() as usize;
      let target = packet.pos() + num_bits;
      while packet.pos() < target {
        version_sum += parse_packet(&mut packet);
      }
    }
    version_sum
  }

  fn as_literal(packet: &mut BitRange) {
    // Literal
    let mut literal: Vec<char> = Vec::new();

    loop {
      let mut is_last_batch = false;
      let mut batch: BitRange = packet.slice(5).unwrap();
      if batch.next() == '0' {
        is_last_batch = true
      }

      let mut next_4 = batch.take(4);
      literal.append(&mut next_4);

      if is_last_batch {
        break;
      }
    }

    // BitRange::from(&literal).as_isize().unwrap();
  }
}

mod p2 {
  use crate::utils::bit_range::*;

  pub fn run(mut packet: BitRange) -> isize {
    parse_packet(&mut packet)
  }

  fn parse_packet(mut packet: &mut BitRange) -> isize {
    let (_, type_id) = header(&mut packet);

    if type_id == 4 {
      // Our base case. Literal
      as_literal(&mut packet)
    } else {
      as_op(&mut packet, type_id)
    }
  }

  fn as_op(mut packet: &mut BitRange, type_id: isize) -> isize {
    let length_type_id = length_type_id(&mut packet);

    let mut sub_packet_results: Vec<isize> = Vec::new();

    if length_type_id == 1 {
      let sub_packets = packet.slice(11).unwrap().as_isize().unwrap();
      for _ in 0..sub_packets {
        sub_packet_results.push(parse_packet(&mut packet));
      }
    } else {
      let num_bits = packet.slice(15).unwrap().as_isize().unwrap() as usize;
      let target = packet.pos() + num_bits;
      while packet.pos() < target {
        sub_packet_results.push(parse_packet(&mut packet));
      }
    }

    match type_id {
      0 => { // Sum
        sub_packet_results.iter().sum()
      },
      1 => { // Product
        sub_packet_results.iter().product()
      },
      2 => { // Min
        *sub_packet_results.iter().min().unwrap()
      },
      3 => { // Max
        *sub_packet_results.iter().max().unwrap()
      },
      5 => { // Greater than
        if sub_packet_results.len() != 2 {
          panic!("Greater than with more than two packets")
        }
        if sub_packet_results[0] > sub_packet_results[1] {
          1
        } else {
          0
        }
      },
      6 => { // Less than
        if sub_packet_results.len() != 2 {
          panic!("Greater than with more than two packets")
        }
        if sub_packet_results[0] < sub_packet_results[1] {
          1
        } else {
          0
        }
      },
      7 => { // Equal
        if sub_packet_results.len() != 2 {
          panic!("Greater than with more than two packets")
        }
        if sub_packet_results[0] == sub_packet_results[1] {
          1
        } else {
          0
        }
      },
      x => panic!("Unrecognized type_id: {}", x)
    }
  }

  fn as_literal(packet: &mut BitRange) -> isize {
    // Literal
    let mut literal: Vec<char> = Vec::new();

    loop {
      let mut is_last_batch = false;
      let mut batch: BitRange = packet.slice(5).unwrap();
      if batch.next() == '0' {
        is_last_batch = true
      }

      let mut next_4 = batch.take(4);
      literal.append(&mut next_4);

      if is_last_batch {
        break;
      }
    }

    BitRange::from(&literal).as_isize().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "31");
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "54");
  }
}