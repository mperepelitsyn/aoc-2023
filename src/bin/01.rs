fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| get_digit(&line[i..], false))
                .unwrap();
            let last = (0..line.len())
                .rev()
                .find_map(|i| get_digit(&line[i..], false))
                .unwrap();
            first * 10 + last
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|i| get_digit(&line[i..], true))
                .unwrap();
            let last = (0..line.len())
                .rev()
                .find_map(|i| get_digit(&line[i..], true))
                .unwrap();
            first * 10 + last
        })
        .sum()
}

const DICT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit(s: &str, chars: bool) -> Option<u32> {
    if s.as_bytes()[0].is_ascii_digit() {
        return Some((s.as_bytes()[0] - b'0') as _);
    }
    if !chars {
        return None;
    }
    return DICT
        .iter()
        .enumerate()
        .filter(|w| w.1.len() <= s.len())
        .find(|w| &s[..w.1.len()] == *w.1)
        .map(|w| (w.0 + 1) as u32);
}

fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/01-1.txt").unwrap();
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/01-2.txt").unwrap();
        assert_eq!(part2(&input), 281);
    }
}
