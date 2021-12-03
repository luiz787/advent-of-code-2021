use std::io;
use std::io::Result;
use std::io::BufRead;

pub fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut prev: Option<i64> = None;
    let mut increases: u64 = 0;
    for line in lines {
        let reading: i64 = line.unwrap().trim().parse().unwrap();
        let should_increment = prev.map(|v| v < reading).unwrap_or(false);
        if should_increment {
            increases += 1;
        }

        prev = Some(reading)
    }

    println!("{}", increases);

    Ok(())
}
