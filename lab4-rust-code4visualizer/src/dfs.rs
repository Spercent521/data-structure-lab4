use crate::graph::{self, Visualization, VisualizationStep};
use colored::*;
use std::collections::HashMap;

/// perform DFS traversal on the graph starting from the given root city name
pub fn dfs(
    graph: &graph::Graph, 
    idx_to_city: &Vec<String>, 
    root_cityname_for_dfs: &String, 
    city_to_idx: &HashMap<String, usize>
) -> Visualization {
    let start_node_idx = match city_to_idx.get(root_cityname_for_dfs) {
        Some(&idx) => idx,
        None => {
            println!(
                "{} Error: Root city '{}' not found in graph.",
                "[Error]".red(),
                root_cityname_for_dfs
            );
            return Visualization { steps: vec![] };
        }
    };

    let mut visualization = Visualization { steps: vec![] };
    let mut visited = vec![false; graph.node_count];
    let mut stack = vec![start_node_idx];
    let mut traversal_edges = Vec::new();
    let mut parent_map = HashMap::new();

    let initial_step = VisualizationStep {
        visited_nodes: vec![],
        current_node: Some(start_node_idx),
        edges_in_path: vec![],
        candidate_edges: graph.adj[start_node_idx]
            .iter()
            .map(|edge| (start_node_idx, edge.to))
            .collect(),
        explanation: format!("Starting DFS from {}", idx_to_city[start_node_idx]),
    };
    visualization.steps.push(initial_step);

    while let Some(u) = stack.last().cloned() {
        if !visited[u] {
            visited[u] = true;

            if let Some(parent) = parent_map.get(&u) {
                traversal_edges.push((*parent, u));
            }

            let mut visited_nodes_indices = Vec::new();
            for (i, &is_visited) in visited.iter().enumerate() {
                if is_visited {
                    visited_nodes_indices.push(i);
                }
            }

            let step = VisualizationStep {
                visited_nodes: visited_nodes_indices.clone(),
                current_node: Some(u),
                edges_in_path: traversal_edges.clone(),
                candidate_edges: graph.adj[u]
                    .iter()
                    .filter(|edge| !visited[edge.to])
                    .map(|edge| (u, edge.to))
                    .collect(),
                explanation: format!("Visiting node {}", idx_to_city[u]),
            };
            visualization.steps.push(step);
        }

        let mut found_unvisited = false;
        for edge in graph.adj[u].iter() {
            let v = edge.to;
            if !visited[v] {
                parent_map.insert(v, u);
                stack.push(v);
                found_unvisited = true;
                break; 
            }
        }

        if !found_unvisited {
            stack.pop();
            if let Some(current_u) = stack.last() {
                let step = VisualizationStep {
                    visited_nodes: visualization.steps.last().unwrap().visited_nodes.clone(),
                    current_node: Some(*current_u),
                    edges_in_path: traversal_edges.clone(),
                    candidate_edges: graph.adj[*current_u]
                        .iter()
                        .filter(|edge| !visited[edge.to])
                        .map(|edge| (*current_u, edge.to))
                        .collect(),
                    explanation: format!("Backtracking to {}", idx_to_city[*current_u]),
                };
                visualization.steps.push(step);
            }
        }
    }

    let final_step = VisualizationStep {
        visited_nodes: (0..graph.node_count).filter(|&i| visited[i]).collect(),
        current_node: None,
        edges_in_path: traversal_edges,
        candidate_edges: vec![],
        explanation: "DFS traversal complete".to_string(),
    };
    visualization.steps.push(final_step);

    visualization
}