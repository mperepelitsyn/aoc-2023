use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let (moves, map) = parse_input(input);
    count_steps("AAA", moves, &map, |s| s == "ZZZ")
}

fn part2(input: &str) -> i64 {
    let (moves, map) = parse_input(input);
    let start_nodes = map.keys().filter(|node| node.ends_with('A'));
    start_nodes
        .map(|node| count_steps(node, moves, &map, |node| node.ends_with('Z')) as i64)
        .fold(1, num::integer::lcm)
}

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let moves = input.lines().next().unwrap();
    let mut map = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let (root, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();
        map.insert(root, (left, right));
    });
    (moves, map)
}

fn count_steps<F>(start_node: &str, moves: &str, map: &HashMap<&str, (&str, &str)>, stop: F) -> i32
where
    F: Fn(&str) -> bool,
{
    let mut node = start_node;
    let mut steps = 0;
    for dir in moves.bytes().cycle() {
        if stop(node) {
            break;
        }
        if dir == b'L' {
            node = map[node].0;
        } else {
            node = map[node].1;
        }
        steps += 1;
    }
    steps
}

fn main() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/08-1.txt").unwrap();
        assert_eq!(part1(&input), 2);

        let input = std::fs::read_to_string("examples/08-2.txt").unwrap();
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/08-3.txt").unwrap();
        assert_eq!(part2(&input), 6);
    }
}
