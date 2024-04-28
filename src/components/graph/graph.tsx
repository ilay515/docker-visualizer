import ReactFlow from 'reactflow'

import 'reactflow/dist/style.css';
import { useState } from 'react';

export interface GraphProps {
  size: {
    x: number,
    y: number
  },
  nodes: Node[]
}

const Graph = ({ nodes }: GraphProps) => {
  const [isLocked, setIsLocked] = useState<boolean>(true)

  return (
    <ReactFlow
      nodes={nodes}
      fitView
      preventScrolling={!isLocked}
      nodesDraggable={!isLocked}
      nodesConnectable={!isLocked}
      nodesFocusable={true}
      elementsSelectable={true}
    >
    </ReactFlow>
  );
};

export default Graph;

