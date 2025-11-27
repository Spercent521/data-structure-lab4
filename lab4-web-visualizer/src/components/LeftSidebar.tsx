import React from 'react';
import './LeftSidebar.css';

interface LeftSidebarProps {
  setSelectedAlgorithm: (algo: string) => void;
}

const LeftSidebar: React.FC<LeftSidebarProps> = ({ setSelectedAlgorithm }) => {
  return (
    <div className="left-sidebar">
      <h1>Algorithm Visualizer</h1>
      <ul>
        <li onClick={() => setSelectedAlgorithm('DFS')}>DFS</li>
        <li onClick={() => setSelectedAlgorithm('PRIM')}>PRIM</li>
        <li onClick={() => setSelectedAlgorithm('DIJKSTRA')}>DIJKSTRA</li>
      </ul>
    </div>
  );
};

export default LeftSidebar;
