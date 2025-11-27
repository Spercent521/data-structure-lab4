import { useState, useEffect } from 'react';
import { Allotment } from 'allotment';
import 'allotment/dist/style.css';
import './App.css';
import LeftSidebar from './components/LeftSidebar';
import Visualization from './components/Visualization';
import CodeDisplay from './components/CodeDisplay';
import type { VisualizationData, VisualizationStep } from './types';

import dfsSteps from './dfs_steps.json';
import primSteps from './prim_steps.json';
import dijkstraSteps from './dijkstra_steps.json';

const algorithmData: { [key: string]: VisualizationData } = {
  DFS: dfsSteps as VisualizationData,
  PRIM: primSteps as VisualizationData,
  DIJKSTRA: dijkstraSteps as VisualizationData,
};

const dfsCode = `pub fn dfs(graph: &Graph, start_node: usize) -> Visualization {
    let mut visualization = Visualization::new();
    let mut visited = vec![false; graph.node_count];
    let mut stack = vec![start_node];
    let mut path_edges = Vec::new();

    while let Some(u) = stack.last().cloned() {
        if !visited[u] {
            visited[u] = true;
            
            visualization.add_step(
                visited.iter().enumerate().filter(|&(_, &v)| v).map(|(i, _)| i).collect(),
                Some(u),
                path_edges.clone(),
                graph.adj[u].iter().map(|e| (u, e.to)).collect(),
                format!("Visiting node {}", u),
            );

            let mut pushed = false;
            for edge in &graph.adj[u] {
                let v = edge.to;
                if !visited[v] {
                    stack.push(v);
                    path_edges.push((u, v));
                    pushed = true;
                    break; 
                }
            }
            if !pushed {
                stack.pop();
                path_edges.pop();
            }
        } else {
            stack.pop();
            path_edges.pop();
        }
    }
    visualization
}`;

const primCode = `pub fn prim(graph: &Graph, start_node: usize) -> Visualization {
    let mut visualization = Visualization::new();
    let mut pq = BinaryHeap::new();
    let mut visited = vec![false; graph.node_count];
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    visited[start_node] = true;
    visualization.add_step(
        vec![start_node],
        Some(start_node),
        mst_edges.clone(),
        graph.adj[start_node].iter().map(|e| (start_node, e.to)).collect(),
        format!("Starting Prim's from node {}", start_node),
    );

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
        mst_edges.push((from, to));
        total_weight += weight;

        let candidate_edges: Vec<(usize, usize)> = pq.iter().map(|s| (s.from, s.to)).collect();
        visualization.add_step(
            visited.iter().enumerate().filter(|&(_, &v)| v).map(|(i, _)| i).collect(),
            Some(to),
            mst_edges.clone(),
            candidate_edges,
            format!("Adding edge ({}, {}) with weight {}", from, to, weight),
        );

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
    visualization
}`;

const dijkstraCode = `pub fn dijkstra(graph: &Graph, start_node: usize) -> Visualization {
    let mut visualization = Visualization::new();
    let mut dist = vec![u32::MAX; graph.node_count];
    let mut prev = vec![None; graph.node_count];
    let mut visited = vec![false; graph.node_count];
    dist[start_node] = 0;

    let mut pq = std::collections::BinaryHeap::new();
    pq.push((0u32, start_node));

    visualization.add_step(
        vec![],
        Some(start_node),
        vec![],
        vec![],
        format!("Starting Dijkstra's from node {}", start_node),
    );

    while let Some((current_dist, u)) = pq.pop() {
        let current_dist = -(current_dist as i32) as u32;

        if visited[u] {
            continue;
        }
        visited[u] = true;

        let mut path_to_u = Vec::new();
        let mut curr = u;
        while let Some(p) = prev[curr] {
            path_to_u.push((p, curr));
            curr = p;
        }
        path_to_u.reverse();

        visualization.add_step(
            visited.iter().enumerate().filter(|&(_, &v)| v).map(|(i, _)| i).collect(),
            Some(u),
            path_to_u,
            graph.adj[u].iter().filter(|e| !visited[e.to]).map(|e| (u, e.to)).collect(),
            format!("Visiting node {}, current distance: {}", u, current_dist),
        );

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
    visualization
}`;

