use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    solve(input, 0)
}

fn part2(input: &str) -> u32 {
    solve(input, 1)
}

fn solve(input: &str, err: u32) -> u32 {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.bytes().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|grid| find_line(grid, err))
        .map(|(v, h)| v + h * 100)
        .sum()
}

fn cmp(row1: &[u8], row2: &[u8]) -> u32 {
    row1.iter().zip(row2.iter()).filter(|(a, b)| a != b).count() as _
}

fn find_hor(grid: &[Vec<u8>], err: u32) -> u32 {
    for i in 1..grid.len() {
        let mut cur_err = 0;
        let mut matches = 0;
        for (j, k) in (0..i).rev().zip(i..grid.len()) {
            cur_err += cmp(&grid[j], &grid[k]);
            if cur_err <= err {
                matches += 1
            } else {
                break;
            }
        }
        if cur_err == err && i.min(grid.len() - i) == matches {
            return i as _;
        }
    }
    0
}

fn find_line(grid: Vec<Vec<u8>>, err: u32) -> (u32, u32) {
    let hor = find_hor(&grid, err);
    if hor > 0 {
        return (0, hor);
    }
    let vert = find_hor(&transpose(grid), err);
    (vert, 0)
}

fn transpose(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut ret = vec![vec![0; grid.len()]; grid[0].len()];
    for (i, row) in grid.into_iter().enumerate() {
        for (j, b) in row.into_iter().enumerate() {
            ret[j][i] = b;
        }
    }
    ret
}

fn main() {
    let input = read_to_string("input/13.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/13.txt").unwrap()), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/13.txt").unwrap()), 400);
    }
}
