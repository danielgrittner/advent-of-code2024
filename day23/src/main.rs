use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Computer = [char; 2];

fn read_data(path: &str) -> std::io::Result<Vec<(Computer, Computer)>> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|row| {
            let edge = row
                .split("-")
                .map(|s| {
                    let cs = s.chars().collect::<Vec<char>>();
                    [cs[0], cs[1]]
                })
                .collect::<Vec<[char; 2]>>();
            (edge[0], edge[1])
        })
        .collect())
}

fn find_max_clique(
    init_clique: &mut HashSet<Computer>,
    idx: usize,
    candidates: &Vec<Computer>,
    adj_list: &HashMap<Computer, HashSet<Computer>>,
) -> HashSet<Computer> {
    if idx == candidates.len() {
        return init_clique.clone();
    }

    let opt1 = find_max_clique(init_clique, idx + 1, candidates, adj_list);

    let candidate = candidates[idx];
    let intersection_length = init_clique
        .intersection(adj_list.get(&candidate).unwrap())
        .count();

    if intersection_length != init_clique.len() {
        return opt1;
    }

    init_clique.insert(candidate);
    let opt2 = find_max_clique(init_clique, idx + 1, candidates, adj_list);
    init_clique.remove(&candidate);

    if opt1.len() < opt2.len() {
        opt2
    } else {
        opt1
    }
}

fn main() -> std::io::Result<()> {
    // let edges = read_data("example.txt")?;
    let edges = read_data("input.txt")?;

    let mut node_to_id = HashMap::new();
    for &edge in &edges {
        for node in [edge.0, edge.1] {
            let next_id = node_to_id.len();
            node_to_id.entry(node).or_insert(next_id);
        }
    }

    let mut adj_list: HashMap<Computer, HashSet<Computer>> = HashMap::new();
    edges.iter().for_each(|(e1, e2)| {
        adj_list
            .entry(*e1)
            .and_modify(|neighbors| {
                neighbors.insert(*e2);
            })
            .or_insert_with(|| {
                let mut ns = HashSet::new();
                ns.insert(*e2);
                ns
            });

        adj_list
            .entry(*e2)
            .and_modify(|neighbors| {
                neighbors.insert(*e1);
            })
            .or_insert_with(|| {
                let mut ns = HashSet::new();
                ns.insert(*e1);
                ns
            });
    });

    let start_time = std::time::Instant::now();
    let mut cliques_three = HashSet::new();
    for (c1, c1_neighbors) in adj_list.iter() {
        for c2 in c1_neighbors {
            for c3 in c1_neighbors {
                if c2 == c3 {
                    continue;
                }

                if adj_list.get(c2).unwrap().contains(c3)
                    && (c1[0] == 't' || c2[0] == 't' || c3[0] == 't')
                {
                    let mut group = [*c1, *c2, *c3];
                    group.sort_unstable();
                    cliques_three.insert(group);
                }
            }
        }
    }
    let task1 = cliques_three
        .iter()
        .filter(|&group| group[0][0] == 't' || group[1][0] == 't' || group[2][0] == 't')
        .count();
    let duration = start_time.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let start_time = std::time::Instant::now();
    let max_clique = adj_list
        .par_iter()
        .map(|(center, candidates)| {
            let mut init_clique: HashSet<_> = HashSet::new();
            init_clique.insert(center.clone());
            let candidates = candidates.iter().map(|&c| c).collect::<Vec<Computer>>();
            let result = find_max_clique(&mut init_clique, 0, &candidates, &adj_list);
            result
        })
        .reduce(
            || HashSet::new(),
            |max_group, group| {
                if max_group.len() < group.len() {
                    group
                } else {
                    max_group
                }
            },
        );
    let mut result = max_clique.into_iter().collect::<Vec<_>>();
    result.sort_unstable();
    let task2 = result
        .into_iter()
        .map(|cs| String::from_iter(cs.into_iter()))
        .collect::<Vec<String>>()
        .join(",");
    let duration = start_time.elapsed();
    println!("Task 2: {task2} (took {:?})", duration);

    Ok(())
}
