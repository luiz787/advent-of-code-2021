use std::io;
use std::io::Result;
use std::io::BufRead;

use std::collections::VecDeque;

fn sum(v: &VecDeque<i64>, up_to: usize) -> i64 {
    v.iter().take(up_to).sum()
}

pub fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut prev_window: Option<VecDeque<i64>> = None;
    let mut current_window: VecDeque<i64> = VecDeque::new();
    let mut increases: u64 = 0;
    for line in lines {
        let reading: i64 = line.unwrap().trim().parse().unwrap();
        update_window(&mut current_window, reading);

        if let Some(i) = prev_window.as_mut() {
            let prev_sum = sum(i, i.len());
            let current_sum = sum(&current_window, i.len());
            if prev_sum < current_sum {
                increases += 1;
            }
            update_window(i, reading);
        } else {
            let mut new_vec_deque = VecDeque::new();
            new_vec_deque.push_back(reading);
            prev_window = Some(new_vec_deque);
        }
    }

    println!("{}", increases);

    Ok(())
}

fn update_window(window: &mut VecDeque<i64>, reading: i64) {
    if window.len() < 3 {
        window.push_back(reading);
    } else {
        window.pop_front();
        window.push_back(reading);
    }
}
