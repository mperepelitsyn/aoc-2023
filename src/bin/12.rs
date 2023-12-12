use std::{collections::HashMap, fs::read_to_string};

fn part1(input: &str) -> i64 {
    parse_input(input, 1)
        .map(|(rec, ctl)| dfs(rec.as_bytes(), 0, 0, 0, &ctl, &mut HashMap::new()))
        .sum()
}

fn part2(input: &str) -> i64 {
    parse_input(input, 5)
        .map(|(rec, ctl)| dfs(rec.as_bytes(), 0, 0, 0, &ctl, &mut HashMap::new()))
        .sum()
}

fn parse_input(input: &str, expand: usize) -> impl Iterator<Item = (String, Vec<u32>)> + '_ {
    input.lines().map(move |line| {
        let (rec, ctl) = line.split_once(' ').unwrap();
        let ctl = ctl.split(',').map(|s| s.parse::<u32>().unwrap());
        let mut rec = std::iter::repeat(rec)
            .take(expand)
            .collect::<Vec<_>>()
            .join("?");
        rec.push('.'); // Nice.
        let ctl: Vec<_> = std::iter::repeat_with(|| ctl.clone())
            .take(expand)
            .flatten()
            .collect();
        (rec, ctl)
    })
}

fn dfs(
    rec: &[u8],
    i: usize,
    glen: u32,
    gcount: usize,
    ctl: &[u32],
    memo: &mut HashMap<(u8, u8, u8), i64>,
) -> i64 {
    let key = (i as u8, glen as u8, gcount as u8);
    if let Some(&ret) = memo.get(&key) {
        return ret;
    }

    if i == rec.len() {
        if gcount == ctl.len() {
            return 1;
        }
        return 0;
    }

    let mut ret = 0;
    if rec[i] == b'#' {
        if gcount == ctl.len() || glen == ctl[gcount] {
            return 0;
        }
        ret += dfs(rec, i + 1, glen + 1, gcount, ctl, memo);
    } else if rec[i] == b'.' {
        if glen > 0 && glen < ctl[gcount] {
            return 0;
        }
        ret += dfs(rec, i + 1, 0, gcount + (glen > 0) as usize, ctl, memo);
    } else {
        if gcount < ctl.len() && glen < ctl[gcount] {
            ret += dfs(rec, i + 1, glen + 1, gcount, ctl, memo);
        }
        if glen == 0 || gcount < ctl.len() && glen == ctl[gcount] {
            ret += dfs(rec, i + 1, 0, gcount + (glen > 0) as usize, ctl, memo)
        };
    }
    memo.insert(key, ret);
    ret
}

fn main() {
    let input = read_to_string("input/12.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/12.txt").unwrap()), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/12.txt").unwrap()), 525152);
    }
}
