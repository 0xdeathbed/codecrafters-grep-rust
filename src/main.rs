mod regx;

use crate::regx::Regx;
use anyhow::Result;
use std::env;
use std::io;
use std::process;

// fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     match pattern {
//         c if c.chars().count() == 1 => input_line.contains(c),
//         "\\d" => input_line.chars().any(|c| c.is_digit(10)),
//         "\\w" => input_line
//             .chars()
//             .any(|c| c.is_ascii_alphanumeric() || c == '_'),
//         negative if negative.starts_with("[^") && negative.ends_with("]") => {
//             let p = &negative[2..negative.len() - 1];
//             input_line.chars().all(|c| !p.contains(c))
//         }
//         positve if positve.starts_with("[") && positve.ends_with("]") => {
//             let p = &positve[1..positve.len() - 1];
//             input_line.chars().any(|c| p.contains(c))
//         }
//         _ => panic!("Unhandled pattern: {pattern}"),
//     }
// }

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() -> Result<()> {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    let grep = Regx::new(&input_line, &pattern);

    // println!("{:#?}", grep.pattern);

    if grep.matches() {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
