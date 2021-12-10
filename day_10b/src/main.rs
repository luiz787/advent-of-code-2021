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
    value_table.insert(')', 1);
    value_table.insert(']', 2);
    value_table.insert('}', 3);
    value_table.insert('>', 4);

    let mut values: Vec<u64> = lines
        .iter()
        .map(|&l| is_invalid(l, &opening, &expected_closing))
        .filter(|(invalid, _stack)| !invalid) 
        .map(|(_, stack)| stack)
        .map(|stack: VecDeque<char>| map_to_closing_char(stack, &expected_closing))
        .map(|stack| compute_value(stack, &value_table))
        .collect();

    values.sort_unstable();

    println!("{}", values[values.len() / 2]);
}

fn compute_value(stack: VecDeque<char>, value_table: &HashMap<char, i32>) -> u64 {
    let mut sum: u64 = 0;

    for elem in stack {
        sum *= 5;
        sum += *value_table.get(&elem).unwrap() as u64;
    }

    sum
}

fn map_to_closing_char(stack: VecDeque<char>, expected_closing: &HashMap<char, char>) -> VecDeque<char> {
    stack
        .iter()
        .map(|elem| *expected_closing
                            .get(elem)
                            .unwrap())
        .collect()
}

fn is_invalid(line: &str, opening: &Vec<char>, expected_closing: &HashMap<char, char>) -> (bool, VecDeque<char>) {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        if opening.contains(&c) {
            stack.push_front(c);
        } else {
            let val = stack.pop_front();
            if val == None {
                return (true, stack);
            }
            let val = val.unwrap();
            if expected_closing.get(&val).unwrap() != &c {
                return (true, stack);
            }
        }
    }
    
    (false, stack)
}
