#[derive(Debug)]
enum Number {
    Literal(u64),
    Pair(Box<Number>, Box<Number>),
}

#[derive(PartialEq, Eq, Debug)]
enum PairSide {
    Left,
    Right,
}

impl Number {
    
    fn parse(input: &str) -> Number {
        Number::create(input).0
    }

    fn create(input: &str) -> (Number, usize) {
        let mut saw_own_opening_brace = false;
        let mut l = None;
        let mut r = None;
        let mut resolving = PairSide::Left;
        let mut pos = 0;
        while pos < input.chars().count() {
            let c = input.chars().nth(pos).unwrap();
            if c == '[' {
                if !saw_own_opening_brace {
                    pos += 1;
                    saw_own_opening_brace = true;
                    continue;
                }
                if resolving == PairSide::Left {
                    let (left, advance_by) = Number::create(&input[pos..]);
                    l = Some(left);
                    pos += advance_by;
                } else {
                    let (right, advance_by) = Number::create(&input[pos..]);
                    r = Some(right);
                    pos += advance_by;
                    return (
                        Number::Pair(Box::new(l.unwrap()), Box::new(r.unwrap())),
                        pos,
                    );
                }
            } else if c == ',' {
                resolving = PairSide::Right;
                pos += 1;
            } else if c == ']' {
                if resolving == PairSide::Right {
                    return (
                        Number::Pair(Box::new(l.unwrap()), Box::new(r.unwrap())),
                        pos + 1,
                    );
                } else {
                    pos += 1;
                }
            } else {
                pos += 1;
                if resolving == PairSide::Left {
                    l = Some(Number::Literal(c.to_digit(10).unwrap().into()));
                } else {
                    r = Some(Number::Literal(c.to_digit(10).unwrap().into()));
                    return (
                        Number::Pair(Box::new(l.unwrap()), Box::new(r.unwrap())),
                        pos + 1,
                    );
                }
            }
        }
        unreachable!();
    }

    fn sum(a: Number, b: Number) -> Number {
        Number::Pair(Box::new(a), Box::new(b))
    }

    fn magnitude(self) -> u64 {
        match self {
            Number::Literal(v) => v,
            Number::Pair(a, b) => {
                (3 * a.magnitude()) + (2 * b.magnitude())
            }
        }
    }

    fn add_to_right(&mut self, num: u64) {
        match self {
            Number::Pair(a, _b) => {
                a.add_to_right(num);
            }
            Number::Literal(n) => {
                *n += num;
            }
        }
    }

    fn add_to_left(&mut self, num: u64) {
        match self {
            Number::Pair(_a, b) => {
                b.add_to_left(num);
            }
            Number::Literal(n) => {
                *n += num;
            }
        }
    }

    fn reduce(self) -> Number {
        let mut me = self;
        loop {
            let mut iters = 0;
            let mut exploded = true;

            while exploded {
                exploded = false;
                me = me.explode(0, &mut exploded).1;
                iters += 1;
            }
            let mut did_split = false;
            me = me.split(&mut did_split);
            
            if !did_split && iters == 1 {
                break;
            }
        }

        me
    }

    fn explode(self, depth: usize, exploded: &mut bool) -> (Option<u64>, Number, Option<u64>) {
        match self {
            Number::Pair(a, mut b) => {
                if depth == 3 {
                    match (*a, *b) {
                        (Number::Pair(a, b), mut c) => {
                            *exploded = true;
                            let a = if let Number::Literal(v) = *a { v } else { unreachable!() };
                            let b = if let Number::Literal(v) = *b { v } else { unreachable!() };

                            c.add_to_right(b);

                            (
                                Some(a),
                                Number::Pair(Box::new(Number::Literal(0)), Box::new(c)),
                                None
                            )
                        }
                        (mut a, Number::Pair(b, c)) => {
                            *exploded = true;
                            let b = if let Number::Literal(v) = *b { v } else { unreachable!() };
                            let c = if let Number::Literal(v) = *c { v } else { unreachable!() };

                            a.add_to_left(b);
                            (
                                None,
                                Number::Pair(Box::new(a), Box::new(Number::Literal(0))),
                                Some(c)
                            )
                        }
                        (a, b) => (None, Number::Pair(Box::new(a), Box::new(b)), None),
                    }
                } else {
                    let (le, mut a, mut re) = a.explode(depth + 1, exploded);
                    if let Some(re) = re.take() {
                        b.add_to_right(re);
                    }
                    if *exploded {
                        return (le, Number::Pair(Box::new(a), b), re);
                    }

                    let (mut le, b, re) = b.explode(depth + 1, exploded);
                    if let Some(le) = le.take() {
                        a.add_to_left(le);
                    }
                    (le, Number::Pair(Box::new(a), Box::new(b)), re)
                }
            }
            Number::Literal(_) => (None, self, None),
        }
    }

    fn split(self, did_split: &mut bool) -> Number {
        match self {
            Number::Pair(a, b) => {
                if *did_split {
                    return Number::Pair(a, b);
                }
                let a = a.split(did_split);
                if *did_split {
                    Number::Pair(Box::new(a), b)
                } else {
                    Number::Pair(Box::new(a), Box::new(b.split(did_split)))
                }
            }
            Number::Literal(a) => {
                if a > 9 {
                    *did_split = true;
                    Number::Pair(
                        Box::new(Number::Literal(a / 2)),
                        Box::new(Number::Literal(a - (a / 2)))
                    )
                } else {
                    self
                }
            }
        }
    }

}



fn main() {
    let input = include_str!("../input");

    let numbers = parse_input(input);
    
    let sum = numbers
        .into_iter()
        .reduce(|a, b| Number::sum(a, b).reduce())
        .unwrap();
    
    
    println!("{}", sum.magnitude());
}

fn parse_input(input: &str) -> Vec<Number> {
    let lines: Vec<_> = input.trim().split('\n').collect();

    let mut nums = Vec::new();
    for line in lines {
        let n = Number::parse(line).reduce();
        nums.push(n);
    }

    nums
}
