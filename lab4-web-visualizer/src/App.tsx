import { useState } from 'react';
import { Allotment } from 'allotment';
import 'allotment/dist/style.css';
import './App.css';
import LeftSidebar from './components/LeftSidebar';
import Visualization from './components/Visualization';
import CodeDisplay from './components/CodeDisplay';

const dfsCode = `pub fn dfs(
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
    let mut traversal_path = Vec::new();
    let mut stack = vec![start_node_idx];

    while let Some(u) = stack.pop() {
        if !visited[u] {
            visited[u] = true;
            traversal_path.push(u);

            for edge in graph.adj[u].iter().rev() {
                let v = edge.to;
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
    }
}`;

const primCode = `pub fn prim(
    graph: &Graph,
    idx_to_city: &Vec<String>,
    root_cityname_for_prim: &String,
    city_to_idx: &HashMap<String, usize>,
) {
    let start_node = match city_to_idx.get(root_cityname_for_prim) {
        Some(&idx) => idx,
        None => {
            println!("Root city '{}' not found.", root_cityname_for_prim);
            return;
        }
    };

    let mut pq = BinaryHeap::new();
    let mut visited = vec![false; graph.node_count];
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    visited[start_node] = true;
    for edge in &graph.adj[start_node] {
        pq.push(State {
            weight: edge.weight,
            from: start_node,
            to: edge.to,
        });
    }

    while let Some(State { weight, from, to }) = pq.pop() {
        if visited[to] {
            continue;
        }

        visited[to] = true;
        mst_edges.push((from, to, weight));
        total_weight += weight;

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
            }
        }
    }
}`;

const dijkstraCode = `pub fn dijkstra(
    graph: &graph::Graph, 
    idx_to_city: &Vec<String>, 
    root_cityname_for_dijkstra: &String, 
    city_to_idx: &HashMap<String, usize>
){
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
    pq.push((0u32, start_node_idx));

    while let Some((current_dist, u)) = pq.pop() {
        let current_dist = -(current_dist as i32) as u32;

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
                    pq.push((-(new_dist as i32) as u32, v));
                }
            }
        }
    }
}`;

const algorithmCodes: { [key: string]: string } = {
  DFS: dfsCode,
  PRIM: primCode,
  DIJKSTRA: dijkstraCode,
};

function App() {
  const [code, setCode] = useState(algorithmCodes['DFS']);

  const handleAlgorithmChange = (algo: string) => {
    setCode(algorithmCodes[algo]);
  };

  return (
    <div className="app">
      <Allotment>
        <Allotment.Pane>
          <LeftSidebar setSelectedAlgorithm={handleAlgorithmChange} />
        </Allotment.Pane>
        <Allotment.Pane>
          <Visualization />
        </Allotment.Pane>
        <Allotment.Pane>
          <CodeDisplay code={code} />
        </Allotment.Pane>
      </Allotment>
    </div>
  );
}

export default App;



