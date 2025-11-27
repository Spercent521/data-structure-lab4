use lab4_rust_code4visualizer::graph::{Graph};
use lab4_rust_code4visualizer::dfs;
use lab4_rust_code4visualizer::prim;
use lab4_rust_code4visualizer::dijkstra;
use std::collections::HashMap;
use std::fs;
use serde_json;
use std::io;
use rand::Rng;
use colored::*;

/// choose a root (default == random , and a input)
/// returns the chosen city name
fn choose_root_city(idx_to_city: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen_range(0..idx_to_city.len());
    let default_input = idx_to_city[random_number].clone();     // clone is a must
    
    println!("{} choose a root to start traversal (using default root {} by ENTER):",
        "[Interaction]".bright_cyan(),
        default_input.cyan()
    );

    let mut user_input = String::new();

    let waring_info = format!("{} select a root city failed", "[Error]".red());

    io::stdin().read_line(&mut user_input).expect(&waring_info);

    let input = if user_input.trim().is_empty() {
        default_input
    } else {
        user_input.trim().to_string()
    };

    println!("{} selected root city: {}",
        "[Info]".blue(),
        input.cyan()
    );

    input
}

/// print info of the graph (when init)
fn print_graph_info(graph: &Graph, idx_to_city: &Vec<String>) {
    // [Info] about the graph
    println!("{}", format!("[Info] Graph created with {} nodes.", graph.node_count).blue());

    // [Info] print the adjacency list
    for (i, city_name) in idx_to_city.iter().enumerate() {
        let mut neighbors_str = String::new();
        for (j, edge) in graph.adj[i].iter().enumerate() {
            neighbors_str.push_str(&format!("({}, {})", idx_to_city[edge.to], edge.weight));
            if j < graph.adj[i].len() - 1 {
                neighbors_str.push_str(", ");
            }
        }
        println!("{} Neighbors of {}: {}", 
            "[Info]".blue(), 
            city_name, 
            neighbors_str
        );
    }
}

fn main() {
    let (graph, idx_to_city, city_to_idx) = build_graph();
    print_graph_info(&graph, &idx_to_city);

    let root_city = choose_root_city(&idx_to_city);

    // DFS
    let dfs_visualization = dfs::dfs(&graph, &idx_to_city, &root_city, &city_to_idx);
    let dfs_json = serde_json::to_string_pretty(&dfs_visualization).unwrap();
    fs::write(
        "../lab4-web-visualizer/src/dfs_steps.json",
        dfs_json,
    )
    .expect("Unable to write file");
    println!("DFS visualization steps saved to dfs_steps.json");

    // Prim
    let prim_visualization = prim::prim(&graph, &idx_to_city, &root_city, &city_to_idx);
    let prim_json = serde_json::to_string_pretty(&prim_visualization).unwrap();
    fs::write(
        "../lab4-web-visualizer/src/prim_steps.json",
        prim_json,
    )
    .expect("Unable to write file");
    println!("Prim visualization steps saved to prim_steps.json");

    // Dijkstra
    let dijkstra_visualization = dijkstra::dijkstra(&graph, &idx_to_city, &root_city, &city_to_idx);
    let dijkstra_json = serde_json::to_string_pretty(&dijkstra_visualization).unwrap();
    fs::write(
        "../lab4-web-visualizer/src/dijkstra_steps.json",
        dijkstra_json,
    )
    .expect("Unable to write file");
    println!("Dijkstra visualization steps saved to dijkstra_steps.json");
}

fn build_graph() -> (Graph, Vec<String>, HashMap<String, usize>) {
    let cities = vec![
        "Beijing", "Shenyang", "Qingdao", "Xian", "Zhengzhou",
        "Wuhan", "Chengdu", "Chongqing", "Changsanjiao", "Zhusanjiao",
    ];
    let city_to_idx: HashMap<String, usize> = cities
        .iter()
        .enumerate()
        .map(|(i, &city)| (city.to_string(), i))
        .collect();
    let idx_to_city: Vec<String> = cities.iter().map(|&s| s.to_string()).collect();

    let edges = vec![
        ("Shenyang", "Beijing", 750),
        ("Shenyang", "Qingdao", 680),
        ("Beijing", "Qingdao", 800),
        ("Beijing", "Xian", 1140),
        ("Beijing", "Zhengzhou", 650),
        ("Xian", "Zhengzhou", 570),
        ("Qingdao", "Zhengzhou", 820),
        ("Zhengzhou", "Wuhan", 530),
        ("Xian", "Chengdu", 840),
        ("Chengdu", "Chongqing", 340),
        ("Wuhan", "Chongqing", 900),
        ("Zhengzhou", "Changsanjiao", 1200),
        ("Qingdao", "Changsanjiao", 960),
        ("Wuhan", "Changsanjiao", 680),
        ("Chongqing", "Zhusanjiao", 2500),
        ("Wuhan", "Zhusanjiao", 1380),
        ("Zhusanjiao", "Changsanjiao", 2600),
    ];

    let mut graph = Graph::new(cities.len());
    for (u_str, v_str, weight) in edges {
        let u = *city_to_idx.get(u_str).unwrap();
        let v = *city_to_idx.get(v_str).unwrap();
        graph.add_edge(u, v, weight);
    }

    (graph, idx_to_city, city_to_idx)
}
