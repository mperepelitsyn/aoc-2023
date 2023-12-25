use std::fs::read_to_string;

fn part1(input: &str, lo: i64, hi: i64) -> usize {
    let hails: Vec<_> = input
        .lines()
        .map(|line| {
            line.split(" @ ")
                .flat_map(|nums| {
                    nums.split(", ")
                        .map(|num| num.trim().parse::<i64>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (0..hails.len() - 1)
        .flat_map(|i| (i + 1..hails.len()).map(move |j| (i, j)))
        .filter_map(|(i, j)| intersect_2d(&hails[i], &hails[j]))
        .filter(|&pos| is_within_bounds(pos, lo, hi))
        .count()
}

fn intersect_2d(r1: &[i64], r2: &[i64]) -> Option<(f64, f64)> {
    let (x1, y1, dx1, dy1) = (r1[0], r1[1], r1[3], r1[4]);
    let (x2, y2, dx2, dy2) = (r2[0], r2[1], r2[3], r2[4]);
    let div = dx2 * dy1 - dy2 * dx1;
    if div == 0 {
        return None;
    }

    let t1 = (dy2 * (x1 - x2) + dx2 * (y2 - y1)) as f64 / div as f64;
    let t2 = (dy1 * (x1 - x2) + dx1 * (y2 - y1)) as f64 / div as f64;
    if t1 <= 0.0 || t2 <= 0.0 {
        return None;
    }

    Some((x1 as f64 + dx1 as f64 * t1, y1 as f64 + dy1 as f64 * t1))
}

fn is_within_bounds((x, y): (f64, f64), lo: i64, hi: i64) -> bool {
    x >= lo as f64 && x <= hi as f64 && y >= lo as f64 && y <= hi as f64
}

fn main() {
    let input = read_to_string("input/24.txt").unwrap();
    let start = std::time::Instant::now();
    println!(
        "Part 1: {}",
        part1(&input, 200000000000000, 400000000000000)
    );
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/24.txt").unwrap(), 7, 27), 2);
    }
}
