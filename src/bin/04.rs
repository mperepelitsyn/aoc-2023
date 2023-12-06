fn part1(input: &str) -> i32 {
    parse_input(input)
        .filter(|&c| c > 0)
        .map(|c| 2i32.pow(c as u32 - 1))
        .sum()
}

fn part2(input: &str) -> i32 {
    let cards = parse_input(input).collect::<Vec<_>>();
    let mut dp = vec![1; cards.len()];
    for i in (0..dp.len()).rev() {
        dp[i] = 1 + dp[i + 1..].iter().take(cards[i]).sum::<i32>();
    }
    dp.into_iter().sum()
}

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        line.split_once(": ")
            .unwrap()
            .1
            .split_once(" | ")
            .map(|(win, nums)| {
                let mut seen = [false; 100];
                win.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .for_each(|n| seen[n] = true);
                nums.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .filter(|&n| seen[n])
                    .count()
            })
            .unwrap()
    })
}

fn main() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/04.txt").unwrap();
        assert_eq!(part2(&input), 30);
    }
}
