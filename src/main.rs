use std::{process, io::{self, stdin, stdout, BufRead, Write}};

fn validate_aaah(input: &str) {
    let mut has_seen_h = false;
    for c in input.chars() {
        if has_seen_h {
            if c == '\n'{
                return
            };
            eprintln!("invalid 'aaah' due to characters after 'h' in '{}'", input);
            process::exit(3);
        }
        match c {
            _ if c == 'a' => continue,
            _ if c == 'h' => {
                has_seen_h = true;
                continue;
            }
            _ => {
                eprintln!("invalid 'aaah' due to character which was not 'a' or 'h' in '{}'", input);
                process::exit(3);
            }
        };
    }
}

fn aaahs_of(input: &str) -> usize {
    validate_aaah(input);
    input.len() - 1
}

fn main() -> Result<(), io::Error> {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin_lock, mut stdout_lock) = (stdin.lock(), stdout.lock());

    let mut first_line = String::new();
    let mut second_line = String::new();
    loop {
        first_line.clear();
        second_line.clear();
        stdin_lock.read_line(&mut first_line)?;
        stdin_lock.read_line(&mut second_line)?;

        match (first_line.len(), second_line.len()) {
            (0, 0) => process::exit(0),
            (_, 0) => {
                eprintln!("input exhausted prematurely");
                process::exit(2)
            }
            _ => if aaahs_of(&first_line) >= aaahs_of(&second_line) {
                writeln!(stdout_lock, "go")?;
            } else {
                writeln!(stdout_lock, "no")?;
            },
        }
    }
}
