use std::fs::read_to_string;

fn part1(input: &str) -> u64 {
    let grid: Vec<_> = input.lines().map(|line| line.bytes().collect::<Vec<_>>()).collect();
    let grid = tilt(grid);
    // for row in &grid {
    //     println!("{}", String::from_utf8(row.clone()).unwrap());
    // }
    count_mass(&grid)
}

fn part2(input: &str) -> u64 {
    let mut grid: Vec<_> = input.lines().map(|line| line.bytes().collect::<Vec<_>>()).collect();
    let mut map = std::collections::HashMap::new();
    let mut cache = std::collections::HashMap::new();
    map.insert(grid.clone(), 0);
    let mut offset = 0;
    let mut start = 0;
    let mut len = 0;

    for i in 1..200 {
        let t= std::time::Instant::now();
        grid = cycle(grid);
        println!("elapsed: {:?}", t.elapsed());

        cache.insert(i, grid.clone());

        if map.contains_key(&grid) {
            if offset == 0 { offset = i; }
            if start == 0 { start = map[&grid] }
            else if start == map[&grid] {
                println!("found cycle at {i}");
                len = i - offset;
                break;
            }

            println!("already seen {i} -> {}", map[&grid]);
        } else {
            map.insert(grid.clone(), i);
        }

        // for row in &grid {
        //     println!("{}", String::from_utf8(row.clone()).unwrap());
        // }
        // println!("\n+++++++++++++++++++++++++++++++++++++++++++++++++\n");
    }

    println!("off={offset} start={start} len={len}");
    let grid = cache[&(((1_000_000_000 - offset) % len) as usize + start as usize)].clone();
    count_mass(&grid)
}

fn cycle(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let grid = tilt(grid);
    let grid = rotate(grid);
    let grid = tilt(grid);
    let grid = rotate(grid);
    let grid = tilt(grid);
    let grid = rotate(grid);
    let grid = tilt(grid);
    let grid = rotate(grid);
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
    grid.iter().enumerate().map(|(i, row)| {
        row.iter().filter(|&&b| b == b'O').count() * (grid.len() - i)
    }).sum::<usize>() as _
}

fn tilt(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut h = vec![0; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'O' {
                grid[i][j] = b'.';
                grid[h[j]][j] = b'O';
                h[j] += 1;
            } else if grid[i][j] == b'#' {
                h[j] = i + 1;
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
