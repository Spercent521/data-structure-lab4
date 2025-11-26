use crate::graph;
use colored::*;
use std::collections::HashMap;

/// perform DFS traversal on the graph starting from the given root city name
pub fn dfs(
    graph: &graph::Graph, 
    idx_to_city: &Vec<String>, 
    root_cityname_for_dfs: &String, 
    city_to_idx: &HashMap<String, usize>
){
    let start_node_idx = match city_to_idx.get(root_cityname_for_dfs) {
        Some(&idx) => idx,
        None => {
            println!("{} Error: Root city '{}' not found in graph.", "[Error]".red(), root_cityname_for_dfs);
            return;
        }
    };

    let mut visited = vec![false; graph.node_count];
    let mut traversal_path = Vec::new();            // Vec<usize> to store the order of traversal
    let mut stack = vec![start_node_idx];           // Vec<usize> impl stack and maybe better performance than VecDeque(cpp's stack impl)

    while let Some(u) = stack.pop() {
        if !visited[u] {
            visited[u] = true;
            traversal_path.push(u);

            // To mimic the recursive DFS order, push neighbors in reverse order.
            for edge in graph.adj[u].iter().rev() {
                let v = edge.to;
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
    }

    let path_str: Vec<String> = traversal_path.iter().map(|&idx| idx_to_city[idx].clone()).collect();
    
    println!("{} DFS Traversal (Stack-based) from {}: {}", 
        "[dfs]".blue(), 
        root_cityname_for_dfs.cyan(),
        path_str.join(" -> ")
    );
}