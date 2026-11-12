import React, { useCallback, useEffect, useRef } from 'react';
import ReactFlow, { 
  Background, 
  Controls, 
  Minimap, 
  NodeTypes, 
  EdgeTypes,
  Viewport,
  Connection,
  addEdge,
  Node,
  Edge,
  OnNodesChange,
  OnEdgesChange,
  OnConnect,
  OnSelectionChange,
  PanelPosition,
  HandleType,
} from 'reactflow';
import 'reactflow/dist/style.css';

import { useStore } from './hooks/useStore';
import { CustomNode } from './components/CustomNode';
import { CustomEdge } from './components/CustomEdge';
import { Sidebar } from './components/Sidebar';
import { architectureNodes, architectureEdges, categoryGroups, views } from './data/architecture';
import { NodeData, EdgeData } from './data/architecture';
import { Menu, ZoomIn, ZoomOut, RotateCcw, Home, Layers, HelpCircle, ChevronDown, GitBranch, ExternalLink } from 'lucide-react';

const nodeTypes: NodeTypes = {
  custom: CustomNode,
};

const edgeTypes: EdgeTypes = {
  custom: CustomEdge,
};

const initialNodes: Node[] = architectureNodes.map(node => ({
  id: node.id,
  type: 'custom',
  position: node.position,
  data: node,
  style: {
    width: 220,
  },
}));

const initialEdges: Edge[] = architectureEdges.map(edge => ({
  id: edge.id,
  source: edge.source,
  target: edge.target,
  type: 'custom',
  animated: edge.type === 'data-flow',
  label: edge.label,
  style: {
    strokeWidth: edge.type === 'data-flow' ? 2 : 1.5,
  },
}));

