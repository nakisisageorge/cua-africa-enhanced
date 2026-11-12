import React, { memo, useState } from 'react';
import { Handle, Position, NodeProps } from 'reactflow';
import { LucideIcon, Server, Cpu, Database, Globe, Shield, Monitor, Smartphone, Cloud, Network, Terminal, Box, Microscope, Zap, Layers, BookOpen } from 'lucide-react';

interface CustomNodeProps extends NodeProps {
  data: {
    name: string;
    category: string;
    description: string;
    crates: string[];
    language: string;
    license: string;
    tags: string[];
    color: string;
  };
}

const categoryIcons: Record<string, LucideIcon> = {
  'SDK Core': Server,
  'SDK Bindings': Globe,
  'Cua Driver': Monitor,
  'Cua Spaces': Layers,
  'Virtualization': Cpu,
  'Benchmarks': Microscope,
  'Media & Cluster': Network,
  'Security & Observability': Shield,
  'African Extensions': BookOpen,
};

const getCategoryIcon = (category: string): LucideIcon => {
  return categoryIcons[category] || Box;
};

export const CustomNode = memo(({ data, selected, isConnectable }: CustomNodeProps) => {
  const [expanded, setExpanded] = useState(false);
  const Icon = getCategoryIcon(data.category);
  
  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setExpanded(!expanded);
  };
  
  return (
    <div
      className={`custom-node ${selected ? 'selected' : ''}`}
      onClick={handleClick}
      style={{
        borderLeft: `4px solid ${data.color}`,
        background: selected ? 'rgba(88, 166, 255, 0.05)' : 'var(--bg-secondary)',
      }}
    >
      <div className="node-header">
        <div 
          className="node-icon" 
          style={{ background: `${data.color}20`, color: data.color }}
        >
          <Icon size={14} />
        </div>
        <span className="node-name" title={data.name}>{data.name}</span>
        <span className="node-category" style={{ borderColor: data.color, color: data.color }}>
          {data.category}
        </span>
      </div>
      
      <p className="node-description" title={data.description}>
        {data.description}
      </p>
      
      <div className="node-meta">
        <span className="node-tag" style={{ borderColor: data.color, color: data.color }}>
          {data.language}
        </span>
        <span className="node-tag" style={{ borderColor: data.color, color: data.color }}>
          {data.license}
        </span>
        {data.tags.slice(0, 3).map((tag) => (
          <span key={tag} className="node-tag">{tag}</span>
        ))}
        {data.tags.length > 3 && (
          <span className="node-tag">+{data.tags.length - 3} more</span>
        )}
      </div>
      
      {/* Connection handles */}
      <Handle type="target" position={Position.Left} id="left" className="react-flow__handle" style={{ background: data.color }} />
      <Handle type="source" position={Position.Right} id="right" className="react-flow__handle" style={{ background: data.color }} />
      <Handle type="target" position={Position.Top} id="top" className="react-flow__handle" style={{ background: data.color }} />
      <Handle type="source" position={Position.Bottom} id="bottom" className="react-flow__handle" style={{ background: data.color }} />
    </div>
  );
});

CustomNode.displayName = 'CustomNode';