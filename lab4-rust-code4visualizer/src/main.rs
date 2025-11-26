use lab4_rust_code4visualizer::graph;

fn main() {
    // 现在你可以调用 create_city_graph 函数了
    let (graph, city_to_idx, idx_to_city) = graph::create_city_graph();

    // 打印一些信息来验证
    println!("Graph created with {} nodes.", graph.node_count);
    println!("Index of 'Beijing': {}", city_to_idx["Beijing"]);
    println!("City at index 0: {}", idx_to_city[0]);

    // 你还可以遍历邻接表示
    for (i, city_name) in idx_to_city.iter().enumerate() {
        print!("Neighbors of {}: ", city_name);
        for edge in &graph.adj[i] {
            print!("({}, {}) ", idx_to_city[edge.to], edge.weight);
        }
        println!();
    }
}
