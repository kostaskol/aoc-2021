use std::fs::read_to_string;

pub fn read_file(day: &str, test: bool) -> Vec<String> {
    read_lines(&inp_file(day, test))
}

pub fn convert_to_ints(lines: &[String]) -> Vec<i32> {
    lines.iter().map(|s| s.parse::<i32>().
    unwrap_or_else(|_| panic!("{} is not a number", s)))
    .collect()
}

fn read_lines(input: &str) -> Vec<String> {
    let content = read_to_string(input)
        .unwrap_or_else(|_| panic!("Put something in {} first", input));

    content.split('\n').map(|s| s.to_string()).collect()
}

fn inp_file(day: &str, test: bool) -> String {
    if test {
        format!("input/test/day{}.in", day)
    } else {
        format!("input/day{}.in", day)
    }
}