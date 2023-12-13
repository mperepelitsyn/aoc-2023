use std::fs::read_to_string;

fn part1(input: &str) -> i64 {
    solve(input, 1)
}

fn part2(input: &str) -> i64 {
    solve(input, 5)
}

fn solve(input: &str, expand: usize) -> i64 {
    parse_input(input, expand)
        .map(|(rec, ctl)| dfs(rec.as_bytes(), 0, 0, 0, &ctl, &mut create_memo(&rec, &ctl)))
        .sum()
}

fn parse_input(input: &str, expand: usize) -> impl Iterator<Item = (String, Vec<usize>)> + '_ {
    input.lines().map(move |line| {
        let (rec, ctl) = line.split_once(' ').unwrap();
        let ctl = ctl.split(',').map(|s| s.parse::<usize>().unwrap());
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
    glen: usize,
    gcount: usize,
    ctl: &[usize],
    memo: &mut Vec<Vec<Vec<i64>>>,
) -> i64 {
    if i == rec.len() {
        if gcount == ctl.len() {
            return 1;
        }
        return 0;
    }
    if memo[i][glen][gcount] != -1 {
        return memo[i][glen][gcount];
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
    memo[i][glen][gcount] = ret;
    ret
}

fn create_memo(rec: &str, ctl: &[usize]) -> Vec<Vec<Vec<i64>>> {
    vec![vec![vec![-1; ctl.len() + 1]; *ctl.iter().max().unwrap() + 1]; rec.len()]
}

fn main() {
    let input = read_to_string("input/12.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/12.txt").unwrap()), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_to_string("examples/12.txt").unwrap()), 525152);
    }
}
