use std::fs::read_to_string;

fn part1(input: &str) -> u64 {
    let grid = parse_input(input);
    count_mass(&tilt(grid))
}

fn part2(input: &str) -> u64 {
    let mut grid = parse_input(input);
    let mut seen_grids = std::collections::HashMap::new();
    let mut seen_cycles = std::collections::HashMap::new();
    seen_grids.insert(grid.clone(), 0);

    let (mut start, mut offset, mut len) = (0, 0, 0);
    for i in 1.. {
        grid = cycle(grid);
        seen_cycles.insert(i, grid.clone());

        if let Some(&prev_i) = seen_grids.get(&grid) {
            if start == 0 {
                start = i;
            }
            if offset == 0 {
                offset = seen_grids[&grid]
            } else if offset == prev_i {
                len = i - offset;
                break;
            }
        } else {
            seen_grids.insert(grid.clone(), i);
        }
    }

    let grid = &seen_cycles[&((1_000_000_000 - start) % len + offset)];
    count_mass(grid)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect()
}

fn cycle(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for _ in 0..4 {
        grid = tilt(grid);
        grid = rotate(grid);
    }
    grid
}

fn rotate(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut grid = transpose(grid);
    for row in &mut grid {
        row.reverse()
    }
    grid
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

fn count_mass(grid: &Vec<Vec<u8>>) -> u64 {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&b| b == b'O').count() * (grid.len() - i))
        .sum::<usize>() as _
}

fn tilt(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut height = vec![0; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'O' {
                grid[i][j] = b'.';
                grid[height[j]][j] = b'O';
                height[j] += 1;
            } else if grid[i][j] == b'#' {
                height[j] = i + 1;
            }
        }
    }
    grid
}

fn main() {
    let input = read_to_string("input/14.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/14.txt").unwrap()), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/14.txt").unwrap()), 64);
    }
}
