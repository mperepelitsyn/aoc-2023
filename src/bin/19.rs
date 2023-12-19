use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

fn part1(input: &str) -> i32 {
    let (block1, block2) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(block1);
    let parts = parse_parts(block2);

    filter_parts(&workflows, &parts)
        .map(|i| parts[i].iter().sum::<i32>())
        .sum::<i32>()
}

fn part2(input: &str) -> u64 {
    let (block1, _) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(block1);
    total_combs(&workflows)
}

fn parse_parts(input: &str) -> Vec<Vec<i32>> {
    let parts: Vec<_> = input
        .lines()
        .map(|line| {
            line[..line.len() - 1]
                .split(',')
                .flat_map(|s| s.split('='))
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect();
    parts
}

enum Rule<'a> {
    Cond(usize, Ordering, i32, &'a str),
    Goto(&'a str),
}

fn parse_workflows(input: &str) -> HashMap<&str, Vec<Rule>> {
    let workflows: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rules: Vec<_> = rest
                .split(',')
                .map(|rule| {
                    if let Some(goto) = rule.strip_suffix('}') {
                        Rule::Goto(goto)
                    } else {
                        let idx = match rule.as_bytes()[0] {
                            b'x' => 0,
                            b'm' => 1,
                            b'a' => 2,
                            b's' => 3,
                            _ => unreachable!(),
                        };
                        let cond = if rule.as_bytes()[1] == b'>' {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        };
                        let (num, goto) = rule[2..].split_once(':').unwrap();
                        Rule::Cond(idx, cond, num.parse().unwrap(), goto)
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();
    workflows
}

fn filter_parts<'a>(
    wfs: &'a HashMap<&str, Vec<Rule>>,
    parts: &'a [Vec<i32>],
) -> impl Iterator<Item = usize> + 'a {
    parts.iter().enumerate().filter_map(|(i, rating)| {
        let mut wf = "in";
        while wf != "A" && wf != "R" {
            for rule in wfs[wf].iter() {
                match rule {
                    Rule::Cond(idx, ord, num, goto) => {
                        if rating[*idx].cmp(num) == *ord {
                            wf = goto;
                            break;
                        }
                    }
                    Rule::Goto(goto) => {
                        wf = goto;
                        break;
                    }
                }
            }
        }
        if wf == "A" {
            Some(i)
        } else {
            None
        }
    })
}

fn total_combs(wfs: &HashMap<&str, Vec<Rule>>) -> u64 {
    let mut ans = 0;
    backtrack(wfs, "in", 0, &mut vec![vec![(1, 4000)]; 4], &mut ans);
    ans
}

fn backtrack(
    wfs: &HashMap<&str, Vec<Rule>>,
    wf: &str,
    i: usize,
    cur: &mut Vec<Vec<(i32, i32)>>,
    ans: &mut u64,
) {
    if wf == "A" {
        *ans += count_combs(cur.clone());
        return;
    }
    if wf == "R" {
        return;
    }

    let rules = &wfs[wf];
    if i == rules.len() {
        return;
    }

    match rules[i] {
        Rule::Cond(idx, ord, num, goto) => {
            cur[idx].push(cond_to_range(ord, num));
            backtrack(wfs, goto, 0, cur, ans);
            cur[idx].pop();

            let range = match ord {
                Ordering::Less => cond_to_range(Ordering::Greater, num - 1),
                Ordering::Greater => cond_to_range(Ordering::Less, num + 1),
                _ => unreachable!(),
            };
            cur[idx].push(range);
            backtrack(wfs, wf, i + 1, cur, ans);
            cur[idx].pop();
        }
        Rule::Goto(goto) => backtrack(wfs, goto, 0, cur, ans),
    }
}

fn cond_to_range(ord: Ordering, num: i32) -> (i32, i32) {
    match ord {
        Ordering::Less => (1, num - 1),
        Ordering::Greater => (num + 1, 4000),
        _ => unreachable!(),
    }
}

fn count_combs(mut chain: Vec<Vec<(i32, i32)>>) -> u64 {
    chain
        .iter_mut()
        .map(|ranges| {
            combine_ranges(ranges);
            ranges.iter().map(|(lo, hi)| hi - lo + 1).sum::<i32>() as u64
        })
        .product()
}

fn combine_ranges(ranges: &mut Vec<(i32, i32)>) {
    ranges.sort_unstable();

    let mut j = 0;
    for i in 1..ranges.len() {
        if ranges[j].1 >= ranges[i].0 {
            ranges[j] = (ranges[j].0.max(ranges[i].0), ranges[j].1.min(ranges[i].1));
        } else {
            j += 1;
            ranges.swap(i, j);
        }
    }
    ranges.truncate(j + 1);
}

fn main() {
    let input = read_to_string("input/19.txt").unwrap();
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
        assert_eq!(part1(&read_to_string("examples/19.txt").unwrap()), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&read_to_string("examples/19.txt").unwrap()),
            167409079868000
        );
    }
}
