use std::fs::read_to_string;

fn part1(input: &str) -> i32 {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    let mut steps = 0;
    traverse_loop(&grid, |_| steps += 1);
    steps / 2
}

fn part2(input: &str) -> i32 {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut is_loop = vec![vec![false; grid[0].len()]; grid.len()];
    traverse_loop(&grid, |(i, j)| is_loop[i][j] = true);

    let mut tiles = 0;
    let mut splits = 0;
    let (mut i, mut j) = (0, 0);
    while i < grid.len() {
        while j < grid[0].len() {
            if is_loop[i][j] {
                match grid[i][j] {
                    b'|' => splits += 1,
                    b'F' => {
                        j = follow_tube(grid[i], j);
                        if grid[i][j] == b'J' {
                            splits += 1
                        }
                    }
                    b'L' => {
                        j = follow_tube(grid[i], j);
                        if grid[i][j] == b'7' {
                            splits += 1
                        }
                    }
                    _ => (),
                }
            } else if splits % 2 != 0 {
                tiles += 1
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }

    tiles
}

fn traverse_loop<F>(grid: &[&[u8]], mut cb: F)
where
    F: FnMut((usize, usize)),
{
    let start = find_start(grid);
    let mut pos = find_tube(grid);

    cb(pos);
    let mut prev = start;
    while pos != start {
        let tmp = pos;
        pos = advance(grid, pos, prev);
        prev = tmp;
        cb(pos);
    }
}

fn advance(grid: &[&[u8]], (i, j): (usize, usize), (pi, pj): (usize, usize)) -> (usize, usize) {
    match grid[i][j] {
        b'|' => {
            if i + 1 == pi {
                (i - 1, j)
            } else {
                (i + 1, j)
            }
        }
        b'-' => {
            if j + 1 == pj {
                (i, j - 1)
            } else {
                (i, j + 1)
            }
        }
        b'F' => {
            if j + 1 == pj {
                (i + 1, j)
            } else {
                (i, j + 1)
            }
        }
        b'7' => {
            if j - 1 == pj {
                (i + 1, j)
            } else {
                (i, j - 1)
            }
        }
        b'J' => {
            if i - 1 == pi {
                (i, j - 1)
            } else {
                (i - 1, j)
            }
        }
        b'L' => {
            if i - 1 == pi {
                (i, j + 1)
            } else {
                (i - 1, j)
            }
        }
        _ => unreachable!(),
    }
}

fn find_tube(grid: &[&[u8]]) -> (usize, usize) {
    let (i, j) = find_start(grid);

    for (dir, tiles) in [
        ((-1, 0), "|F7"),
        ((1, 0), "|LJ"),
        ((0, -1), "LF-"),
        ((0, 1), "-J7"),
    ] {
        let (ni, nj) = (((i as i32) + dir.0) as usize, ((j as i32) + dir.1) as usize);
        if ni >= grid.len() || nj >= grid[0].len() {
            continue;
        }
        if tiles.as_bytes().contains(&grid[ni][nj]) {
            return (ni, nj);
        }
    }
    unreachable!()
}

fn find_start(grid: &[&[u8]]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            if b == b'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn follow_tube(row: &[u8], j: usize) -> usize {
    j + row[j + 1..].iter().take_while(|&&b| b == b'-').count() + 1
}

fn main() {
    let input = read_to_string("input/10.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/10-1.txt").unwrap()), 4);
        assert_eq!(part1(&read_to_string("examples/10-2.txt").unwrap()), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/10-3.txt").unwrap()), 4);
        assert_eq!(part2(&read_to_string("examples/10-4.txt").unwrap()), 4);
        assert_eq!(part2(&read_to_string("examples/10-5.txt").unwrap()), 8);
        assert_eq!(part2(&read_to_string("examples/10-6.txt").unwrap()), 10);
    }
}
