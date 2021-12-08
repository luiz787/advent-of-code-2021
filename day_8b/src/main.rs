use std::collections::HashSet;

fn main() {
    let input = include_str!("../input").trim();
    let lines = parse_input(input);

    let sum: u64 = lines
        .iter()
        .map(|&l| l.split_once(" | ").unwrap())
        .map(|(pattern, digits)| {
            let mut patterns = as_hashset_vector(pattern);
            patterns.sort_by_key(HashSet::len);


            (patterns, as_hashset_vector(digits))
        })
        .map(|(patterns, digits)| digits.iter()
            .map(|d| decode(&patterns, d))
            .map(|v| v.to_string())
            .fold(String::new(), |acc, v| {
                acc + &v
            })
            .parse::<u64>()
            .unwrap()
        )
        .sum();

    println!("{}", sum);
}

fn as_hashset_vector(str: &str) -> Vec<HashSet<char>> {
    str
        .trim()
        .split_whitespace()
        .map(str::chars)
        .map(Iterator::collect::<HashSet<char>>)
        .collect()
}

fn decode(patterns: &[HashSet<char>], digit: &HashSet<char>) -> u16 {
    if digit.len() == patterns[0].len() {
        return 1;
    }
    if digit.len() == patterns[1].len() {
        return 7;
    }
    if digit.len() == patterns[2].len() {
        return 4;
    }
    if digit.len() == patterns[patterns.len() -1].len() {
        return 8;
    }

    if digit.len() == 5 {
        return if patterns[0].difference(digit).count() == 0 {
            3
        } else if patterns[2].difference(digit).count() == 2 {
            2
        } else {
            5
        }
    } else {
        return if patterns[1].difference(digit).count() == 1 {
            6
        } else if patterns[2].difference(digit).count() == 0 {
            9
        } else {
            0
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input
        .trim()
        .split('\n')
        .collect()
}
