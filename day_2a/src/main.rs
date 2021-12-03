use std::io;
use std::io::Result;
use std::io::BufRead;

pub fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for line in lines {
        let line = line.unwrap();
        if line.starts_with("forward") {
            let delta = get_delta(line);
            horizontal_pos += delta;
        } else if line.starts_with("down") {
            let delta = get_delta(line);
            vertical_pos += delta;
        } else if line.starts_with("up") {
            let delta = get_delta(line);
            vertical_pos -= delta;
        }
    }

    println!("{}", horizontal_pos * vertical_pos);

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
