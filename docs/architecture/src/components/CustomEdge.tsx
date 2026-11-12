import React from 'react';
import { Edge, EdgeProps } from 'reactflow';

interface CustomEdgeProps extends EdgeProps {
  animated?: boolean;
}

export const CustomEdge = ({ 
  animated = false, 
  style = {}, 
  ...props 
}: CustomEdgeProps) => {
  const edgeStyle = {
    stroke: animated ? 'var(--accent-blue)' : 'var(--border-color)',
    strokeWidth: animated ? 2 : 1.5,
    strokeDasharray: animated ? '5,5' : undefined,
    ...style,
  };

  return (
    <Edge
      type="smoothstep"
      style={edgeStyle}
      markerEnd={{
        type: 'arrowclosed',
        color: animated ? 'var(--accent-blue)' : 'var(--border-color)',
        width: 12,
        height: 12,
      }}
      animated={animated}
      {...props}
    />
  );
};