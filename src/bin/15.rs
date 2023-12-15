use std::fs::read_to_string;

fn part1(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    input.trim().split(',').for_each(|s| {
        let op = s.bytes().position(|b| !b.is_ascii_alphabetic()).unwrap();
        if s.as_bytes()[op] == b'=' {
            let (label, b, focal) = (
                &s[..op],
                hash(&s[..op]),
                s[op + 1..].parse::<usize>().unwrap(),
            );
            if let Some(pos) = boxes[b].iter().position(|t| t.0 == label) {
                boxes[b][pos].1 = focal;
            } else {
                boxes[b].push((label, focal));
            }
        } else {
            let (label, b) = (&s[..op], hash(&s[..op]));
            if let Some(pos) = boxes[b].iter().position(|t| t.0 == label) {
                boxes[b].remove(pos);
            }
        }
    });

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(|(j, (_, focal))| (i + 1) * (j + 1) * focal)
                .sum::<usize>()
        })
        .sum()
}

fn hash(str: &str) -> usize {
    str.bytes().fold(0, |acc, b| (acc + b as usize) * 17 % 256)
}

fn main() {
    let input = read_to_string("input/15.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/15.txt").unwrap()), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/15.txt").unwrap()), 145);
    }
}
