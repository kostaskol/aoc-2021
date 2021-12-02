use std::fs;

pub fn read_lines(input: &str) -> Vec<String> {
    let content = fs::read_to_string(input).expect(&format!("Put something in {}, jeez..", input));

    content.split('\n').map(|s| s.to_string()).collect()

}

pub fn convert_to_ints(lines: &Vec<String>) -> Vec<i32> {
    lines.iter().map(|s| s.parse::<i32>().
    unwrap_or_else(|_| panic!("{} is not a number", s)))
    .collect()
}
