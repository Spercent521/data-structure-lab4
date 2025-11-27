import React from 'react';
import './LeftSidebar.css';

interface LeftSidebarProps {
  algorithms: string[];
  selectedAlgorithm: string;
  onSelectAlgorithm: (algo: string) => void;
  onPlay: () => void;
  onNext: () => void;
  onPrev: () => void;
  onReset: () => void;
  isPlaying: boolean;
}

const LeftSidebar: React.FC<LeftSidebarProps> = ({
  algorithms,
  selectedAlgorithm,
  onSelectAlgorithm,
  onPlay,
  onNext,
  onPrev,
  onReset,
  isPlaying,
}) => {
  return (
    <div className="left-sidebar">
      <h1>Algorithm Visualizer</h1>
      <ul>
        {algorithms.map(algo => (
          <li
            key={algo}
            className={selectedAlgorithm === algo ? 'selected' : ''}
            onClick={() => onSelectAlgorithm(algo)}
          >
            {algo}
          </li>
        ))}
      </ul>
      <div className="controls">
        <button onClick={onPrev}>Prev</button>
        <button onClick={onPlay}>{isPlaying ? 'Pause' : 'Play'}</button>
        <button onClick={onNext}>Next</button>
        <button onClick={onReset}>Reset</button>
      </div>
    </div>
  );
};

export default LeftSidebar;
