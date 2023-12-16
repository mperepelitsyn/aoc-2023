use std::collections::HashSet;
use std::convert::identity;
use std::fs::read_to_string;

fn part1(input: &str) -> u32 {
    let grid: Vec<_> = input.lines().map(|line| line.bytes().collect()).collect();
    traverse(&grid, (0, 0), (0, 1))
}

fn part2(input: &str) -> u32 {
    let grid: Vec<_> = input.lines().map(|line| line.bytes().collect()).collect();
    let mut ret = 0;
    for i in 0..grid.len() {
        ret = ret.max(traverse(&grid, (i as i8, 0), (0, 1)));
        ret = ret.max(traverse(&grid, (i as i8, grid[0].len() as i8 - 1), (0, -1)));
    }
    for j in 0..grid[0].len() {
        ret = ret.max(traverse(&grid, (0, j as i8), (1, 0)));
        ret = ret.max(traverse(&grid, (grid.len() as i8 - 1, j as i8), (1, 0)));
    }
    ret
}

fn traverse(grid: &Vec<Vec<u8>>, start: (i8, i8), dir: (i8, i8)) -> u32 {
    let mut stack = vec![(start, dir)];
    let mut seen = HashSet::from([(start, dir)]);

    while let Some((pos, dir)) = stack.pop() {
        let next_dir = match grid[pos.0 as usize][pos.1 as usize] {
            b'|' if dir == (0, 1) || dir == (0, -1) => [Some((-1, 0)), Some((1, 0))],
            b'-' if dir == (1, 0) || dir == (-1, 0) => [Some((0, 1)), Some((0, -1))],
            b'\\' => [Some((dir.1, dir.0)), None],
            b'/' => [Some((-dir.1, -dir.0)), None],
            _ => [Some(dir), None],
        };
        #[allow(clippy::filter_map_identity)]
        next_dir.into_iter().filter_map(identity).for_each(|dir| {
            let pos = (pos.0 + dir.0, pos.1 + dir.1);
            if pos.0 >= 0
                && pos.1 >= 0
                && pos.0 < grid.len() as i8
                && pos.1 < grid[0].len() as i8
                && !seen.contains(&(pos, dir))
            {
                stack.push((pos, dir));
                seen.insert((pos, dir));
            }
        });
    }

    let uniq: HashSet<_> = seen.iter().map(|(pos, _)| pos).collect();
    uniq.len() as _
}

fn main() {
    let input = read_to_string("input/16.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/16.txt").unwrap()), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/16.txt").unwrap()), 51);
    }
}
