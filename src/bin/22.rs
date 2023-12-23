use std::fs::read_to_string;

fn part1(input: &str) -> usize {
    let bricks = parse_input(input);
    let adj = build_tree(bricks);
    (1..adj.len())
        .map(|i| traverse(&adj, i))
        .filter(|&reached| reached == adj.len() - 1)
        .count()
}

fn part2(input: &str) -> usize {
    let bricks = parse_input(input);
    let adj = build_tree(bricks);
    (1..adj.len())
        .map(|i| traverse(&adj, i))
        .map(|n| adj.len() - 1 - n)
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<Vec<usize>>> {
    let mut points: Vec<_> = input
        .lines()
        .map(|line| {
            let mut points: Vec<_> = line
                .split('~')
                .map(|p| {
                    p.split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
            points.sort_unstable();
            points
        })
        .collect();
    points.sort_unstable_by(|a, b| a[0][2].min(a[1][2]).cmp(&b[0][2].min(b[1][2])));
    points
}

fn get_xy_dim(bricks: &[Vec<Vec<usize>>]) -> (usize, usize) {
    let (mut x_max, mut y_max) = (usize::MIN, usize::MIN);
    bricks.iter().for_each(|p| {
        x_max = x_max.max(p[0][0]).max(p[1][0]);
        y_max = y_max.max(p[0][1]).max(p[1][1]);
    });
    (x_max + 1, y_max + 1)
}

fn build_tree(bricks: Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
    let dim = get_xy_dim(&bricks);
    let mut base = vec![vec![(0, 0); dim.1]; dim.0];
    let mut adj = vec![Vec::new(); bricks.len() + 1];

    for (cur, p) in bricks.into_iter().enumerate() {
        let max_height = base
            .iter()
            .take(p[1][0] + 1)
            .skip(p[0][0])
            .map(|row| {
                row.iter()
                    .take(p[1][1] + 1)
                    .skip(p[0][1])
                    .map(|(h, _)| *h)
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();

        base.iter_mut()
            .take(p[1][0] + 1)
            .skip(p[0][0])
            .for_each(|row| {
                row.iter_mut()
                    .take(p[1][1] + 1)
                    .skip(p[0][1])
                    .for_each(|(height, parent)| {
                        if *height == max_height && !adj[*parent].contains(&(cur + 1)) {
                            adj[*parent].push(cur + 1);
                        }
                        *height = max_height + 1 + p[1][2] - p[0][2];
                        *parent = cur + 1;
                    });
            });
    }
    adj
}

fn traverse(adj: &[Vec<usize>], skip: usize) -> usize {
    let mut q = Vec::from([0]);
    let mut seen = vec![0; adj.len()];
    seen[0] = 1;

    while let Some(cur) = q.pop() {
        for &next in adj[cur].iter() {
            if next != skip && seen[next] == 0 {
                q.push(next);
                seen[next] = 1;
            }
        }
    }
    seen.iter().sum()
}

fn main() {
    let input = read_to_string("input/22.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/22.txt").unwrap()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/22.txt").unwrap()), 7);
    }
}
