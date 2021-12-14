
use std::collections::HashMap;
use itertools::Itertools;

struct PairInsertionRule {
    from: Vec<char>,
    to: char
}

impl PairInsertionRule {
    pub fn new(rule_line: &str) -> PairInsertionRule {
        let (from, to) = rule_line.split_once(" -> ").unwrap();
        
        PairInsertionRule {
            from: from.chars().take(2).collect(),
            to: to.chars().next().unwrap()
        }
    }
}
fn main() {
    let input = include_str!("../input").trim();
    let (template, rules) = parse_input(input);

    let mut template: Vec<char> = template.chars().collect();

    let iterations = 10;

    for _ in 0..iterations {
        let mut insertions = Vec::new();
        let mut pos: usize = 1;
        for (a, b) in template.iter().tuple_windows() {
            for rule in &rules {
                if rule.from[0] == *a && rule.from[1] == *b {
                    insertions.push((pos, rule.to));
                    pos += 1;
                    break;
                }
            }
            pos += 1;
        }

        for (pos, val) in insertions {
            template.insert(pos, val);
        }
    }

    let mut frequencies: HashMap<char, u64> = HashMap::new();
    for char in template {
        *frequencies
            .entry(char)
            .or_default() += 1;
    }


    let frequencies: Vec<_> = frequencies
        .iter()
        .sorted_by_key(|&t| t.1)
        .map(|pair| *pair.1)
        .collect_vec();

    let first= frequencies[0];
    let last = frequencies[frequencies.len() - 1];

    println!("{}", last - first);
}

fn parse_input(input: &str) -> (&str, Vec<PairInsertionRule>) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules: Vec<PairInsertionRule> = rules
        .split('\n')
        .map(PairInsertionRule::new)
        .collect();
    
    (template, rules)
}