const algorithmCode: { [key: string]: string } = {
  DFS: dfsCode,
  PRIM: primCode,
  DIJKSTRA: dijkstraCode,
};

function App() {
  const [selectedAlgorithm, setSelectedAlgorithm] = useState('DFS');
  const [visualization, setVisualization] = useState<VisualizationData>(algorithmData[selectedAlgorithm]);
  const [stepIndex, setStepIndex] = useState(0);
  const [currentStep, setCurrentStep] = useState<VisualizationStep | null>(visualization.steps[0] || null);
  const [isPlaying, setIsPlaying] = useState(false);

  useEffect(() => {
    const data = algorithmData[selectedAlgorithm];
    setVisualization(data);
    setStepIndex(0);
    setCurrentStep(data.steps[0] || null);
    setIsPlaying(false);
  }, [selectedAlgorithm]);

  useEffect(() => {
    let interval: number | undefined;
    if (isPlaying && stepIndex < visualization.steps.length - 1) {
      interval = setInterval(() => {
        setStepIndex(prevIndex => {
          const nextIndex = prevIndex + 1;
          if (nextIndex < visualization.steps.length) {
            setCurrentStep(visualization.steps[nextIndex]);
            return nextIndex;
          } else {
            setIsPlaying(false);
            return prevIndex;
          }
        });
      }, 1000);
    } else if (isPlaying && stepIndex >= visualization.steps.length - 1) {
      setIsPlaying(false);
    }
    return () => clearInterval(interval);
  }, [isPlaying, stepIndex, visualization]);

  const handleSelectAlgorithm = (algo: string) => {
    setSelectedAlgorithm(algo);
  };

  const handlePlay = () => {
    if (stepIndex >= visualization.steps.length - 1) {
      setStepIndex(0);
      setCurrentStep(visualization.steps[0]);
    }
    setIsPlaying(!isPlaying);
  };

  const handleNext = () => {
    if (stepIndex < visualization.steps.length - 1) {
      const nextIndex = stepIndex + 1;
      setStepIndex(nextIndex);
      setCurrentStep(visualization.steps[nextIndex]);
      setIsPlaying(false);
    }
  };

  const handlePrev = () => {
    if (stepIndex > 0) {
      const prevIndex = stepIndex - 1;
      setStepIndex(prevIndex);
      setCurrentStep(visualization.steps[prevIndex]);
      setIsPlaying(false);
    }
  };

  const handleReset = () => {
    setStepIndex(0);
    setCurrentStep(visualization.steps[0]);
    setIsPlaying(false);
  };

  return (
    <div className="App">
      <Allotment>
        <Allotment.Pane minSize={200} maxSize={400}>
          <LeftSidebar
            algorithms={['DFS', 'PRIM', 'DIJKSTRA']}
            selectedAlgorithm={selectedAlgorithm}
            onSelectAlgorithm={handleSelectAlgorithm}
            onPlay={handlePlay}
            onNext={handleNext}
            onPrev={handlePrev}
            onReset={handleReset}
            isPlaying={isPlaying}
          />
        </Allotment.Pane>
        <Allotment.Pane>
          <Allotment vertical>
            <Allotment.Pane>
              <Visualization currentStep={currentStep} />
            </Allotment.Pane>
            <Allotment.Pane>
              <CodeDisplay
                code={algorithmCode[selectedAlgorithm]}
                explanation={currentStep?.explanation}
              />
            </Allotment.Pane>
          </Allotment>
        </Allotment.Pane>
      </Allotment>
    </div>
  );
}

export default App;



