fn main() {
    let input = include_str!("../input");
    
    let (_xbounds, ybounds) = parse_input(input);

    if ybounds.0 > 0 && ybounds.1 > 0 {
        let vel = ybounds.0.max(ybounds.1) - 1;
        println!("{}", vel * (vel + 1) / 2);
    } else if ybounds.0 < 0 && ybounds.1 < 0 {
        let vel = ybounds.0.min(ybounds.1).abs() - 1;
        println!("{}", vel * (vel + 1) / 2);
    } else {
        unreachable!();
    };

}

fn parse_input(input: &str) -> ((i64, i64), (i64, i64)) {
    let input = input
        .trim()
        .replace("target area: ", "");

    let (xbounds, ybounds) = input
        .split_once(", ").unwrap();

    let (xstart, xend) = xbounds
        .replace("x=", "")
        .split_once("..")
        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
        .unwrap();
    
    let (ystart, yend) = ybounds
        .replace("y=", "")
        .split_once("..")
        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
        .unwrap();
    
    ((xstart, xend), (ystart, yend))
}
