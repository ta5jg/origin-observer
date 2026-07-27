// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/network.rs
// Purpose : Blockchain network domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain network domain model.
//!
//! A blockchain describes a protocol family, while a [`Network`] represents a
//! concrete deployment of that blockchain, such as Ethereum Mainnet, Sepolia,
//! BNB Smart Chain Mainnet, TRON Nile or a local development network.

use oo_core::{BlockchainId, NetworkId};

/// General classification of a blockchain network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NetworkKind {
    /// Production network carrying real economic value.
    Mainnet,

    /// Public test network.
    Testnet,

    /// Development-oriented public network.
    Devnet,

    /// Private or local development network.
    Local,

    /// Network whose classification is project-specific.
    Custom,
}

impl NetworkKind {
    /// Returns whether the network is intended for production use.
    #[must_use]
    pub const fn is_production(self) -> bool {
        matches!(self, Self::Mainnet)
    }

    /// Returns whether the network is intended for testing or development.
    #[must_use]
    pub const fn is_development(self) -> bool {
        matches!(self, Self::Testnet | Self::Devnet | Self::Local)
    }
}

/// RPC endpoint associated with a blockchain network.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RpcEndpoint {
    url: String,
    enabled: bool,
}

impl RpcEndpoint {
    /// Creates an enabled RPC endpoint.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            enabled: true,
        }
    }

    /// Returns the endpoint URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns whether the endpoint is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables the endpoint.
    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the endpoint.
    pub const fn disable(&mut self) {
        self.enabled = false;
    }
}

/// A concrete network belonging to a blockchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    id: NetworkId,
    blockchain_id: BlockchainId,
    kind: NetworkKind,
    name: String,
    short_name: String,
    chain_id: Option<u64>,
    rpc_endpoints: Vec<RpcEndpoint>,
    explorer_url: Option<String>,
    enabled: bool,
}

impl Network {
    /// Creates a new blockchain network.
    #[must_use]
    pub fn new(
        blockchain_id: BlockchainId,
        kind: NetworkKind,
        name: impl Into<String>,
        short_name: impl Into<String>,
    ) -> Self {
        Self {
            id: NetworkId::new(),
            blockchain_id,
            kind,
            name: name.into(),
            short_name: short_name.into(),
            chain_id: None,
            rpc_endpoints: Vec::new(),
            explorer_url: None,
            enabled: true,
        }
    }

    /// Returns the network identifier.
    #[must_use]
    pub const fn id(&self) -> NetworkId {
        self.id
    }

    /// Returns the parent blockchain identifier.
    #[must_use]
    pub const fn blockchain_id(&self) -> BlockchainId {
        self.blockchain_id
    }

    /// Returns the network classification.
    #[must_use]
    pub const fn kind(&self) -> NetworkKind {
        self.kind
    }

    /// Returns the network name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the abbreviated network name.
    #[must_use]
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Returns the numeric chain identifier when one is defined.
    #[must_use]
    pub const fn chain_id(&self) -> Option<u64> {
        self.chain_id
    }

    /// Assigns the numeric chain identifier.
    pub const fn set_chain_id(&mut self, chain_id: u64) {
        self.chain_id = Some(chain_id);
    }

    /// Removes the numeric chain identifier.
    pub const fn clear_chain_id(&mut self) {
        self.chain_id = None;
    }

    /// Returns all configured RPC endpoints.
    #[must_use]
    pub fn rpc_endpoints(&self) -> &[RpcEndpoint] {
        &self.rpc_endpoints
    }

    /// Returns enabled RPC endpoints.
    pub fn enabled_rpc_endpoints(&self) -> impl Iterator<Item = &RpcEndpoint> {
        self.rpc_endpoints
            .iter()
            .filter(|endpoint| endpoint.is_enabled())
    }

    /// Adds an RPC endpoint unless the URL is already registered.
    ///
    /// Returns `true` when a new endpoint is added and `false` when an
    /// endpoint with the same URL already exists.
    pub fn add_rpc_endpoint(&mut self, url: impl Into<String>) -> bool {
        let url = url.into();

        if self
            .rpc_endpoints
            .iter()
            .any(|endpoint| endpoint.url() == url)
        {
            return false;
        }

        self.rpc_endpoints.push(RpcEndpoint::new(url));
        true
    }

