fn part1(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    get_digits(lines[0])
        .zip(get_digits(lines[1]))
        .fold(1, |acc, (time, dist)| {
            let wins = (1..time)
                .map(|t| t * (time - t))
                .filter(|&s| s > dist)
                .count() as i64;
            acc * wins
        })
}

fn part2(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let (time, dist) = (get_digit_concat(lines[0]), get_digit_concat(lines[1]));

    let (mut lo, mut hi) = (1, time / 2);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mid * (time - mid) > dist {
            hi = mid
        } else {
            lo = mid + 1
        }
    }
    time + 1 - lo * 2
}

fn get_digits(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
}

fn get_digit_concat(line: &str) -> i64 {
    line.split_whitespace().skip(1).fold(0, |acc, s| {
        acc * 10i64.pow(s.len() as u32) + s.parse::<i64>().unwrap()
    })
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/06.txt").unwrap();
        assert_eq!(part2(&input), 71503);
    }
}
