fn part1(input: &str) -> i32 {
    solve(input, get_next)
}

fn part2(input: &str) -> i32 {
    solve(input, get_prev)
}

fn solve<F>(input: &str, cb: F) -> i32
where
    F: Fn(&[i32]) -> i32,
{
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            cb(&nums)
        })
        .sum()
}

fn get_next(nums: &[i32]) -> i32 {
    if nums.iter().all(|e| e == &0) {
        return 0;
    }
    let slope: Vec<_> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    nums.last().unwrap() + get_next(&slope)
}

fn get_prev(nums: &[i32]) -> i32 {
    if nums.iter().all(|e| e == &0) {
        return 0;
    }
    let slope: Vec<_> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    nums[0] - get_prev(&slope)
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/09.txt").unwrap();
        assert_eq!(part2(&input), 2);
    }
}