    /// Removes an RPC endpoint by URL.
    ///
    /// Returns `true` when an endpoint is removed.
    pub fn remove_rpc_endpoint(&mut self, url: &str) -> bool {
        let original_len = self.rpc_endpoints.len();

        self.rpc_endpoints.retain(|endpoint| endpoint.url() != url);

        self.rpc_endpoints.len() != original_len
    }

    /// Returns the block explorer URL when configured.
    #[must_use]
    pub fn explorer_url(&self) -> Option<&str> {
        self.explorer_url.as_deref()
    }

    /// Assigns the block explorer URL.
    pub fn set_explorer_url(&mut self, explorer_url: impl Into<String>) {
        self.explorer_url = Some(explorer_url.into());
    }

    /// Removes the block explorer URL.
    pub fn clear_explorer_url(&mut self) {
        self.explorer_url = None;
    }

    /// Returns whether the network is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables the network.
    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the network.
    pub const fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns whether the network is a production network.
    #[must_use]
    pub const fn is_mainnet(&self) -> bool {
        self.kind.is_production()
    }

    /// Returns whether the network is intended for development or testing.
    #[must_use]
    pub const fn is_development(&self) -> bool {
        self.kind.is_development()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ethereum_network(kind: NetworkKind) -> Network {
        Network::new(BlockchainId::new(), kind, "Ethereum", "eth")
    }

    #[test]
    fn creates_enabled_network() {
        let network = ethereum_network(NetworkKind::Mainnet);

        assert!(network.is_enabled());
        assert!(network.is_mainnet());
        assert!(!network.is_development());
        assert_eq!(network.name(), "Ethereum");
        assert_eq!(network.short_name(), "eth");
    }

    #[test]
    fn testnet_is_development_network() {
        let network = Network::new(BlockchainId::new(), NetworkKind::Testnet, "Sepolia", "sep");

        assert!(!network.is_mainnet());
        assert!(network.is_development());
    }

    #[test]
    fn chain_id_can_be_managed() {
        let mut network = ethereum_network(NetworkKind::Mainnet);

        assert_eq!(network.chain_id(), None);

        network.set_chain_id(1);
        assert_eq!(network.chain_id(), Some(1));

        network.clear_chain_id();
        assert_eq!(network.chain_id(), None);
    }

    #[test]
    fn rpc_endpoints_can_be_added_and_removed() {
        let mut network = ethereum_network(NetworkKind::Mainnet);

        assert!(network.add_rpc_endpoint("https://rpc.example.invalid"));

        assert!(!network.add_rpc_endpoint("https://rpc.example.invalid"));

        assert_eq!(network.rpc_endpoints().len(), 1);

        assert!(network.remove_rpc_endpoint("https://rpc.example.invalid"));

        assert!(network.rpc_endpoints().is_empty());
    }

    #[test]
    fn disabled_rpc_endpoints_are_filtered() {
        let mut network = ethereum_network(NetworkKind::Mainnet);

        network.add_rpc_endpoint("https://rpc-one.example.invalid");
        network.add_rpc_endpoint("https://rpc-two.example.invalid");

        network.rpc_endpoints[0].disable();

        let enabled: Vec<_> = network.enabled_rpc_endpoints().collect();

        assert_eq!(enabled.len(), 1);
        assert_eq!(enabled[0].url(), "https://rpc-two.example.invalid");
    }

    #[test]
    fn explorer_url_can_be_managed() {
        let mut network = ethereum_network(NetworkKind::Mainnet);

        assert_eq!(network.explorer_url(), None);

        network.set_explorer_url("https://explorer.example.invalid");

        assert_eq!(
            network.explorer_url(),
            Some("https://explorer.example.invalid")
        );

        network.clear_explorer_url();

        assert_eq!(network.explorer_url(), None);
    }

    #[test]
    fn network_can_be_disabled_and_enabled() {
        let mut network = ethereum_network(NetworkKind::Mainnet);

        network.disable();
        assert!(!network.is_enabled());

        network.enable();
        assert!(network.is_enabled());
    }

    #[test]
    fn network_identifiers_are_unique() {
        let blockchain_id = BlockchainId::new();

        let first = Network::new(blockchain_id, NetworkKind::Mainnet, "Ethereum", "eth");

        let second = Network::new(blockchain_id, NetworkKind::Testnet, "Sepolia", "sep");

        assert_ne!(first.id(), second.id());
        assert_eq!(first.blockchain_id(), second.blockchain_id());
    }
}
