import React from 'react';
import { useStore } from '../hooks/useStore';
import { architectureNodes, categoryGroups, views } from '../data/architecture';
import { ChevronRight, Search, Filter, Layers, Minimize, Maximize } from 'lucide-react';

export const Sidebar = () => {
  const { 
    activeView, 
    setActiveView, 
    selectedNode, 
    filterCategory, 
    setFilterCategory,
    filterText,
    setFilterText,
    sidebarOpen,
    toggleSidebar,
  } = useStore();

  const filteredNodes = architectureNodes.filter(node => {
    const matchesCategory = !filterCategory || node.category === filterCategory;
    const matchesText = !filterText || 
      node.name.toLowerCase().includes(filterText.toLowerCase()) ||
      node.description.toLowerCase().includes(filterText.toLowerCase()) ||
      node.tags.some(t => t.toLowerCase().includes(filterText.toLowerCase()));
    return matchesCategory && matchesText;
  });

  const categories = [...new Set(architectureNodes.map(n => n.category))].sort();

  return (
    <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
      <div className="sidebar-section">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
          <span className="sidebar-title">Views</span>
          <button 
            onClick={toggleSidebar}
            className="control-btn"
            style={{ padding: '0.25rem' }}
            aria-label="Toggle sidebar"
          >
            {sidebarOpen ? <Minimize size={18} /> : <Maximize size={18} />}
          </button>
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.375rem' }}>
          {views.map(view => (
            <button
              key={view.id}
              onClick={() => setActiveView(view.id)}
              className={`legend-item ${activeView === view.id ? 'active' : ''}`}
              style={{ 
                justifyContent: 'flex-start',
                padding: '0.5rem 0.75rem',
                borderRadius: 'var(--radius-sm)',
                background: activeView === view.id ? 'var(--bg-tertiary)' : 'transparent',
              }}
              title={view.description}
            >
              <Layers size={16} style={{ color: activeView === view.id ? 'var(--accent-blue)' : 'var(--text-secondary)' }} />
              <span style={{ fontSize: '0.875rem', fontWeight: 500 }}>{view.label}</span>
            </button>
          ))}
        </div>
      </div>

      <div className="sidebar-section">
        <span className="sidebar-title">Filter</span>
        <div className="control-group" style={{ flexDirection: 'column', gap: '0.75rem', alignItems: 'stretch' }}>
          <div style={{ position: 'relative' }}>
            <Search size={16} style={{ position: 'absolute', left: '0.75rem', top: '50%', transform: 'translateY(-50%)', color: 'var(--text-secondary)', pointerEvents: 'none' }} />
            <input
              type="text"
              placeholder="Search nodes..."
              value={filterText}
              onChange={(e) => setFilterText(e.target.value)}
              style={{
                width: '100%',
                padding: '0.5rem 0.5rem 0.5rem 2.5rem',
                background: 'var(--bg-tertiary)',
                border: '1px solid var(--border-color)',
                borderRadius: 'var(--radius-sm)',
                color: 'var(--text-primary)',
                fontSize: '0.875rem',
              }}
            />
          </div>
          <select
            value={filterCategory}
            onChange={(e) => setFilterCategory(e.target.value)}
            className="control-select"
            style={{ width: '100%' }}
          >
            <option value="">All Categories</option>
            {categories.map(cat => (
              <option key={cat} value={cat}>{cat}</option>
            ))}
          </select>
        </div>
      </div>

      <div className="sidebar-section">
        <span className="sidebar-title">Categories ({filteredNodes.length} nodes)</span>
        <div className="legend" style={{ maxHeight: '300px', overflowY: 'auto' }}>
          {categoryGroups
            .filter(g => filteredNodes.some(n => g.nodes.includes(n.id)))
            .map(group => (
              <div
                key={group.id}
                className="legend-item"
                style={{ 
                  borderLeft: `3px solid ${group.color}`,
                  padding: '0.5rem 0.75rem',
                }}
              >
                <span 
                  className="legend-color" 
                  style={{ background: group.color }}
                />
                <span className="legend-text" style={{ flex: 1 }}>
                  {group.label}
                </span>
                <span style={{ 
                  fontSize: '0.75rem', 
                  color: 'var(--text-secondary)',
                  background: 'var(--bg-tertiary)',
                  padding: '0.125rem 0.375rem',
                  borderRadius: '999px',
                }}>
                  {filteredNodes.filter(n => group.nodes.includes(n.id)).length}
                </span>
              </div>
            ))}
        </div>
      </div>

      {selectedNode && (
        <div className="node-info">
          <div className="node-info-title">{selectedNode.data.name}</div>
          <div className="node-info-detail">
            <strong>Category:</strong> {selectedNode.data.category}
          </div>
          <div className="node-info-detail">
            <strong>Language:</strong> {selectedNode.data.language}
          </div>
          <div className="node-info-detail">
            <strong>License:</strong> {selectedNode.data.license}
          </div>
          <div className="node-info-detail">
            <strong>Crates:</strong> {selectedNode.data.crates.join(', ')}
          </div>
          <div className="node-info-detail">
            <strong>Tags:</strong> {selectedNode.data.tags.join(', ')}
          </div>
        </div>
      )}
    </aside>
  );
};