export const App = () => {
  const {
    activeView,
    setActiveView,
    sidebarOpen,
    filterCategory,
    filterText,
    selectedNode,
    setSelectedNode,
    selectedEdge,
    setSelectedEdge,
    viewport,
    setViewport,
    getFilteredNodes,
    getFilteredEdges,
  } = useStore();

  const reactFlowRef = useRef<ReactFlow<unknown>>(null);
  const [fitView, setFitView] = React.useState(false);

  // Handle view changes
  useEffect(() => {
    const view = views.find(v => v.id === activeView);
    if (view && reactFlowRef.current) {
      const viewNodes = architectureNodes.filter(n => view.groups.some(g => 
        categoryGroups.find(cg => cg.id === g)?.nodes.includes(n.id)
      ));
      if (viewNodes.length > 0) {
        reactFlowRef.current.fitView({ 
          nodes: viewNodes.map(n => ({ id: n.id, position: n.position, width: 220, height: 100 })),
          padding: 0.2,
          duration: 500,
        });
      }
    }
  }, [activeView]);

  const onNodesChange: OnNodesChange = useCallback((changes) => {
    // Nodes are not draggable in this visualization
  }, []);

  const onEdgesChange: OnEdgesChange = useCallback((changes) => {
    // Edges are not editable
  }, []);

  const onConnect: OnConnect = useCallback((params) => {
    // Connections not allowed in this visualization
  }, []);

  const onSelectionChange: OnSelectionChange = useCallback(({ nodes, edges }) => {
    if (nodes.length > 0) {
      const nodeData = architectureNodes.find(n => n.id === nodes[0]);
      if (nodeData) setSelectedNode(nodeData);
    } else {
      setSelectedNode(null);
    }
    
    if (edges.length > 0) {
      const edgeData = architectureEdges.find(e => e.id === edges[0]);
      if (edgeData) setSelectedEdge(edgeData);
    } else {
      setSelectedEdge(null);
    }
  }, [setSelectedNode, setSelectedEdge]);

  const onViewportChange: ReactFlow['onViewportChange'] = useCallback((viewport) => {
    setViewport(viewport);
  }, [setViewport]);

  const handleFitView = useCallback(() => {
    reactFlowRef.current?.fitView({ padding: 0.1, duration: 500 });
  }, []);

  const handleZoomIn = useCallback(() => {
    reactFlowRef.current?.zoomIn();
  }, []);

  const handleZoomOut = useCallback(() => {
    reactFlowRef.current?.zoomOut();
  }, []);

  const handleReset = useCallback(() => {
    reactFlowRef.current?.setViewport({ x: 0, y: 0, zoom: 1 }, { duration: 500 });
  }, []);

  const filteredNodes = getFilteredNodes();
  const filteredEdges = getFilteredEdges();

  return (
    <div className="app">
      <header className="header">
        <div className="header-content">
          <a href="https://github.com/nakisisageorge/cua-enhanced" className="logo" target="_blank" rel="noopener noreferrer">
            <div className="logo-icon">
              <GitBranch size={20} style={{ color: 'white' }} />
            </div>
            <span className="logo-text">Cua Enhanced</span>
          </a>
          
          <nav className="nav-tabs" role="tablist" aria-label="Architecture views">
            {views.map(view => (
              <button
                key={view.id}
                role="tab"
                aria-selected={activeView === view.id}
                onClick={() => setActiveView(view.id)}
                className={`nav-tab ${activeView === view.id ? 'active' : ''}`}
                title={view.description}
              >
                {view.label}
              </button>
            ))}
          </nav>

          <div className="controls">
            <div className="control-group">
              <span className="control-label">Category</span>
              <select
                value={filterCategory}
                onChange={(e) => useStore.getState().setFilterCategory(e.target.value)}
                className="control-select"
                aria-label="Filter by category"
              >
                <option value="">All Categories</option>
                {categoryGroups.map(cat => (
                  <option key={cat.id} value={cat.label}>{cat.label}</option>
                ))}
              </select>
            </div>
            <div className="control-group" style={{ position: 'relative' }}>
              <Search size={16} style={{ position: 'absolute', left: '0.75rem', top: '50%', transform: 'translateY(-50%)', color: 'var(--text-secondary)', pointerEvents: 'none' }} />
              <input
                type="text"
                placeholder="Search nodes..."
                value={filterText}
                onChange={(e) => useStore.getState().setFilterText(e.target.value)}
                className="control-select"
                style={{ paddingLeft: '2.5rem', width: '200px' }}
                aria-label="Search nodes"
              />
            </div>
          </div>
        </div>
      </header>

      <main className="main">
        <Sidebar />
        
        <div className="canvas-container">
          <ReactFlow
            ref={reactFlowRef}
            nodes={filteredNodes.map(node => initialNodes.find(n => n.id === node.id)!).filter(Boolean)}
            edges={filteredEdges.map(edge => initialEdges.find(e => e.id === edge.id)!).filter(Boolean)}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            onSelectionChange={onSelectionChange}
            onViewportChange={onViewportChange}
            viewport={viewport}
            fitView={false}
            attributionPosition={PanelPosition.BottomRight}
            nodesDraggable={false}
            nodesConnectable={false}
            elementsSelectable={true}
            selectNodesOnDrag={false}
            connectionMode={HandleType.Target}
            minZoom={0.1}
            maxZoom={2}
          >
            <Background 
              color="#30363d" 
              gap={16} 
              size={1}
              style={{ opacity: 0.3 }}
            />
            <Controls 
              position={PanelPosition.TopLeft}
              showZoom={true}
              showFitView={true}
              showInteractive={false}
            />
            <Minimap 
              position={PanelPosition.BottomRight}
              nodeColor={(node) => {
                const nodeData = architectureNodes.find(n => n.id === node.id);
                return nodeData?.color || '#58a6ff';
              }}
              maskColor="rgba(13, 17, 23, 0.9)"
            />
          </ReactFlow>

          <div className="controls-panel">
            <button 
              className="control-btn" 
              onClick={handleFitView}
              aria-label="Fit view"
              title="Fit view"
            >
              <Home size={18} />
            </button>
            <button 
              className="control-btn" 
              onClick={handleZoomIn}
              aria-label="Zoom in"
              title="Zoom in"
            >
              <ZoomIn size={18} />
            </button>
            <button 
              className="control-btn" 
              onClick={handleZoomOut}
              aria-label="Zoom out"
              title="Zoom out"
            >
              <ZoomOut size={18} />
            </button>
            <button 
              className="control-btn" 
              onClick={handleReset}
              aria-label="Reset view"
              title="Reset view"
            >
              <RotateCcw size={18} />
            </button>
          </div>

          {selectedNode && (
            <div 
              className="tooltip"
              style={{
                top: '80px',
                right: '20px',
                maxWidth: '350px',
              }}
            >
              <div className="tooltip-title">{selectedNode.data.name}</div>
              <div className="tooltip-row">
                <span className="tooltip-label">Category</span>
                <span className="tooltip-value">{selectedNode.data.category}</span>
              </div>
              <div className="tooltip-row">
                <span className="tooltip-label">Language</span>
                <span className="tooltip-value">{selectedNode.data.language}</span>
              </div>
              <div className="tooltip-row">
                <span className="tooltip-label">License</span>
                <span className="tooltip-value">{selectedNode.data.license}</span>
              </div>
              <div className="tooltip-row">
                <span className="tooltip-label">Crates</span>
                <span className="tooltip-value" style={{ fontSize: '0.7rem' }}>{selectedNode.data.crates.join(', ')}</span>
              </div>
            </div>
          )}

          {selectedEdge && (
            <div 
              className="tooltip"
              style={{
                bottom: '100px',
                left: '20px',
                maxWidth: '300px',
              }}
            >
              <div className="tooltip-title">Connection</div>
              <div className="tooltip-row">
                <span className="tooltip-label">Type</span>
                <span className="tooltip-value">{selectedEdge.type}</span>
              </div>
              {selectedEdge.label && (
                <div className="tooltip-row">
                  <span className="tooltip-label">Label</span>
                  <span className="tooltip-value">{selectedEdge.label}</span>
                </div>
              )}
              <div className="tooltip-row">
                <span className="tooltip-label">Source</span>
                <span className="tooltip-value">{selectedEdge.source}</span>
              </div>
              <div className="tooltip-row">
                <span className="tooltip-label">Target</span>
                <span className="tooltip-value">{selectedEdge.target}</span>
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
};

export default App;