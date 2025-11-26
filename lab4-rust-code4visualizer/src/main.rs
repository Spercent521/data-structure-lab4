use colored::*;
use lab4_rust_code4visualizer::graph;
use lab4_rust_code4visualizer::dfs;
use lab4_rust_code4visualizer::prim;
use rand::Rng;
use std::io;
// !cargo add petgraph
// [Optional] Efficient Implementation of Built-in Graph Theory Algorithms in Rust
// [Optional] https://docs.rs/petgraph/latest/petgraph/
// [Optional] chech details with ai tools
// use petgraph::Graph;
// use petgraph::visit::Dfs;

/// choose a root (default == random , and a input)
/// returns the chosen city name
fn choose_root_city(idx_to_city: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen_range(0..10);
    let default_input = idx_to_city[random_number].clone();     // clone is a must
    
    println!("{} choose a root to start DFS traversal (using default root {} by ENTER):",
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
fn print_graph_info(graph: &graph::Graph, idx_to_city: &Vec<String>) {
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
    // 1. tomography of the graph
    //      data structure: adjacency list . we use numbers to represent cities
    //      numbers 0 to 9 represent the cities , as in graph.rs's immutable variable 'cities'
    let (graph, _city_to_idx, idx_to_city) = graph::create_city_graph();

    print_graph_info(&graph, &idx_to_city);

    // 2. dfs for traversal
     
    //    choose a root city interactively
    let root_cityname_for_dfs = choose_root_city(&idx_to_city);

    //    dfs traversal from the chosen root city
    dfs::dfs(&graph, &idx_to_city, &root_cityname_for_dfs, &_city_to_idx);    
    
    // 3. prim for mst
    let root_cityname_for_prim = choose_root_city(&idx_to_city);
    prim::prim(&graph, &idx_to_city, &root_cityname_for_prim, &_city_to_idx);

    // 4. dijkstra for shortest path

    // just to avoid unused variable warning
    let _ = root_cityname_for_dfs.clone();
}
