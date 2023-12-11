use std::fs::read_to_string;

fn part1(input: &str) -> usize {
    solve(input, 2)
}

fn part2(input: &str, expand: usize) -> usize {
    solve(input, expand)
}

fn solve(input: &str, expand: usize) -> usize {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect();
    let grid = expand_rows(transpose(expand_rows(grid)));
    let gals = find_galaxies(&grid);

    (0..gals.len() - 1)
        .flat_map(|i| (i + 1..gals.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (from, to) = sort_coords(gals[i].0, gals[j].0);
            let i_dir: usize = (from + 1..=to)
                .map(|k| {
                    if grid[k][gals[i].1] == b'+' {
                        expand
                    } else {
                        1
                    }
                })
                .sum();

            let (from, to) = sort_coords(gals[i].1, gals[j].1);
            let j_dir: usize = (from + 1..=to)
                .map(|k| {
                    if grid[gals[i].0][k] == b'+' {
                        expand
                    } else {
                        1
                    }
                })
                .sum();

            i_dir + j_dir
        })
        .sum()
}

fn transpose(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut ret = vec![vec![0; grid.len()]; grid[0].len()];
    for (i, row) in grid.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            ret[j][i] = c;
        }
    }
    ret
}

fn expand_rows(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for row in grid.iter_mut() {
        if row.iter().all(|&b| b == b'.' || b == b'+') {
            row.iter_mut().for_each(|b| *b = b'+')
        }
    }
    grid
}

fn find_galaxies(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &b)| b == b'#')
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

fn sort_coords(i: usize, j: usize) -> (usize, usize) {
    if i < j {
        (i, j)
    } else {
        (j, i)
    }
}

fn main() {
    let input = read_to_string("input/11.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/11.txt").unwrap()), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&read_to_string("examples/11.txt").unwrap(), 100),
            8410
        );
    }
}
