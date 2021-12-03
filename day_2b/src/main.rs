use std::io;
use std::io::Result;
use std::io::BufRead;

pub fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut aim = 0;
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("forward") {
            let delta = get_delta(line);
            horizontal_pos += delta;
            depth += aim * delta;
        } else if line.starts_with("down") {
            let delta = get_delta(line);
            aim += delta;
        } else if line.starts_with("up") {
            let delta = get_delta(line);
            aim -= delta;
        }
    }

    println!("{}", horizontal_pos * depth);

    Ok(())
}

fn get_delta(line: String) -> i64 {
    let delta: i64 = line
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();
    delta
}
