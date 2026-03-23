use crate::common::{get_lines, Answer};
use crate::Args;

type Position = [f64; 3];
type Velocity = [f64; 3];

type Hailstone = (Position, Velocity);

pub fn run(_args: &Args) -> Answer {
    let (lines, lower, upper) = if _args.test {
        (get_lines("day24-test"), 7_f64, 27_f64)
    } else {
        (get_lines("day24"), 200000000000000_f64, 400000000000000_f64)
    };

    let hailstones = lines.iter().map(to_hail).collect::<Vec<_>>();

    let part1 = hailstones
        .iter()
        .enumerate()
        .map(|(i, a)| {
            hailstones
                .iter()
                .skip(i + 1)
                .filter(|b| collides_in_range_xy(a, b, lower, upper))
                .count()
        })
        .sum::<usize>();

    (part1.to_string(), "".to_string())
}

fn collides_in_range_xy(a: &Hailstone, b: &Hailstone, min: f64, max: f64) -> bool {
    let Some((x, y)) = path_collision(a, b) else {
        return false;
    };

    min <= x && x <= max && min <= y && y <= max
}

fn path_collision(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let (a_grad, a_intercept) = decompose(a);
    let (b_grad, b_intercept) = decompose(b);

    if a_grad == b_grad {
        return None;
    }

    // ay = bx + c
    // iy = jx + k
    //
    // x = (c - k) / (j - b)
    let x_intercept = (a_intercept - b_intercept) / (b_grad - a_grad);
    let y_intercept = a_grad * x_intercept + a_intercept;

    let ax_diff = x_intercept - a.0[0];
    if ax_diff * a.1[0] < 0_f64 {
        return None;
    }

    let bx_diff = x_intercept - b.0[0];
    if bx_diff * b.1[0] < 0_f64 {
        return None;
    }

    Some((x_intercept, y_intercept))
}

fn decompose(a: &Hailstone) -> (f64, f64) {
    // m = dy/dx
    let gradient = a.1[1] / a.1[0];
    // y - mx = c
    let intercept = a.0[1] - (gradient * a.0[0]);

    (gradient, intercept)
}

fn collision_xy(a: &Hailstone, b: &Hailstone) -> Option<Position> {
    let Some(x_t) = collision_dim_t(a, b, 0) else {
        return None;
    };

    let Some(y_t) = collision_dim_t(a, b, 1) else {
        return None;
    };

    None
}

fn collision_dim_t(a: &Hailstone, b: &Hailstone, dim: usize) -> Option<f64> {
    let p_diff = a.0[dim] - b.0[dim];
    let v_diff = a.1[dim] - b.1[dim];
    if v_diff == 0.0 && p_diff != 0.0 {
        return None;
    }

    None
}

fn to_hail(line: &String) -> Hailstone {
    let no_whitespace = line.replace(" ", "");

    let (pos, vel) = no_whitespace
        .split_once('@')
        .expect("Couldn't find delimiter");

    (to_vector(pos), to_vector(vel))
}

fn to_vector(str: &str) -> [f64; 3] {
    str.split(',')
        .filter_map(|x| x.parse::<f64>().ok())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
