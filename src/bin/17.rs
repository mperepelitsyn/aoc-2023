use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    shortest_path(parse_input(input), 1, 3)
}

fn part2(input: &str) -> u32 {
    shortest_path(parse_input(input), 4, 10)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn dir_to_idx(dir: (i16, i16)) -> usize {
    match dir {
        (0, 1) => 1,
        (0, -1) => 2,
        (1, 0) => 3,
        (-1, 0) => 4,
        _ => 0,
    }
}

fn shortest_path(grid: Vec<Vec<u8>>, min_straight: u8, max_straight: u8) -> u32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut min_dist = vec![vec![vec![vec![u16::MAX; n]; m]; max_straight as usize + 1]; 5];
    let mut heap = BinaryHeap::from([(Reverse(0), (-1i16, -1i16), 0u8, (0i16, 0i16))]);
    min_dist[dir_to_idx((-1, -1))][0][0][0] = 0;

    let mut ans = u16::MAX;
    while let Some((Reverse(cur_dist), dir, straight, (i, j))) = heap.pop() {
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (ni, nj) = (i + di, j + dj);
            if ni < 0
                || nj < 0
                || ni >= m as i16
                || nj >= n as i16
                || (ni, nj) == (i - dir.0, j - dir.1)
            {
                continue;
            }

            if (i, j) != (0, 0) && (di, dj) != dir && straight < min_straight {
                continue;
            }
            let straight = if (di, dj) == dir { straight + 1 } else { 1 };
            if straight > max_straight {
                continue;
            }

            let new_dist = cur_dist + grid[ni as usize][nj as usize] as u16;
            if min_dist[dir_to_idx((di, dj))][straight as usize][ni as usize][nj as usize]
                <= new_dist
            {
                continue;
            }

            min_dist[dir_to_idx((di, dj))][straight as usize][ni as usize][nj as usize] = new_dist;
            heap.push((Reverse(new_dist), (di, dj), straight, (ni, nj)));
        }

        if (i, j) == (m as i16 - 1, n as i16 - 1) {
            ans = ans.min(cur_dist)
        }
    }

    ans as _
}

fn main() {
    let input = read_to_string("input/17.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/17.txt").unwrap()), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/17.txt").unwrap()), 94);
    }
}
