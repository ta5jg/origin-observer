// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/graph.rs
// Purpose : Observation graph domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Observation graph.
//!
//! The graph is the central relationship model of Origin Observer.
//! It does **not** store blockchain data itself.
//! Instead it stores immutable relationships between already existing
//! domain objects.
//!
//! Address ─────► Transaction
//! Address ─────► Contract
//! Transaction ─► Block
//! Transaction ─► Asset
//! Contract ────► Asset
//! Provider ────► Evidence
//! Evidence ────► Snapshot
//! Snapshot ────► Session
//!
//! Higher layers build temporal analysis on top of this graph.

use std::collections::{BTreeMap, BTreeSet};

use oo_core::{
    AddressId, AssetId, BlockId, ContractId, EvidenceId, GraphId, ProviderId, SessionId,
    SnapshotId, TransactionId,
};

/// Relationship type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphRelation {
    AddressToTransaction,
    AddressToContract,
    TransactionToBlock,
    TransactionToAsset,
    ContractToAsset,
    ProviderToEvidence,
    EvidenceToSnapshot,
    SnapshotToSession,
}

/// Generic graph node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphNode {
    Address(AddressId),
    Asset(AssetId),
    Block(BlockId),
    Contract(ContractId),
    Transaction(TransactionId),
    Provider(ProviderId),
    Evidence(EvidenceId),
    Snapshot(SnapshotId),
    Session(SessionId),
}

/// Graph edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: GraphNode,
    pub to: GraphNode,
    pub relation: GraphRelation,
}

/// Observation graph.
#[derive(Debug, Clone)]
pub struct Graph {
    id: GraphId,

    nodes: BTreeSet<GraphNode>,

    edges: Vec<GraphEdge>,

    adjacency: BTreeMap<GraphNode, BTreeSet<usize>>,
}

impl Graph {
    /// Creates an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: GraphId::new(),
            nodes: BTreeSet::new(),
            edges: Vec::new(),
            adjacency: BTreeMap::new(),
        }
    }

    #[must_use]
    pub const fn id(&self) -> GraphId {
        self.id
    }

    /// Adds a node.
    pub fn add_node(&mut self, node: GraphNode) -> bool {
        self.nodes.insert(node)
    }

    /// Adds an edge.
    pub fn add_edge(&mut self, from: GraphNode, to: GraphNode, relation: GraphRelation) {
        self.nodes.insert(from);
        self.nodes.insert(to);

        let index = self.edges.len();

        self.edges.push(GraphEdge { from, to, relation });

        self.adjacency.entry(from).or_default().insert(index);
    }

    #[must_use]
    pub fn contains(&self, node: GraphNode) -> bool {
        self.nodes.contains(&node)
    }

    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns outgoing edges.
    #[must_use]
    pub fn outgoing(&self, node: GraphNode) -> Vec<&GraphEdge> {
        self.adjacency
            .get(&node)
            .map(|set| set.iter().map(|i| &self.edges[*i]).collect())
            .unwrap_or_default()
    }

    #[must_use]
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    #[must_use]
    pub fn nodes(&self) -> &BTreeSet<GraphNode> {
        &self.nodes
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.adjacency.clear();
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn unique_graph_ids() {
        let a = Graph::new();
        let b = Graph::new();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn add_nodes() {
        let mut graph = Graph::new();

        let address = GraphNode::Address(AddressId::new());

        assert!(graph.add_node(address));

        assert!(graph.contains(address));

        assert_eq!(graph.node_count(), 1,);
    }

    #[test]
    fn add_edge() {
        let mut graph = Graph::new();

        let address = GraphNode::Address(AddressId::new());

        let tx = GraphNode::Transaction(TransactionId::new());

        graph.add_edge(address, tx, GraphRelation::AddressToTransaction);

        assert_eq!(graph.node_count(), 2,);

        assert_eq!(graph.edge_count(), 1,);

        let edges = graph.outgoing(address);

        assert_eq!(edges.len(), 1,);

        assert_eq!(edges[0].relation, GraphRelation::AddressToTransaction,);
    }

    #[test]
    fn clear_graph() {
        let mut graph = Graph::new();

        graph.add_edge(
            GraphNode::Address(AddressId::new()),
            GraphNode::Transaction(TransactionId::new()),
            GraphRelation::AddressToTransaction,
        );

        graph.clear();

        assert!(graph.is_empty());

        assert_eq!(graph.edge_count(), 0,);
    }
}
