use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Represents an edge in the graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Edge {
    pub to: usize,
    pub weight: u32,
}

// Represents the graph using an adjacency list
// node 0 : (Beijing): [Edge{to:1, weight:750}, Edge{to:2, weight:800}, Edge{to:3, weight:1140}, Edge{to:4, weight:650}]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub adj: Vec<Vec<Edge>>,
    pub node_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualizationStep {
    pub visited_nodes: Vec<usize>,
    pub current_node: Option<usize>,
    pub edges_in_path: Vec<(usize, usize)>, // (from, to)
    pub candidate_edges: Vec<(usize, usize)>, // Edges being considered
    pub explanation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Visualization {
    pub steps: Vec<VisualizationStep>,
}

impl Graph {
    // Creates a new graph with a given number of nodes
    pub fn new(node_count: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); node_count],
            node_count,
        }
    }

    // Adds an undirected edge between two nodes
    pub fn add_edge(&mut self, u: usize, v: usize, weight: u32) {
        self.adj[u].push(Edge { to: v, weight });
        self.adj[v].push(Edge { to: u, weight });
    }
}

/// Creates the graph from hardcoded data based on tp.md
/// Returns the graph and a map from city name to its index in the adjacency list.
pub fn create_city_graph() -> (Graph, HashMap<String, usize>, Vec<String>) {
    let cities = vec![
        "Beijing", "Shenyang", "Qingdao", "Xian", "Zhengzhou",
        "Wuhan", "Chengdu", "Chongqing", "Changsanjiao", "Zhusanjiao",
    ];

    let node_count = cities.len();
    let mut graph = Graph::new(node_count);
    let mut city_to_idx: HashMap<String, usize> = HashMap::new();
    let mut idx_to_city: Vec<String> = vec![String::new(); node_count];

    for (i, city) in cities.iter().enumerate() {
        city_to_idx.insert(city.to_string(), i);
        idx_to_city[i] = city.to_string();
    }

    let edges = vec![
        ("Beijing", "Shenyang", 750),
        ("Shenyang", "Qingdao", 680),
        ("Beijing", "Qingdao", 800),
        ("Beijing", "Xian", 1140),
        ("Beijing", "Zhengzhou", 650),
        ("Xian", "Zhengzhou", 570),
        ("Zhengzhou", "Qingdao", 820),
        ("Zhengzhou", "Wuhan", 530),
        ("Xian", "Chengdu", 840),
        ("Chengdu", "Chongqing", 340),
        ("Chongqing", "Wuhan", 900),
        ("Zhengzhou", "Changsanjiao", 1200),
        ("Qingdao", "Changsanjiao", 960),
        ("Wuhan", "Changsanjiao", 680),
        ("Zhusanjiao", "Chongqing", 2500),
        ("Zhusanjiao", "Wuhan", 1380),
        ("Zhusanjiao", "Changsanjiao", 2600),
    ];

    for (city1, city2, weight) in edges {
        let u = *city_to_idx.get(city1).unwrap();
        let v = *city_to_idx.get(city2).unwrap();
        graph.add_edge(u, v, weight);
    }

    (graph, city_to_idx, idx_to_city)
}
