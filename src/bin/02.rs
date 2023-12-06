fn part1(input: &str) -> u32 {
    let total = (12, 13, 14);
    parse_input(input)
        .into_iter()
        .enumerate()
        .filter(|(_, game)| game.0 <= total.0 && game.1 <= total.1 && game.2 <= total.2)
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_input(input)
        .into_iter()
        .map(|rgb| rgb.0 * rgb.1 * rgb.2)
        .sum()
}

fn parse_input(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut rgb = (0, 0, 0);
            line.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .for_each(|turn| {
                    turn.split(", ").for_each(|cube| {
                        cube.split_whitespace()
                            .collect::<Vec<_>>()
                            .chunks(2)
                            .for_each(|c| {
                                let n = c[0].parse::<u32>().unwrap();
                                match c[1].as_bytes()[0] {
                                    b'r' => rgb.0 = rgb.0.max(n),
                                    b'g' => rgb.1 = rgb.1.max(n),
                                    b'b' => rgb.2 = rgb.2.max(n),
                                    _ => unreachable!(),
                                }
                            });
                    });
                });
            rgb
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/02.txt").unwrap();
        assert_eq!(part2(&input), 2286);
    }
}
