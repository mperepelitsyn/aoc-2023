use crate::Module::{Broadcaster, Conjunction, FlipFlop, Other};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Clone)]
enum Module {
    Broadcaster(Vec<usize>),
    FlipFlop(Vec<usize>, Vec<usize>, u32),
    Conjunction(Vec<usize>, Vec<usize>, u64),
    Other(Vec<usize>),
}

fn part1(input: &str) -> u64 {
    let (names, modules) = parse_modules(input);
    let (lo, hi, _) = press_button(modules, names["broadcaster"], None, 1000);
    lo * hi
}

fn part2(input: &str) -> u64 {
    let (names, modules) = parse_modules(input);
    let start = names["broadcaster"];
    let Other(indeg) = &modules[names["rx"]] else {
        unreachable!()
    };
    let rx_parent = indeg[0];
    let Conjunction(indeg, ..) = &modules[rx_parent] else {
        unreachable!()
    };

    indeg
        .iter()
        .map(|&from| {
            press_button(
                modules.clone(),
                start,
                Some((from, rx_parent, 1)),
                1_000_000_000_000,
            )
        })
        .fold(1, |acc, (_, _, times)| num::integer::lcm(acc, times))
}

fn parse_input(input: &str) -> impl Iterator<Item = (u8, &str, Vec<&str>)> {
    input.lines().map(|line| {
        let (src, dst) = line.split_once(" -> ").unwrap();
        let dst: Vec<_> = dst.split(", ").collect();
        let (module, name) = if src.starts_with('&') || src.starts_with('%') {
            (src.as_bytes()[0], &src[1..])
        } else {
            (if src == "broadcaster" { b'b' } else { b'o' }, src)
        };
        (module, name, dst)
    })
}

fn parse_modules(input: &str) -> (HashMap<&str, usize>, Vec<Module>) {
    let mut names = HashMap::new();
    let mut dsts = Vec::new();
    parse_input(input).for_each(|(_, name, dst)| {
        names.insert(name, names.len());
        dsts.extend(dst);
    });

    let mut other = HashSet::new();
    for dst in dsts {
        if !names.contains_key(dst) {
            other.insert(dst);
            names.insert(dst, names.len());
        }
    }

    let mut types = vec![0; names.len()];
    let mut indeg = vec![vec![]; names.len()];
    let mut outdeg = vec![vec![]; names.len()];
    parse_input(input).for_each(|(module, name, dst)| {
        let src = names[name];
        types[src] = module;
        dst.iter().map(|dst| names[dst]).for_each(|dst| {
            outdeg[src].push(dst);
            indeg[dst].push(src);
        });
    });

    let modules: Vec<_> = types
        .into_iter()
        .zip(indeg)
        .zip(outdeg)
        .map(|((type_, indeg), outdeg)| match type_ {
            b'%' => FlipFlop(indeg, outdeg, 0),
            b'&' => Conjunction(indeg, outdeg, 0),
            b'b' => Broadcaster(outdeg),
            _ => Other(indeg),
        })
        .collect();

    (names, modules)
}

fn press_button(
    mut modules: Vec<Module>,
    start: usize,
    target: Option<(usize, usize, u32)>,
    times: u64,
) -> (u64, u64, u64) {
    let mut pulses = [0, 0];

    for time in 1..=times {
        let mut q = VecDeque::from([(0, start, 0)]);

        while let Some((from, to, pulse)) = q.pop_front() {
            pulses[pulse as usize] += 1;
            if let Some(target) = target {
                if target == (from, to, pulse) {
                    return (pulses[0], pulses[1], time);
                };
            }

            match modules.get_mut(to).unwrap() {
                FlipFlop(_, dsts, state) => {
                    if pulse == 0 {
                        *state = 1 - *state;
                        dsts.iter().for_each(|&dst| q.push_back((to, dst, *state)));
                    }
                }
                Conjunction(srcs, dsts, state) => {
                    if pulse == 1 {
                        *state |= 1 << from;
                    } else {
                        *state &= !(1 << from);
                    }
                    dsts.iter().for_each(|&dst| {
                        q.push_back((to, dst, (state.count_ones() != srcs.len() as u32) as u32))
                    });
                }
                Broadcaster(dsts) => {
                    dsts.iter().for_each(|&dst| q.push_back((to, dst, pulse)));
                }
                Other(..) => (),
            }
        }
    }
    (pulses[0], pulses[1], times)
}

fn main() {
    let input = read_to_string("input/20.txt").unwrap();
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
        assert_eq!(
            part1(&read_to_string("examples/20-1.txt").unwrap()),
            32000000
        );
        assert_eq!(
            part1(&read_to_string("examples/20-2.txt").unwrap()),
            11687500
        );
    }
}
