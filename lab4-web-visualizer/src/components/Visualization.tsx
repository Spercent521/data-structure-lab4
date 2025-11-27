import React, { useEffect } from 'react';
import ReactFlow, {
  addEdge,
  useNodesState,
  useEdgesState,
  Background,
  Position,
} from 'reactflow';
import type { Connection, Edge, Node } from 'reactflow';
import 'reactflow/dist/style.css';
import type { VisualizationStep } from '../types';

import './Visualization.css';

const nodeStyle = {
  borderRadius: '50%',
  width: '50px',
  height: '50px',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  border: '2px solid #fff',
  background: '#1a1a1a',
  color: '#fff',
};

const cityIdMap: { [key: string]: number } = {
  "Beijing": 0, "Shenyang": 1, "Qingdao": 2, "Xian": 3, "Zhengzhou": 4,
  "Wuhan": 5, "Chengdu": 6, "Chongqing": 7, "Changsanjiao": 8, "Zhusanjiao": 9,
};

// const idToCityMap = Object.fromEntries(Object.entries(cityIdMap).map(([key, value]) => [value, key]));


const initialNodes: Node[] = [
  { id: 'Beijing', style: nodeStyle, position: { x: 450, y: 150 }, data: { label: 'Beijing' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Shenyang', style: nodeStyle, position: { x: 680, y: 80 }, data: { label: 'Shenyang' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Qingdao', style: nodeStyle, position: { x: 600, y: 280 }, data: { label: 'Qingdao' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Xian', style: nodeStyle, position: { x: 300, y: 350 }, data: { label: 'Xian' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Zhengzhou', style: nodeStyle, position: { x: 450, y: 380 }, data: { label: 'Zhengzhou' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Wuhan', style: nodeStyle, position: { x: 480, y: 530 }, data: { label: 'Wuhan' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Chengdu', style: nodeStyle, position: { x: 150, y: 500 }, data: { label: 'Chengdu' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Chongqing', style: nodeStyle, position: { x: 280, y: 580 }, data: { label: 'Chongqing' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Changsanjiao', style: nodeStyle, position: { x: 680, y: 500 }, data: { label: 'Changsanjiao' }, sourcePosition: Position.Bottom, targetPosition: Position.Top },
  { id: 'Zhusanjiao', style: nodeStyle, position: { x: 480, y: 850 }, data: { label: 'Zhusanjiao' }, sourcePosition: Position.Top, targetPosition: Position.Top },
];

const initialEdges: Edge[] = [
  { id: 'e-shenyang-beijing', source: 'Shenyang', target: 'Beijing', label: '750' },
  { id: 'e-shenyang-qingdao', source: 'Shenyang', target: 'Qingdao', label: '680' },
  { id: 'e-beijing-qingdao', source: 'Beijing', target: 'Qingdao', label: '800' },
  { id: 'e-beijing-xian', source: 'Beijing', target: 'Xian', label: '1140' },
  { id: 'e-beijing-zhengzhou', source: 'Beijing', target: 'Zhengzhou', label: '650' },
  { id: 'e-xian-zhengzhou', source: 'Xian', target: 'Zhengzhou', label: '570' },
  { id: 'e-qingdao-zhengzhou', source: 'Qingdao', target: 'Zhengzhou', label: '820' },
  { id: 'e-zhengzhou-wuhan', source: 'Zhengzhou', target: 'Wuhan', label: '530' },
  { id: 'e-xian-chengdu', source: 'Xian', target: 'Chengdu', label: '840' },
  { id: 'e-chengdu-chongqing', source: 'Chengdu', target: 'Chongqing', label: '340' },
  { id: 'e-wuhan-chongqing', source: 'Wuhan', target: 'Chongqing', label: '900' },
  { id: 'e-zhengzhou-changsanjiao', source: 'Zhengzhou', target: 'Changsanjiao', label: '1200' },
  { id: 'e-qingdao-changsanjiao', source: 'Qingdao', target: 'Changsanjiao', label: '960' },
  { id: 'e-wuhan-changsanjiao', source: 'Wuhan', target: 'Changsanjiao', label: '680' },
  { id: 'e-chongqing-zhusanjiao', source: 'Chongqing', target: 'Zhusanjiao', label: '2500' },
  { id: 'e-wuhan-zhusanjiao', source: 'Wuhan', target: 'Zhusanjiao', label: '1380' },
  { id: 'e-zhusanjiao-changsanjiao', source: 'Zhusanjiao', target: 'Changsanjiao', label: '2600' },
];

interface VisualizationProps {
  currentStep: VisualizationStep | null;
}

const Visualization: React.FC<VisualizationProps> = ({ currentStep }) => {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  useEffect(() => {
    if (currentStep) {
      const newNodes = initialNodes.map(node => {
        const isVisited = currentStep.visited_nodes.includes(cityIdMap[node.id]);
        const isCurrent = currentStep.current_node === cityIdMap[node.id];
        return {
          ...node,
          style: {
            ...node.style,
            background: isCurrent ? '#ff0072' : (isVisited ? '#00ff00' : '#1a1a1a'),
          },
        };
      });
      setNodes(newNodes);

      const newEdges = initialEdges.map(edge => {
        const sourceId = cityIdMap[edge.source!];
        const targetId = cityIdMap[edge.target!];
        const isInPath = currentStep.edges_in_path.some(
          ([from, to]) => (from === sourceId && to === targetId) || (from === targetId && to === sourceId)
        );
        const isCandidate = currentStep.candidate_edges.some(
          ([from, to]) => (from === sourceId && to === targetId) || (from === targetId && to === sourceId)
        );
        return {
          ...edge,
          animated: isCandidate,
          style: {
            ...edge.style,
            stroke: isInPath ? '#ff0072' : (isCandidate ? '#00ff00' : '#ffffff'),
          },
        };
      });
      setEdges(newEdges);
    } else {
      setNodes(initialNodes);
      setEdges(initialEdges);
    }
  }, [currentStep, setNodes, setEdges]);

  const onConnect = React.useCallback(
    (params: Edge | Connection) => setEdges((eds: Edge[]) => addEdge(params, eds)),
    [setEdges]
  );

  return (
    <div className="visualization-area">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
        proOptions={{ hideAttribution: true }}
      >
        <Background />
      </ReactFlow>
    </div>
  );
};

export default Visualization;

