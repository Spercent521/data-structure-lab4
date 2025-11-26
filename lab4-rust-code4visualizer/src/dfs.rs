use std::collections::HashMap;

use crate::graph;

pub fn dfs(graph: &graph::Graph, 
            idx_to_city: &Vec<String>, 
            root_cityname_for_dfs: &String, 
            _city_to_idx: &HashMap<String, usize>){

    // just to avoid unused variable warning
    let _ = (graph, idx_to_city , _city_to_idx ,root_cityname_for_dfs.clone());
}