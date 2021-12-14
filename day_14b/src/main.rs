
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

    let template: Vec<char> = template.chars().collect();

    let original_last = template[template.len() - 1];

    let mut pair_freqs: HashMap<(char, char), u64> = HashMap::new();

    for (a, b) in template.iter().tuple_windows() {
        *pair_freqs
            .entry((*a, *b))
            .or_default() += 1;
    }

    let iterations = 40;

    for _ in 0..iterations {
        let mut new_pair_freqs = HashMap::new();
        
        for (a, b) in pair_freqs.keys() {
            let kk = vec![*a, *b];
            if rules.contains_key(&kk) {
                let rule = rules.get(&kk).unwrap();
                let amount = pair_freqs.get(&(*a, *b)).unwrap();
                *new_pair_freqs
                    .entry((*a, *rule))
                    .or_default() += amount;
                
                *new_pair_freqs
                    .entry((*rule, *b))
                    .or_default() += amount;

            } else {
                let amount = pair_freqs.get(&(*a, *b)).unwrap();
                *new_pair_freqs
                    .entry((*a, *b))
                    .or_default() += amount;
            }
        }

        pair_freqs = new_pair_freqs;
    }

    let mut frequencies: HashMap<char, u64> = HashMap::new();
    for ((a, _), v) in &pair_freqs {
        *frequencies
            .entry(*a)
            .or_default() += v;
    }

    *frequencies.get_mut(&original_last).unwrap() += 1;

    let frequencies: Vec<_> = frequencies
        .iter()
        .sorted_by_key(|&t| t.1)
        .map(|pair| *pair.1)
        .collect_vec();

    let first= frequencies[0];
    let last = frequencies[frequencies.len() - 1];

    println!("{}", last - first);
}

fn parse_input(input: &str) -> (&str, HashMap<Vec<char>, char>) {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules: HashMap<Vec<char>, char> = rules
        .split('\n')
        .map(PairInsertionRule::new)
        .map(|p| (p.from, p.to))
        .into_iter()
        .collect();
    
    
    (template, rules)
}
