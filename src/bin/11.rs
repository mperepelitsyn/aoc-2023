use std::fs::read_to_string;

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str, dist: i32) -> i64 {
    solve(input, dist)
}

fn solve(input: &str, dist: i32) -> i64 {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect();
    let (rows, cols) = get_expansion_coords(&grid);
    let gals = expand_galaxies(find_galaxies(&grid), &rows, &cols, dist);

    (0..gals.len() - 1)
        .flat_map(|i| (i + 1..gals.len()).map(move |j| (i, j)))
        .map(|(i, j)| gals[i].0.abs_diff(gals[j].0) + gals[i].1.abs_diff(gals[j].1))
        .sum::<usize>() as _
}

fn get_expansion_coords(grid: &[Vec<u8>]) -> (Vec<usize>, Vec<usize>) {
    let rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&b| b == b'.'))
        .map(|(i, _)| i)
        .collect();
    let cols = (0..grid[0].len())
        .filter(|&j| (0..grid.len()).all(|i| grid[i][j] == b'.'))
        .collect();

    (rows, cols)
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

fn expand_galaxies(
    mut gals: Vec<(usize, usize)>,
    rows: &[usize],
    cols: &[usize],
    dist: i32,
) -> Vec<(usize, usize)> {
    gals.iter_mut().for_each(|(i, j)| {
        [(i, rows), (j, cols)].into_iter().for_each(|(p, coords)| {
            let times = coords.binary_search(p).unwrap_err();
            *p += times * dist as usize - times;
        });
    });
    gals
}

fn main() {
    let input = read_to_string("input/11.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 1_000_000));
    println!("Time: {:?}", start.elapsed());
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
