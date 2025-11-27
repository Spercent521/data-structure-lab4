use crate::graph::{self, Visualization, VisualizationStep};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

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
    graph: &graph::Graph,
    idx_to_city: &Vec<String>,
    root_cityname_for_prim: &String,
    city_to_idx: &HashMap<String, usize>,
) -> Visualization {
    let start_node = match city_to_idx.get(root_cityname_for_prim) {
        Some(&idx) => idx,
        None => {
            println!("Root city '{}' not found.", root_cityname_for_prim);
            return Visualization { steps: vec![] };
        }
    };

    let mut visualization = Visualization { steps: vec![] };
    let mut pq = BinaryHeap::new();
    let mut visited = vec![false; graph.node_count];
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    visited[start_node] = true;
    let mut candidate_edges = Vec::new();
    for edge in &graph.adj[start_node] {
        pq.push(State {
            weight: edge.weight,
            from: start_node,
            to: edge.to,
        });
        candidate_edges.push((start_node, edge.to));
    }

    visualization.steps.push(VisualizationStep {
        visited_nodes: vec![start_node],
        current_node: Some(start_node),
        edges_in_path: vec![],
        candidate_edges: candidate_edges.clone(),
        explanation: format!("Starting Prim's algorithm from {}", idx_to_city[start_node]),
    });

    // loop , greedy
    while let Some(State { weight, from, to }) = pq.pop() {
        if visited[to] {
            continue;
        }

        visited[to] = true;
        mst_edges.push((from, to, weight));
        total_weight += weight;

        let visited_nodes_indices: Vec<usize> = (0..visited.len()).filter(|&i| visited[i]).collect();

        candidate_edges.retain(|&(f, t)| f != from || t != to);

        visualization.steps.push(VisualizationStep {
            visited_nodes: visited_nodes_indices.clone(),
            current_node: Some(to),
            edges_in_path: mst_edges.iter().map(|&(f, t, _)| (f, t)).collect(),
            candidate_edges: candidate_edges.clone(),
            explanation: format!(
                "Adding edge ({}, {}) with weight {} to MST",
                idx_to_city[from], idx_to_city[to], weight
            ),
        });

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
                candidate_edges.push((to, edge.to));
            }
        }
        
        visualization.steps.push(VisualizationStep {
            visited_nodes: visited_nodes_indices.clone(),
            current_node: Some(to),
            edges_in_path: mst_edges.iter().map(|&(f, t, _)| (f, t)).collect(),
            candidate_edges: candidate_edges.clone(),
            explanation: format!("Updating candidate edges from {}", idx_to_city[to]),
        });
    }

    visualization.steps.push(VisualizationStep {
        visited_nodes: (0..graph.node_count).collect(),
        current_node: None,
        edges_in_path: mst_edges.iter().map(|&(f, t, _)| (f, t)).collect(),
        candidate_edges: vec![],
        explanation: format!("Prim's algorithm complete. Total MST weight: {}", total_weight),
    });

    visualization
}
