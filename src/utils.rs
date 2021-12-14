use std::fs;

pub fn read_lines(input: &str) -> Vec<String> {
    let content = fs::read_to_string(input).unwrap_or_else(|_| panic!("Put something in {}, jeez..", input));

    content.split('\n').map(|s| s.to_string()).collect()
}

pub fn convert_to_ints(lines: &[String]) -> Vec<i32> {
    lines.iter().map(|s| s.parse::<i32>().
    unwrap_or_else(|_| panic!("{} is not a number", s)))
    .collect()
}

pub fn inp_file(day: &str, test: bool) -> String {
    format!("inputs/{}.txt",
        match test {
            true => "test",
            false => day
        }
    )
}