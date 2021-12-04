#[derive(Debug)]
struct Bingo {
    drawn_numbers: Vec<u8>,
    played_rounds: u8,
    boards: Vec<Board>
}

impl Bingo {
    fn play_round(&mut self) -> Option<u64> {
        let current_number = self.drawn_numbers.get(self.played_rounds as usize).unwrap();

        for board in &mut self.boards {
            let won = board.play_round(*current_number);

            if won {
                let score = board.score();
                
                return Some(score)
            }
        }

        self.played_rounds += 1;
        None
    }
}

#[derive(Debug)]
struct Board {
    values: Vec<BoardCell>,
    winning_number: Option<u8>
}

impl Board {
    fn play_round(&mut self, number: u8) -> bool {
        for item in self.values.iter_mut() {
            if item.value == number {
                item.seen = true;
            }
        }
        if self.won_by_row() || self.won_by_column() {
            self.winning_number = Some(number);
            true
        } else {
            false
        }
    }

    fn score(&self) -> u64 {
        let sum_of_unmarked: u64 = self
            .values
            .iter()
            .filter(|c| !c.seen)
            .map(|c| c.value as u64)
            .sum();
        self.winning_number.unwrap_or(0) as u64 * sum_of_unmarked
    }

    fn won_by_row(&self) -> bool {
        self
            .values
            .chunks(5)
            .any(|line| line
                                    .iter()
                                    .all(|c| c.seen))
    }

    fn won_by_column(&self) -> bool {
        (0..5).any(|col| {
            self
                .values
                .iter()
                .enumerate()
                .filter(|(i, _v)| i % 5 == col)
                .all(|(_i, v)| v.seen)
        })
    }
}

#[derive(Debug)]
struct BoardCell {
    value: u8,
    seen: bool
}

fn main() {
    let input = include_str!("../input");

    let mut bingo = parse_input(input);

    let winning_board_score = loop {
        let round_result = bingo.play_round();

        if let Some(score) = round_result {
            break score;
        }
    };

    println!("{}", winning_board_score);

}

fn parse_input(input: &str) -> Bingo {
    let input_lines: Vec<&str> = input
        .split('\n')
        .filter(|v| !v.is_empty())
        .collect();

    let mut lines_iterator = input_lines.iter();

    let drawn_numbers: Vec<u8> = parse_line(lines_iterator
        .next()
        .unwrap(), ',');

    let mut boards = Vec::new();
    let board_count = (input_lines.len() - 1) / 5;

    for _i in 0..board_count {
        let board_cells = create_board_cells(&mut lines_iterator);

        boards.push(Board { values: board_cells, winning_number: None });
    }

    Bingo { boards, drawn_numbers, played_rounds: 0 }
}

fn create_board_cells(lines_iterator: &mut std::slice::Iter<&str>) -> Vec<BoardCell> {
    lines_iterator
        .take(5)
        .flat_map(|v| parse_line(v, ' '))
        .map(|k| BoardCell {value: k, seen: false})
        .collect()
}

fn parse_line(line: &str, separator: char) -> Vec<u8> {
    line
        .trim()
        .split(separator)
        .filter(|v| !v.is_empty())
        .map(|v| v.trim().parse().unwrap())
        .collect()
}
