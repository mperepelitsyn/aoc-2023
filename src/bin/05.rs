fn part1(input: &str) -> i64 {
    let (seeds, maps) = (parse_seeds(input), parse_maps(input));

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                if let Some((_, _, offset)) =
                    map.iter().find(|(lo, hi, _)| lo <= &seed && &seed <= hi)
                {
                    seed + offset
                } else {
                    seed
                }
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let seeds: Vec<_> = parse_seeds(input)
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect();
    let maps = parse_maps(input);

    maps.into_iter()
        .fold(seeds, |seeds, map| {
            seeds
                .into_iter()
                .flat_map(|(slo, shi)| {
                    let mut out = vec![(slo, shi)];
                    map.iter()
                        .map(|(mlo, mhi, offset)| (slo.max(*mlo), shi.min(*mhi), offset))
                        .filter(|(ilo, ihi, _)| ilo < ihi)
                        .for_each(|(ilo, ihi, offset)| {
                            let (slo, shi) = out.pop().unwrap();
                            if slo < ilo {
                                out.push((slo, ilo - 1))
                            }
                            out.push((ilo + offset, ihi + offset));
                            if ihi < shi {
                                out.push((ihi + 1, shi))
                            }
                        });
                    out
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap()
        .0
}

fn parse_seeds(input: &str) -> Vec<i64> {
    input
        .split_once('\n')
        .unwrap()
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_maps(input: &str) -> Vec<Vec<(i64, i64, i64)>> {
    let input: Vec<_> = input.lines().collect();

    input[2..]
        .split(|line| line.is_empty())
        .map(|block| {
            let mut map: Vec<_> = block
                .iter()
                .skip(1)
                .map(|line| {
                    let nums: Vec<_> = line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    (nums[1], nums[1] + nums[2] - 1, nums[0] - nums[1])
                })
                .collect();
            map.sort_unstable_by_key(|t| t.1);
            map
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("examples/05.txt").unwrap();
        assert_eq!(part2(&input), 46);
    }
}
