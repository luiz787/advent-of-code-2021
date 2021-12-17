fn main() {
    let input = include_str!("../input");

    let (xbounds, ybounds) = parse_input(input);

    let y_vel_bounds = calculate_y_velocity_bounds(ybounds);
    let x_vel_bounds = calculate_x_velocity_bounds(xbounds);

    let mut sum = 0;
    for xvel in x_vel_bounds.0.min(x_vel_bounds.1)..x_vel_bounds.0.max(x_vel_bounds.1) {
        for yvel in y_vel_bounds.0.min(y_vel_bounds.1)..y_vel_bounds.0.max(y_vel_bounds.1) + 1 {
            if reaches(xbounds, ybounds, xvel, yvel) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn calculate_y_velocity_bounds(ybounds: (i64, i64)) -> (i64, i64) {
    if ybounds.0 > 0 && ybounds.1 > 0 {
        let vel = ybounds.0.max(ybounds.1) - 1;

        (vel, -vel - 1)
    } else if ybounds.0 < 0 && ybounds.1 < 0 {
        let vel = ybounds.0.min(ybounds.1).abs() - 1;

        (vel, -vel - 1)
    } else {
        unreachable!();
    }
}

fn reaches(
    xbounds: (i64, i64),
    ybounds: (i64, i64),
    initial_x_vel: i64,
    initial_y_vel: i64,
) -> bool {
    let mut xpos = 0;
    let mut ypos = 0;

    let mut xvel = initial_x_vel;
    let mut yvel = initial_y_vel;

    loop {
        xpos += xvel;
        ypos += yvel;

        if (xbounds.0..xbounds.1 + 1).contains(&xpos) && (ybounds.0..ybounds.1 + 1).contains(&ypos)
        {
            return true;
        }

        if fell_too_deep(ypos, ybounds)
            || overshot_right(xbounds, xpos)
            || overshot_left(xbounds, xpos)
        {
            return false;
        }

        xvel += drag(xvel);
        yvel -= 1;
    }
}

fn overshot_right(xbounds: (i64, i64), xpos: i64) -> bool {
    xbounds.0 > 0 && xbounds.1 > 0 && xpos > xbounds.0.max(xbounds.1)
}

fn overshot_left(xbounds: (i64, i64), xpos: i64) -> bool {
    xbounds.0 < 0 && xbounds.1 < 0 && xpos < xbounds.0.min(xbounds.1)
}

fn fell_too_deep(ypos: i64, ybounds: (i64, i64)) -> bool {
    ypos < ybounds.0.min(ybounds.1)
}

fn drag(xvel: i64) -> i64 {
    if xvel >= 1 {
        -1
    } else if xvel <= -1 {
        1
    } else {
        0
    }
}

fn calculate_x_velocity_bounds(xbounds: (i64, i64)) -> (i64, i64) {
    if xbounds.0 > 0 && xbounds.1 > 0 {
        let min_x = xbounds.0.min(xbounds.1);
        let max_x = xbounds.0.max(xbounds.1);

        let mut predicted_pos = 0;
        let mut delta = 0;
        while predicted_pos < min_x {
            predicted_pos += delta;
            delta += 1;
        }

        assert!((xbounds.0..xbounds.1).contains(&predicted_pos));
        (delta - 1, max_x + 1)
    } else if xbounds.0 < 0 && xbounds.1 < 0 {
        let max_x = xbounds.0.min(xbounds.1);
        let min_x = xbounds.0.min(xbounds.1);

        let mut predicted_pos = 0;
        let mut delta = 0;
        while predicted_pos > max_x {
            predicted_pos -= delta;
            delta += 1;
        }

        assert!((xbounds.0..xbounds.1).contains(&predicted_pos));
        (delta - 1, min_x + 1)
    } else {
        (xbounds.0.min(xbounds.1), xbounds.0.max(xbounds.1))
    }
}

fn parse_input(input: &str) -> ((i64, i64), (i64, i64)) {
    let input = input.trim().replace("target area: ", "");

    let (xbounds, ybounds) = input.split_once(", ").unwrap();

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
