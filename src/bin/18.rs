use std::fs::read_to_string;

fn part1(input: &str) -> u64 {
    let input = input.lines().map(|line| {
        let words: Vec<_> = line.split_whitespace().collect();
        (words[0].as_bytes()[0], words[1].parse::<i32>().unwrap())
    });

    get_area(input)
}

fn part2(input: &str) -> u64 {
    let input = input.lines().map(|line| {
        let words: Vec<_> = line.split_whitespace().collect();
        let dir = match words[2].as_bytes()[words[2].len() - 2] {
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            b'3' => b'U',
            _ => unreachable!(),
        };
        (dir, i32::from_str_radix(&words[2][2..7], 16).unwrap())
    });

    get_area(input)
}

fn get_area(dirs: impl Iterator<Item = (u8, i32)>) -> u64 {
    let mut vertices = Vec::new();
    let mut perim = 0;

    let (mut i, mut j) = (0, 0);
    for (dir, len) in dirs {
        perim += len;
        match dir {
            b'U' => i -= len,
            b'D' => i += len,
            b'L' => j -= len,
            b'R' => j += len,
            _ => unreachable!(),
        }
        vertices.push((i, j));
    }

    shoelace(vertices) + perim as u64 / 2 + 1
}

fn shoelace(mut vertices: Vec<(i32, i32)>) -> u64 {
    vertices.push(vertices[0]);
    vertices
        .windows(2)
        .map(|w| (w[0].0 + w[1].0) as i64 * (w[0].1 - w[1].1) as i64)
        .sum::<i64>() as u64
        / 2
}

fn main() {
    let input = read_to_string("input/18.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/18.txt").unwrap()), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&read_to_string("examples/18.txt").unwrap()),
            952408144115
        );
    }
}
