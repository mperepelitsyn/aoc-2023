use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

fn part1(input: &str, steps: u32) -> u64 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let start = find_start(&grid);
    count_pots(&grid, steps, start)
}

fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let start = find_start(&grid);
    const STEPS: u32 = 26_501_365;
    const POINTS: u32 = 3;

    let offset = STEPS % grid.len() as u32;
    let init: Vec<_> = (0..POINTS)
        .map(|i| count_pots(&grid, offset + i * grid.len() as u32, start))
        .collect();

    extrapolate(
        init[0],
        init[1],
        init[2],
        STEPS / grid.len() as u32 - POINTS + 1,
    )
}

fn extrapolate(mut p1: u64, mut p2: u64, mut p3: u64, times: u32) -> u64 {
    for _ in 0..times {
        let (d1, d2) = (p2 - p1, p3 - p2);
        let d3 = d2 - d1;
        let np = p3 + d3 + d2;
        p1 = p2;
        p2 = p3;
        p3 = np;
    }
    p3
}

fn find_start(grid: &[Vec<u8>]) -> (i32, i32) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            if b == b'S' {
                return (i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

fn wrap_coord(coord: i32, len: usize) -> usize {
    if coord < 0 {
        (len as i32 - 1 + ((coord + 1) % len as i32)) as _
    } else {
        (coord % len as i32) as _
    }
}

fn count_pots(grid: &Vec<Vec<u8>>, steps: u32, start: (i32, i32)) -> u64 {
    let mut seen = HashSet::from([start]);
    let mut q = VecDeque::from([(start.0, start.1, 0)]);

    let mut ans = 0;
    while let Some((i, j, dist)) = q.pop_front() {
        if (steps - dist) % 2 == 0 {
            ans += 1;
        }
        if steps == dist {
            continue;
        }

        for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if grid[wrap_coord(ni, grid.len())][wrap_coord(nj, grid[0].len())] != b'#'
                && !seen.contains(&(ni, nj))
            {
                seen.insert((ni, nj));
                q.push_back((ni, nj, dist + 1));
            }
        }
    }
    ans
}

fn main() {
    let input = read_to_string("input/21.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input, 64));
    println!("Part 2: {}", part2(&input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/21.txt").unwrap(), 6), 16);
        assert_eq!(part1(&read_to_string("examples/21.txt").unwrap(), 50), 1594);
        assert_eq!(
            part1(&read_to_string("examples/21.txt").unwrap(), 100),
            6536
        );
    }
}
