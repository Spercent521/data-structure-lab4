use crate::graph::Graph;
use colored::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// BinaryHeap is a max-heap by default in Rust

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    weight: u32,
    from: usize,
    to: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// perform prim on the graph starting from the given root city name
pub fn prim(
    graph: &Graph,
    idx_to_city: &Vec<String>,
    root_cityname_for_prim: &String,
    city_to_idx: &HashMap<String, usize>,
) {
    // init
    let start_node = match city_to_idx.get(root_cityname_for_prim) {
        Some(&idx) => idx,
        None => {
            println!("Root city '{}' not found.", root_cityname_for_prim);
            return;
        }
    };

    let mut pq = BinaryHeap::new();
    let mut visited = vec![false; graph.node_count];
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    visited[start_node] = true;
    for edge in &graph.adj[start_node] {
        pq.push(State {
            weight: edge.weight,
            from: start_node,
            to: edge.to,
        });
    }

    // loop , greedy
    while let Some(State { weight, from, to }) = pq.pop() {
        if visited[to] {
            continue;
        }

        visited[to] = true;
        mst_edges.push((from, to, weight));
        total_weight += weight;

        if mst_edges.len() == graph.node_count - 1 {
            break;
        }

        for edge in &graph.adj[to] {
            if !visited[edge.to] {
                pq.push(State {
                    weight: edge.weight,
                    from: to,
                    to: edge.to,
                });
            }
        }
    }

    // print the result
    println!("{} generate MST by Prim's Algorithm from '{}':",
        "[prim]".blue(),
        root_cityname_for_prim.cyan()
    );
    for (u, v, w) in mst_edges {
        println!(
            "Edge: {} - {}, Weight: {}",
            idx_to_city[u].green(),
            idx_to_city[v].green(),
            w.to_string().yellow()
        );
    }
    println!("{} Total weight of MST: {}", 
        "[prim]".blue(),
        total_weight.to_string().cyan()
    );
}
