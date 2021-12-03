use std::io;
use std::io::Result;
use std::io::BufRead;

pub fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let width = lines.iter().last().unwrap().len();

    let mut gamma = String::new();
    let mut epsilon = String::new();
    (0..width).for_each(|w| {
        let mut freqs = [0,0];
        for line in &lines {
            match line.chars().nth(w).unwrap() {
                '0' => freqs[0] += 1,
                '1' => freqs[1] += 1,
                _ => panic!("Unexpected char")
            }
        }

        if freqs[0] > freqs[1] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    });

    let gamma = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();
    
    println!("{}", gamma * epsilon);

    Ok(())
}
