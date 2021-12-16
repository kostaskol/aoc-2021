use crate::utils::input::read_file;
use crate::parsing::Parser;

pub fn run(extra: bool, test: bool) -> String {
  let lines = read_file("16", test);
  let packet = Parser::from(&lines[0][..]);

  format!("{}",
    match extra {
      false => p1::run(packet),
      true => p2::run(packet)
    }
  )
}

mod p1 {
  use crate::parsing::Parser;

  pub fn run(mut parser: Parser) -> isize {
    parser.versions()
  }
}

mod p2 {
  use crate::parsing::Parser;

  pub fn run(mut packet: Parser) -> isize {
    packet.execute()
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