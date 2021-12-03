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

    let mut oxygen_generator_lines = lines.clone();
    let mut co2_scrubber_rate_lines = lines.clone();

    let width = lines.iter().last().unwrap().len();

    for i in 0..width {
        if oxygen_generator_lines.len() == 1 {
            break
        }
        let freqs = compute_freqs_by_index(&oxygen_generator_lines, i);

        filter_oxygen_lines(freqs, i, &mut oxygen_generator_lines);
    }

    for i in 0..width {
        if co2_scrubber_rate_lines.len() == 1 {
            break
        }
        let freqs = compute_freqs_by_index(&co2_scrubber_rate_lines, i);

        filter_co2_lines(freqs, i, &mut co2_scrubber_rate_lines);
    }

    let oxygen_generator_rating = oxygen_generator_lines.iter().last().unwrap();
    let oxygen_generator_rating = i64::from_str_radix(oxygen_generator_rating, 2).unwrap();

    let co2_scrubber_rating = co2_scrubber_rate_lines.iter().last().unwrap();
    let co2_scrubber_rating = i64::from_str_radix(co2_scrubber_rating, 2).unwrap();

    println!("{}", oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}

fn compute_freqs_by_index(lines: &[String], i: usize) -> [i32; 2] {
    let mut freqs = [0,0];
    for line in lines {
        match line.chars().nth(i).unwrap() {
            '0' => freqs[0] += 1,
            '1' => freqs[1] += 1,
            _ => panic!("Unexpected char")
        }
    }
    freqs
}

fn filter_oxygen_lines(freqs: [i32; 2], i: usize, oxygen_generator_lines: &mut Vec<String>) {
    if freqs[0] > freqs[1] {
        *oxygen_generator_lines = oxygen_generator_lines
        .iter()
        .filter(|l| l.chars().nth(i) == Some('0'))
        .map(|s| s.into())
        .collect();
    } else {
        *oxygen_generator_lines = oxygen_generator_lines
        .iter()
        .filter(|l| l.chars().nth(i) == Some('1'))
        .map(|s| s.into())
        .collect();
    }
}

fn filter_co2_lines(freqs: [i32; 2], i: usize, co2_lines: &mut Vec<String>) {
    if freqs[1] < freqs[0] {
        *co2_lines = co2_lines
        .iter()
        .filter(|l| l.chars().nth(i) == Some('1'))
        .map(|s| s.into())
        .collect();
    } else {
        *co2_lines = co2_lines
        .iter()
        .filter(|l| l.chars().nth(i) == Some('0'))
        .map(|s| s.into())
        .collect();
    }
}
