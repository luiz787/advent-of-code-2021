use std::collections::{HashMap, VecDeque};

fn main() {
    let lines: Vec<&str> = include_str!("../input")
        .trim()
        .lines()
        .map(str::trim)
        .collect();

    let opening: Vec<char> = ['{', '(', '[', '<'].to_vec();
    let mut expected_closing = HashMap::new();
    expected_closing.insert('{', '}');
    expected_closing.insert('(', ')');
    expected_closing.insert('[', ']');
    expected_closing.insert('<', '>');

    let mut value_table = HashMap::new();
    value_table.insert(')', 3);
    value_table.insert(']', 57);
    value_table.insert('}', 1197);
    value_table.insert('>', 25137);

    let mut sum = 0;

    for line in lines {
        let mut stack = VecDeque::new();
        for c in line.chars() {
            if opening.contains(&c) {
                stack.push_front(c);
            } else {
                let val = stack.pop_front();
                if val == None {
                    sum += value_table.get(&c).unwrap();
                    break;
                }
                let val = val.unwrap();
                if expected_closing.get(&val).unwrap() != &c {
                    sum += value_table.get(&c).unwrap();
                    break;
                }
            }
        }
    }

    println!("{}", sum);
}
