use crate::graph;
use colored::*;
use std::collections::HashMap;

/// perform Dijkstra's algorithm on the graph starting from the given root city name
pub fn dijkstra(
    graph: &graph::Graph, 
    idx_to_city: &Vec<String>, 
    root_cityname_for_dijkstra: &String, 
    city_to_idx: &HashMap<String, usize>
){
    // init
    let start_node_idx = match city_to_idx.get(root_cityname_for_dijkstra) {
        Some(&idx) => idx,
        None => {
            println!("{} Error: Root city '{}' not found in graph.", "[Error]".red(), root_cityname_for_dijkstra);
            return;
        }
    };

    let mut dist = vec![u32::MAX; graph.node_count];
    let mut prev = vec![None; graph.node_count];
    let mut visited = vec![false; graph.node_count];
    dist[start_node_idx] = 0;

    let mut pq = std::collections::BinaryHeap::new();
    pq.push((0u32, start_node_idx)); // (distance, node_index)

    // loop , greedy
    while let Some((current_dist, u)) = pq.pop() {
        let current_dist = -(current_dist as i32) as u32; // Convert back to positive

        if visited[u] {
            continue;
        }
        visited[u] = true;

        for edge in &graph.adj[u] {
            let v = edge.to;
            let weight = edge.weight;
            if !visited[v] {
                let new_dist = current_dist.saturating_add(weight);
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    prev[v] = Some(u);
                    pq.push((-(new_dist as i32) as u32, v)); // Store negative distance for max-heap
                }
            }
        }
    }

    // Print shortest paths(results)
    println!("{} Dijkstra's shortest paths from {}:", 
        "[dijkstra]".blue(), 
        root_cityname_for_dijkstra.cyan()
    );
    for (i, &d) in dist.iter().enumerate() {
        if d == u32::MAX {
            println!("  To {}: {}", idx_to_city[i].green(), "unreachable".red());
        } else {
            // Reconstruct path
            let mut path_nodes = Vec::new();
            let mut current = i;
            while let Some(p) = prev[current] {
                path_nodes.push(idx_to_city[current].clone());
                current = p;
            }
            path_nodes.push(idx_to_city[start_node_idx].clone());
            path_nodes.reverse();

            // Colorize the path string
            let path_str = path_nodes.iter()
                .map(|node| node.green().to_string())
                .collect::<Vec<String>>()
                .join(&" -> ".white().to_string());

            println!("  To {}: distance = {}, path = {}", 
                idx_to_city[i].cyan(), 
                d.to_string().yellow(), 
                path_str
            );
        }
    }
}