use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn part1(input: &str) -> i32 {
    solve(input, false)
}

fn part2(input: &str) -> i32 {
    solve(input, true)
}

fn solve(input: &str, can_hike: bool) -> i32 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let (adj, target) = compress(grid, can_hike);
    dfs(&adj, 0, target, &mut vec![false; adj.len()])
}

fn compress(grid: Vec<Vec<u8>>, can_hike: bool) -> (Vec<Vec<(usize, u32)>>, usize) {
    let mut map = HashMap::from([((0, 1), 0)]);
    let mut edges = HashSet::new();
    let mut q = Vec::from([((0, 1), 0, DOWN, !can_hike, 0)]);
    let target = (grid.len() as i32 - 1, grid[0].len() as i32 - 2);

    while let Some(((i, j), mut parent, dir, mut directed, mut dist)) = q.pop() {
        if !can_hike && grid[i as usize][j as usize] != b'.' {
            let ndir = get_dir(grid[i as usize][j as usize]);
            q.push(((i + ndir.0, j + ndir.1), parent, ndir, true, dist + 1));
            continue;
        }

        let next: Vec<_> = [UP, DOWN, LEFT, RIGHT]
            .into_iter()
            .filter_map(|ndir| {
                if ndir == (-dir.0, -dir.1) {
                    return None;
                }
                let (ni, nj) = (i + ndir.0, j + ndir.1);
                if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid[0].len() as i32 {
                    return None;
                }
                let b = grid[ni as usize][nj as usize];
                if b == b'#' || !can_hike && b != b'.' && (-ndir.0, -ndir.1) == get_dir(b) {
                    return None;
                }

                Some((ni, nj, ndir))
            })
            .collect();

        if next.len() > 1 || (i, j) == target {
            let len = map.len();
            let child = *map.entry((i, j)).or_insert(len);
            let edge = if parent < child || directed {
                (parent, child, dist, directed)
            } else {
                (child, parent, dist, directed)
            };
            if edges.contains(&edge) {
                continue;
            }

            edges.insert(edge);
            parent = child;
            dist = 0;
            directed = false;
        }

        next.into_iter().for_each(|(ni, nj, ndir)| {
            q.push(((ni, nj), parent, ndir, directed, dist + 1));
        })
    }

    let mut adj = vec![vec![]; edges.len() + 1];
    edges.into_iter().for_each(|(from, to, dist, directed)| {
        adj[from].push((to, dist));
        if !directed {
            adj[to].push((from, dist));
        }
    });

    (adj, map[&target])
}

fn dfs(adj: &[Vec<(usize, u32)>], cur: usize, target: usize, seen: &mut [bool]) -> i32 {
    seen[cur] = true;
    let mut ret = if cur == target { 0 } else { -1_000_000 };
    for &(next, dist) in adj[cur].iter() {
        if seen[next] {
            continue;
        }
        ret = ret.max(dist as i32 + dfs(adj, next, target, seen));
    }
    seen[cur] = false;
    ret
}

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

fn get_dir(b: u8) -> (i32, i32) {
    match b {
        b'>' => RIGHT,
        b'<' => LEFT,
        b'^' => UP,
        b'v' => DOWN,
        _ => unreachable!(),
    }
}

fn main() {
    let input = read_to_string("input/23.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/23.txt").unwrap()), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/23.txt").unwrap()), 154);
    }
}
