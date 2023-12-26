use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

type Adj = Vec<(usize, usize)>;
type Edge = (usize, usize);

fn part1(input: &str) -> usize {
    let (mut adj, edges) = parse_input(input);
    let remove = find_critical_edges(&adj, &edges);
    remove_edges(&mut adj, &remove, &edges);
    count_connected_components(&adj).into_iter().product()
}

fn parse_input(input: &str) -> (Vec<Adj>, Vec<Edge>) {
    let mut edges = Vec::new();
    let mut edge_map = HashMap::new();
    let mut adj = Vec::new();
    let mut node_map = HashMap::new();
    input.lines().for_each(|line| {
        let (src, dst) = line.split_once(": ").unwrap();
        let src = get_or_add_node(src, &mut node_map, &mut adj);
        dst.split_whitespace().for_each(|dst| {
            let dst = get_or_add_node(dst, &mut node_map, &mut adj);
            let edge = get_or_add_edge((src, dst), &mut edge_map, &mut edges);
            adj[src].push((dst, edge));
            adj[dst].push((src, edge));
        });
    });

    (adj, edges)
}

fn get_or_add_node<'a>(s: &'a str, map: &mut HashMap<&'a str, usize>, adj: &mut Vec<Adj>) -> usize {
    let n = map.len();
    *map.entry(s).or_insert_with(|| {
        adj.push(Vec::new());
        n
    })
}

fn get_or_add_edge(edge: Edge, map: &mut HashMap<Edge, usize>, edges: &mut Vec<Edge>) -> usize {
    let edge = if edge.0 < edge.1 {
        edge
    } else {
        (edge.1, edge.0)
    };
    let n = map.len();
    *map.entry(edge).or_insert_with(|| {
        edges.push(edge);
        n
    })
}

fn find_critical_edges(adj: &[Adj], edges: &[Edge]) -> Vec<usize> {
    edges
        .iter()
        .enumerate()
        .filter(|(_, (n1, n2))| is_critical_edge(adj, *n1, *n2, edges.len()))
        .map(|(edge, ..)| edge)
        .collect()
}

fn is_critical_edge(adj: &[Adj], cur: usize, target: usize, edges: usize) -> bool {
    let mut seen_edges = vec![false; edges];
    (0..4.min(adj[cur].len()))
        .filter(|_| is_reachable(adj, cur, target, &mut seen_edges))
        .count()
        == 3
}

fn is_reachable(adj: &[Adj], cur: usize, target: usize, seen_edges: &mut [bool]) -> bool {
    let mut q = VecDeque::from([(cur, None)]);
    let mut path = vec![None; seen_edges.len()];
    let mut seen_nodes = vec![false; adj.len()];
    seen_nodes[cur] = true;

    while let Some((cur, edge)) = q.pop_front() {
        if cur == target {
            let mut cur_edge = edge;
            while let Some(next_edge) = cur_edge {
                seen_edges[next_edge] = true;
                cur_edge = path[next_edge];
            }
            return true;
        }
        for &(next, next_edge) in adj[cur].iter() {
            if seen_nodes[next] || seen_edges[next_edge] {
                continue;
            }
            seen_nodes[next] = true;
            path[next_edge] = edge;
            q.push_back((next, Some(next_edge)));
        }
    }
    false
}

fn remove_edges(adj: &mut [Adj], remove: &[usize], edges: &[Edge]) {
    remove.iter().map(|r| edges[*r]).for_each(|(src, dst)| {
        adj[src].swap_remove(adj[src].iter().position(|(n, _)| *n == dst).unwrap());
        adj[dst].swap_remove(adj[dst].iter().position(|(n, _)| *n == src).unwrap());
    });
}

fn count_connected_components(adj: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut seen = vec![false; adj.len()];
    (0..adj.len())
        .map(|node| count_reachable(adj, node, &mut seen))
        .filter(|&count| count > 0)
        .collect()
}

fn count_reachable(adj: &[Adj], cur: usize, seen: &mut [bool]) -> usize {
    if seen[cur] {
        return 0;
    }
    seen[cur] = true;

    adj[cur]
        .iter()
        .map(|(next, _)| count_reachable(adj, *next, seen))
        .fold(1, |acc, ret| acc + ret)
}

fn main() {
    let input = read_to_string("input/25.txt").unwrap();
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_to_string("examples/25.txt").unwrap()), 54);
    }
}
