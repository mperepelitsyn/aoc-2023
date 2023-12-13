fn part1(input: &str) -> i32 {
    solve(input, "23456789TJQKA", false)
}

fn part2(input: &str) -> i32 {
    solve(input, "J23456789TQKA", true)
}

fn solve(input: &str, ordering: &str, use_joker: bool) -> i32 {
    let mut ord = [0; 128];
    ordering
        .bytes()
        .enumerate()
        .for_each(|(i, b)| ord[b as usize] = i as i8);

    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand: Vec<_> = std::iter::once(get_hand_type(hand, use_joker))
                .chain(hand.bytes().map(|b| ord[b as usize]))
                .collect();
            (hand, bid.parse::<i32>().unwrap())
        })
        .collect();

    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as i32 + 1) * bid)
        .sum()
}

fn get_hand_type(hand: &str, use_joker: bool) -> i8 {
    let mut freq = [0i8; 128];
    let mut jokers = 0;
    hand.bytes().for_each(|b| {
        if b == b'J' && use_joker {
            jokers += 1;
        } else {
            freq[b as usize] += 1
        }
    });
    let mut freq = freq.into_iter().filter(|&c| c > 0).collect::<Vec<_>>();
    freq.sort_unstable();

    if use_joker {
        if let Some(f) = freq.last_mut() {
            *f += jokers;
        } else {
            freq.push(jokers);
        }
    }

    match freq.len() {
        5 => 1,
        4 => 2,
        3 if freq[1] == 2 => 3,
        3 if freq[2] == 3 => 4,
        2 if freq[0] == 2 => 5,
        2 if freq[0] == 1 => 6,
        1 => 7,
        _ => unreachable!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
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
        let input = std::fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/07.txt").unwrap();
        assert_eq!(part2(&input), 5905);
    }
}
