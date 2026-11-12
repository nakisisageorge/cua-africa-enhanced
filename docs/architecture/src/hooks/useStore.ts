import { create } from 'zustand';
import { architectureNodes, architectureEdges, NodeData, EdgeData } from '../data/architecture';

interface StoreState {
  // View state
  activeView: string;
  setActiveView: (view: string) => void;
  
  // Sidebar state
  sidebarOpen: boolean;
  toggleSidebar: () => void;
  
  // Filter state
  filterCategory: string;
  setFilterCategory: (category: string) => void;
  filterText: string;
  setFilterText: (text: string) => void;
  
  // Selection state
  selectedNode: NodeData | null;
  setSelectedNode: (node: NodeData | null) => void;
  selectedEdge: EdgeData | null;
  setSelectedEdge: (edge: EdgeData | null) => void;
  
  // Viewport state
  viewport: { x: number; y: number; zoom: number };
  setViewport: (viewport: { x: number; y: number; zoom: number }) => void;
  
  // Data getters
  getFilteredNodes: () => NodeData[];
  getFilteredEdges: () => EdgeData[];
}

export const useStore = create<StoreState>((set, get) => ({
  activeView: 'overview',
  setActiveView: (view) => set({ activeView: view }),
  
  sidebarOpen: true,
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
  
  filterCategory: '',
  setFilterCategory: (category) => set({ filterCategory: category }),
  filterText: '',
  setFilterText: (text) => set({ filterText: text }),
  
  selectedNode: null,
  setSelectedNode: (node) => set({ selectedNode: node }),
  selectedEdge: null,
  setSelectedEdge: (edge) => set({ selectedEdge: edge }),
  
  viewport: { x: 0, y: 0, zoom: 1 },
  setViewport: (viewport) => set({ viewport }),
  
  getFilteredNodes: () => {
    const { filterCategory, filterText } = get();
    return architectureNodes.filter(node => {
      const matchesCategory = !filterCategory || node.category === filterCategory;
      const matchesText = !filterText || 
        node.name.toLowerCase().includes(filterText.toLowerCase()) ||
        node.description.toLowerCase().includes(filterText.toLowerCase()) ||
        node.tags.some(t => t.toLowerCase().includes(filterText.toLowerCase()));
      return matchesCategory && matchesText;
    });
  },
  
  getFilteredEdges: () => {
    const { filterCategory, filterText } = get();
    const filteredNodeIds = new Set(get().getFilteredNodes().map(n => n.id));
    return architectureEdges.filter(edge => 
      filteredNodeIds.has(edge.source) && filteredNodeIds.has(edge.target)
    );
  },
}));