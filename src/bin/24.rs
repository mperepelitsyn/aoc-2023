use std::fs::read_to_string;

fn part1(input: &str, lo: i64, hi: i64) -> usize {
    let hails = parse_input(input);
    (0..hails.len() - 1)
        .flat_map(|i| (i + 1..hails.len()).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            intersect_2d(
                (hails[i][0], hails[i][1], hails[i][3], hails[i][4]),
                (hails[j][0], hails[j][1], hails[j][3], hails[j][4]),
            )
        })
        .filter(|&(x, y)| x >= lo as f64 && x <= hi as f64 && y >= lo as f64 && y <= hi as f64)
        .count()
}

fn part2(input: &str) -> i64 {
    let hails = parse_input(input);
    let (x, y, z) = find_rock_pos(&hails);
    x + y + z
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" @ ")
                .flat_map(|nums| {
                    nums.split(", ")
                        .map(|num| num.trim().parse::<i64>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn intersect_2d(
    (x1, y1, dx1, dy1): (i64, i64, i64, i64),
    (x2, y2, dx2, dy2): (i64, i64, i64, i64),
) -> Option<(f64, f64)> {
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

fn find_rock_pos(hails: &[Vec<i64>]) -> (i64, i64, i64) {
    const XY_PLANE: usize = 0;
    const YZ_PLANE: usize = 1;
    const RANGE: i64 = 250;

    for vx in -RANGE..RANGE {
        for vy in -RANGE..RANGE {
            if let Some((x, y)) = intersect_2d_all(hails, XY_PLANE, (vx, vy)) {
                for vz in -RANGE..RANGE {
                    if let Some((_, z)) = intersect_2d_all(hails, YZ_PLANE, (vy, vz)) {
                        return (x, y, z);
                    }
                }
            }
        }
    }
    unreachable!()
}

fn intersect_2d_all(hails: &[Vec<i64>], plane: usize, vel: (i64, i64)) -> Option<(i64, i64)> {
    let mut last = None;
    for i in 0..hails.len() - 1 {
        for j in i + 1..hails.len() {
            let r1 = (
                hails[i][plane],
                hails[i][plane + 1],
                hails[i][plane + 3] - vel.0,
                hails[i][plane + 4] - vel.1,
            );
            let r2 = (
                hails[j][plane],
                hails[j][plane + 1],
                hails[j][plane + 3] - vel.0,
                hails[j][plane + 4] - vel.1,
            );
            if r1.2 * r2.3 - r1.3 * r2.2 == 0 {
                continue;
            }

            if let Some(pos) =
                intersect_2d(r1, r2).map(|(x, y)| (x.round() as i64, y.round() as i64))
            {
                if *last.get_or_insert(pos) != pos {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
    last
}

fn main() {
    let input = read_to_string("input/24.txt").unwrap();
    let start = std::time::Instant::now();
    println!(
        "Part 1: {}",
        part1(&input, 200000000000000, 400000000000000)
    );
    println!("Part 2: {}", part2(&input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/24.txt").unwrap(), 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/24.txt").unwrap()), 47);
    }
}
