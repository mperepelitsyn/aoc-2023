fn part1(input: &str) -> i32 {
    let g = parse_input(input);

    g.iter()
        .enumerate()
        .map(|(i, row)| {
            parse_numbers(row)
                .filter(|&(_, j, len)| {
                    get_border(&g, i, j, len).any(|(_, _, b)| !b.is_ascii_digit() && b != &b'.')
                })
                .map(|(n, _, _)| n)
                .sum::<i32>()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let g = parse_input(input);
    let mut map = std::collections::HashMap::new();

    for (i, row) in g.iter().enumerate() {
        parse_numbers(row).for_each(|(n, j, len)| {
            get_border(&g, i, j, len)
                .filter(|t| t.2 == &b'*')
                .for_each(|(i, j, _)| {
                    map.entry((i, j)).or_insert(Vec::new()).push(n);
                });
        });
    }

    map.values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

fn parse_numbers(row: &[u8]) -> impl Iterator<Item = (i32, usize, usize)> + '_ {
    NumberParser {
        iter: row.iter().copied().enumerate(),
    }
}

struct NumberParser<I> {
    iter: I,
}

impl<I: Iterator<Item = (usize, u8)>> Iterator for NumberParser<I> {
    type Item = (i32, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((j, b)) = self.iter.next() {
            if b.is_ascii_digit() {
                let (n, len) = self
                    .iter
                    .by_ref()
                    .take_while(|(_, b)| b.is_ascii_digit())
                    .fold(((b - b'0') as i32, 1), |(n, len), (_, b)| {
                        (n * 10 + (b - b'0') as i32, len + 1)
                    });
                return Some((n, j, len));
            }
        }
        None
    }
}

fn get_border(
    grid: &Vec<Vec<u8>>,
    i: usize,
    j: usize,
    len: usize,
) -> impl Iterator<Item = (usize, usize, &u8)> {
    let (m, n) = (grid.len(), grid[0].len());
    (j.saturating_sub(1)..n.min(j + len + 1))
        .flat_map(move |j| {
            [i.wrapping_sub(1), i + 1]
                .into_iter()
                .filter(move |&i| i < m)
                .map(move |i| (i, j))
        })
        .chain(
            [j.wrapping_sub(1), j + len]
                .into_iter()
                .filter(move |&j| j < n)
                .map(move |j| (i, j)),
        )
        .map(|(i, j)| (i, j, &grid[i][j]))
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/03.txt").unwrap();
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/03.txt").unwrap();
        assert_eq!(part2(&input), 467835);
    }
}
