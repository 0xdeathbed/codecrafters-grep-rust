use anyhow::Result;
use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern.chars().count() {
        2 => match pattern {
            "\\d" => input_line.chars().any(|c| c.is_digit(10)),
            "\\w" => input_line
                .chars()
                .any(|c| c.is_ascii_alphanumeric() || c == '_'),
            _ => panic!("Unrecognized Pattern: {pattern}"),
        },
        _ => return input_line.contains(pattern),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() -> Result<()> {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
