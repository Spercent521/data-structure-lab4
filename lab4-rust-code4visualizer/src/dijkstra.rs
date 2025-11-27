use crate::graph::{self, Visualization, VisualizationStep};
use colored::*;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// perform Dijkstra's algorithm on the graph starting from the given root city name
pub fn dijkstra(
    graph: &graph::Graph,
    idx_to_city: &Vec<String>,
    root_cityname_for_dijkstra: &String,
    city_to_idx: &HashMap<String, usize>,
) -> Visualization {
    let start_node_idx = match city_to_idx.get(root_cityname_for_dijkstra) {
        Some(&idx) => idx,
        None => {
            println!(
                "{} Error: Root city '{}' not found in graph.",
                "[Error]".red(),
                root_cityname_for_dijkstra
            );
            return Visualization { steps: vec![] };
        }
    };

    let mut visualization = Visualization { steps: vec![] };
    let mut dist: Vec<_> = (0..graph.node_count).map(|_| u32::MAX).collect();
    let mut pq = BinaryHeap::new();
    let mut prev = vec![None; graph.node_count];

    dist[start_node_idx] = 0;
    pq.push(State {
        cost: 0,
        position: start_node_idx,
    });

    visualization.steps.push(VisualizationStep {
        visited_nodes: vec![],
        current_node: Some(start_node_idx),
        edges_in_path: vec![],
        candidate_edges: graph.adj[start_node_idx].iter().map(|e| (start_node_idx, e.to)).collect(),
        explanation: format!(
            "Starting Dijkstra's algorithm from {}",
            idx_to_city[start_node_idx]
        ),
    });

    let mut visited_nodes = vec![];

    // loop , greedy
    while let Some(State { cost, position }) = pq.pop() {
        if cost > dist[position] {
            continue;
        }

        if !visited_nodes.contains(&position) {
            visited_nodes.push(position);
        }
        
        let mut path_edges = Vec::new();
        for i in 0..graph.node_count {
            if let Some(p) = prev[i] {
                path_edges.push((p, i));
            }
        }

        visualization.steps.push(VisualizationStep {
            visited_nodes: visited_nodes.clone(),
            current_node: Some(position),
            edges_in_path: path_edges.clone(),
            candidate_edges: graph.adj[position].iter().map(|e| (position, e.to)).collect(),
            explanation: format!("Visiting node {}", idx_to_city[position]),
        });

        for edge in &graph.adj[position] {
            let next = State {
                cost: cost + edge.weight,
                position: edge.to,
            };

            if next.cost < dist[next.position] {
                pq.push(next);
                dist[next.position] = next.cost;
                prev[next.position] = Some(position);
            }
        }
    }
    
    let mut final_path_edges = Vec::new();
    for i in 0..graph.node_count {
        if let Some(p) = prev[i] {
            final_path_edges.push((p, i));
        }
    }

    visualization.steps.push(VisualizationStep {
        visited_nodes: visited_nodes,
        current_node: None,
        edges_in_path: final_path_edges,
        candidate_edges: vec![],
        explanation: "Dijkstra's algorithm complete".to_string(),
    });

    visualization
}