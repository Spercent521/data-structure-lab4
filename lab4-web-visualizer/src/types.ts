export interface VisualizationStep {
  visited_nodes: number[];
  current_node: number | null;
  edges_in_path: [number, number][];
  candidate_edges: [number, number][];
  explanation: string;
}

export interface VisualizationData {
  steps: VisualizationStep[];
}
