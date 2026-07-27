// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/provider.rs
// Purpose : Blockchain data provider domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Blockchain data provider domain model.
//!
//! A [`Provider`] represents an RPC node, explorer API, indexer, archival node,
//! wallet service or another external source used to observe blockchain state.

use std::collections::BTreeSet;

use oo_core::error::invalid_argument;
use oo_core::{NetworkId, ProviderId, Result};

/// Maximum accepted provider name length.
pub const MAX_PROVIDER_NAME_LENGTH: usize = 128;

/// Maximum accepted endpoint URI length.
pub const MAX_ENDPOINT_URI_LENGTH: usize = 2_048;

/// General provider classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ProviderKind {
    /// Standard blockchain JSON-RPC provider.
    #[default]
    Rpc,

    /// WebSocket-based provider.
    WebSocket,

    /// Blockchain explorer API.
    Explorer,

    /// Blockchain indexer.
    Indexer,

    /// Archive node.
    ArchiveNode,

    /// Full node.
    FullNode,

    /// Light node.
    LightNode,

    /// Wallet or account service.
    WalletService,

    /// Price or market-data provider.
    MarketData,

    /// Metadata provider.
    Metadata,

    /// Custom provider implementation.
    Custom,

    /// Provider type has not yet been determined.
    Unknown,
}

/// Transport protocol used by a provider endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ProviderTransport {
    /// HTTP transport.
    Http,

    /// HTTPS transport.
    #[default]
    Https,

    /// WebSocket transport.
    WebSocket,

    /// Secure WebSocket transport.
    SecureWebSocket,

    /// Inter-process communication transport.
    Ipc,

    /// Custom transport.
    Custom,

    /// Transport has not yet been determined.
    Unknown,
}

/// Operational status of a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ProviderStatus {
    /// Provider may be used normally.
    #[default]
    Active,

    /// Provider is temporarily unavailable.
    Unavailable,

    /// Provider is rate-limited.
    RateLimited,

    /// Provider is disabled by configuration.
    Disabled,

    /// Provider is deprecated.
    Deprecated,
}

/// Authentication mechanism used by a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ProviderAuthentication {
    /// Provider requires no authentication.
    #[default]
    None,

    /// API key passed through a request header.
    ApiKeyHeader,

    /// API key passed through the query string.
    ApiKeyQuery,

    /// HTTP bearer token.
    BearerToken,

    /// HTTP basic authentication.
    Basic,

    /// Custom authentication method.
    Custom,
}

/// Capabilities exposed by a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProviderCapability {
    /// Reads current blockchain state.
    ReadState,

    /// Reads historical blockchain state.
    HistoricalState,

    /// Reads blocks.
    Blocks,

    /// Reads transactions.
    Transactions,

    /// Reads transaction receipts.
    Receipts,

    /// Reads logs and events.
    Logs,

    /// Reads account balances.
    Balances,

    /// Reads contract bytecode.
    ContractCode,

    /// Executes read-only contract calls.
    ContractCalls,

    /// Estimates transaction fees or gas.
    FeeEstimation,

    /// Broadcasts signed transactions.
    TransactionBroadcast,

    /// Supports pending transaction observation.
    PendingTransactions,

    /// Supports subscriptions over a persistent connection.
    Subscriptions,

    /// Supports tracing or debugging methods.
    Tracing,

    /// Supports archive-state queries.
    ArchiveQueries,

    /// Provides token or contract metadata.
    Metadata,

    /// Provides market or price data.
    MarketData,
}

/// Blockchain data provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    id: ProviderId,
    network_id: NetworkId,
    name: String,
    endpoint: String,
    kind: ProviderKind,
    transport: ProviderTransport,
    status: ProviderStatus,
    authentication: ProviderAuthentication,
    capabilities: BTreeSet<ProviderCapability>,
    priority: u16,
    request_timeout_ms: u64,
    max_retries: u8,
    rate_limit_per_second: Option<u32>,
    trusted: bool,
    label: Option<String>,
}

impl Provider {
    /// Creates a provider and detects its transport from the endpoint URI.
    pub fn new(
        network_id: NetworkId,
        name: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Result<Self> {
        let name = normalize_name(name.into())?;
        let endpoint = normalize_endpoint(endpoint.into())?;
        let transport = detect_transport(&endpoint);

        Ok(Self {
            id: ProviderId::new(),
            network_id,
            name,
            endpoint,
            kind: ProviderKind::Rpc,
            transport,
            status: ProviderStatus::Active,
            authentication: ProviderAuthentication::None,
            capabilities: BTreeSet::new(),
            priority: 100,
            request_timeout_ms: 30_000,
            max_retries: 3,
            rate_limit_per_second: None,
            trusted: false,
            label: None,
        })
    }

    /// Creates a provider with explicit kind and transport.
    pub fn with_kind(
        network_id: NetworkId,
        name: impl Into<String>,
        endpoint: impl Into<String>,
        kind: ProviderKind,
        transport: ProviderTransport,
    ) -> Result<Self> {
        let mut provider = Self::new(network_id, name, endpoint)?;

        provider.kind = kind;
        provider.transport = transport;

        Ok(provider)
    }

    /// Returns the provider identifier.
    #[must_use]
    pub const fn id(&self) -> ProviderId {
        self.id
    }

    /// Returns the associated network identifier.
    #[must_use]
    pub const fn network_id(&self) -> NetworkId {
        self.network_id
    }

    /// Changes the associated network.
    pub const fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    /// Returns the provider name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Changes the provider name.
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<()> {
        self.name = normalize_name(name.into())?;
        Ok(())
    }

    /// Returns the provider endpoint.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Changes the provider endpoint and redetects its transport.
    pub fn set_endpoint(&mut self, endpoint: impl Into<String>) -> Result<()> {
        let endpoint = normalize_endpoint(endpoint.into())?;

        self.transport = detect_transport(&endpoint);
        self.endpoint = endpoint;

        Ok(())
    }

    /// Returns the provider classification.
    #[must_use]
    pub const fn kind(&self) -> ProviderKind {
        self.kind
    }

    /// Changes the provider classification.
    pub const fn set_kind(&mut self, kind: ProviderKind) {
        self.kind = kind;
    }

    /// Returns the endpoint transport.
    #[must_use]
    pub const fn transport(&self) -> ProviderTransport {
        self.transport
    }

    /// Changes the endpoint transport.
    pub const fn set_transport(&mut self, transport: ProviderTransport) {
        self.transport = transport;
    }

    /// Returns the provider status.
    #[must_use]
    pub const fn status(&self) -> ProviderStatus {
        self.status
    }

    /// Changes the provider status.
    pub const fn set_status(&mut self, status: ProviderStatus) {
        self.status = status;
    }

    /// Activates the provider.
    pub const fn activate(&mut self) {
        self.status = ProviderStatus::Active;
    }

    /// Disables the provider.
    pub const fn disable(&mut self) {
        self.status = ProviderStatus::Disabled;
    }

    /// Marks the provider temporarily unavailable.
    pub const fn mark_unavailable(&mut self) {
        self.status = ProviderStatus::Unavailable;
    }

    /// Marks the provider as rate-limited.
    pub const fn mark_rate_limited(&mut self) {
        self.status = ProviderStatus::RateLimited;
    }

    /// Returns whether the provider can currently be selected.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        matches!(self.status, ProviderStatus::Active)
    }

    /// Returns the authentication mechanism.
    #[must_use]
    pub const fn authentication(&self) -> ProviderAuthentication {
        self.authentication
    }

    /// Changes the authentication mechanism.
    pub const fn set_authentication(&mut self, authentication: ProviderAuthentication) {
        self.authentication = authentication;
    }

    /// Adds a provider capability.
    ///
    /// Returns `true` when the capability was newly inserted.
    pub fn add_capability(&mut self, capability: ProviderCapability) -> bool {
        self.capabilities.insert(capability)
    }

    /// Removes a provider capability.
    ///
    /// Returns `true` when the capability existed.
    pub fn remove_capability(&mut self, capability: ProviderCapability) -> bool {
        self.capabilities.remove(&capability)
    }

    /// Returns whether the provider exposes a capability.
    #[must_use]
    pub fn supports(&self, capability: ProviderCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Returns all declared provider capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<ProviderCapability> {
        &self.capabilities
    }

    /// Returns the number of declared capabilities.
    #[must_use]
    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns the provider priority.
    ///
    /// Lower values represent a stronger selection preference.
    #[must_use]
    pub const fn priority(&self) -> u16 {
        self.priority
    }

    /// Changes the provider priority.
    pub const fn set_priority(&mut self, priority: u16) {
        self.priority = priority;
    }

    /// Returns the request timeout in milliseconds.
    #[must_use]
    pub const fn request_timeout_ms(&self) -> u64 {
        self.request_timeout_ms
    }

    /// Changes the request timeout.
    pub fn set_request_timeout_ms(&mut self, timeout_ms: u64) -> Result<()> {
        if timeout_ms == 0 {
            return Err(invalid_argument(
                "provider request timeout must be greater than zero",
            ));
        }

        self.request_timeout_ms = timeout_ms;
        Ok(())
    }

    /// Returns the maximum retry count.
    #[must_use]
    pub const fn max_retries(&self) -> u8 {
        self.max_retries
    }

    /// Changes the maximum retry count.
    pub const fn set_max_retries(&mut self, max_retries: u8) {
        self.max_retries = max_retries;
    }

    /// Returns the configured request rate limit.
    #[must_use]
    pub const fn rate_limit_per_second(&self) -> Option<u32> {
        self.rate_limit_per_second
    }

    /// Sets the maximum request count per second.
    pub fn set_rate_limit_per_second(&mut self, limit: u32) -> Result<()> {
        if limit == 0 {
            return Err(invalid_argument(
                "provider rate limit must be greater than zero",
            ));
        }

        self.rate_limit_per_second = Some(limit);
        Ok(())
    }

    /// Removes the configured rate limit.
    pub const fn clear_rate_limit(&mut self) {
        self.rate_limit_per_second = None;
    }

    /// Returns whether the provider is trusted.
    #[must_use]
    pub const fn is_trusted(&self) -> bool {
        self.trusted
    }

    /// Marks the provider as trusted.
    pub const fn mark_trusted(&mut self) {
        self.trusted = true;
    }

    /// Removes the trusted-provider designation.
    pub const fn clear_trusted(&mut self) {
        self.trusted = false;
    }

    /// Returns the optional human-readable label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Assigns a human-readable label.
    pub fn set_label(&mut self, label: impl Into<String>) -> Result<()> {
        let label = normalize_required_text(label.into(), "provider label")?;

        self.label = Some(label);
        Ok(())
    }

    /// Removes the provider label.
    pub fn clear_label(&mut self) {
        self.label = None;
    }

    /// Returns whether this provider uses a persistent WebSocket transport.
    #[must_use]
    pub const fn is_websocket(&self) -> bool {
        matches!(
            self.transport,
            ProviderTransport::WebSocket | ProviderTransport::SecureWebSocket
        )
    }

    /// Returns whether this provider supports historical archive queries.
    #[must_use]
    pub fn is_archive_capable(&self) -> bool {
        matches!(self.kind, ProviderKind::ArchiveNode)
            || self.supports(ProviderCapability::ArchiveQueries)
            || self.supports(ProviderCapability::HistoricalState)
    }
}

fn normalize_name(value: String) -> Result<String> {
    let value = normalize_required_text(value, "provider name")?;

    if value.len() > MAX_PROVIDER_NAME_LENGTH {
        return Err(invalid_argument(format!(
            "provider name must not exceed \
             {MAX_PROVIDER_NAME_LENGTH} characters"
        )));
    }

    Ok(value)
}

fn normalize_endpoint(value: String) -> Result<String> {
    let value = normalize_required_text(value, "provider endpoint")?;

    if value.len() > MAX_ENDPOINT_URI_LENGTH {
        return Err(invalid_argument(format!(
            "provider endpoint must not exceed \
             {MAX_ENDPOINT_URI_LENGTH} characters"
        )));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(invalid_argument(
            "provider endpoint must not contain whitespace",
        ));
    }

    let supported = [
        "http://", "https://", "ws://", "wss://", "ipc://", "unix://",
    ];

    if !supported.iter().any(|prefix| value.starts_with(prefix)) {
        return Err(invalid_argument(
            "provider endpoint uses an unsupported URI scheme",
        ));
    }

    Ok(value)
}

fn normalize_required_text(value: String, field: &str) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument(format!("{field} must not be empty")));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(value)
}

fn detect_transport(endpoint: &str) -> ProviderTransport {
    if endpoint.starts_with("https://") {
        ProviderTransport::Https
    } else if endpoint.starts_with("http://") {
        ProviderTransport::Http
    } else if endpoint.starts_with("wss://") {
        ProviderTransport::SecureWebSocket
    } else if endpoint.starts_with("ws://") {
        ProviderTransport::WebSocket
    } else if endpoint.starts_with("ipc://") || endpoint.starts_with("unix://") {
        ProviderTransport::Ipc
    } else {
        ProviderTransport::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider() -> Provider {
        Provider::new(
            NetworkId::new(),
            "Primary RPC",
            "https://rpc.example.invalid",
        )
        .unwrap()
    }

    #[test]
    fn creates_provider() {
        let provider = provider();

        assert_eq!(provider.name(), "Primary RPC");
        assert_eq!(provider.endpoint(), "https://rpc.example.invalid");
        assert_eq!(provider.transport(), ProviderTransport::Https);
        assert_eq!(provider.kind(), ProviderKind::Rpc);
        assert!(provider.is_available());
        assert_eq!(provider.priority(), 100);
        assert_eq!(provider.request_timeout_ms(), 30_000);
        assert_eq!(provider.max_retries(), 3);
    }

    #[test]
    fn identifiers_are_unique() {
        let network_id = NetworkId::new();

        let first = Provider::new(network_id, "RPC", "https://rpc.example.invalid").unwrap();

        let second = Provider::new(network_id, "RPC", "https://rpc.example.invalid").unwrap();

        assert_ne!(first.id(), second.id());
    }

    #[test]
    fn rejects_empty_name() {
        let result = Provider::new(NetworkId::new(), "   ", "https://rpc.example.invalid");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unsupported_endpoint_scheme() {
        let result = Provider::new(NetworkId::new(), "RPC", "ftp://rpc.example.invalid");

        assert!(result.is_err());
    }

    #[test]
    fn detects_websocket_transport() {
        let provider = Provider::new(
            NetworkId::new(),
            "WebSocket",
            "wss://rpc.example.invalid/ws",
        )
        .unwrap();

        assert_eq!(provider.transport(), ProviderTransport::SecureWebSocket);
        assert!(provider.is_websocket());
    }

    #[test]
    fn endpoint_change_redetects_transport() {
        let mut provider = provider();

        provider.set_endpoint("ws://localhost:8546").unwrap();

        assert_eq!(provider.transport(), ProviderTransport::WebSocket);
        assert!(provider.is_websocket());
    }

    #[test]
    fn provider_status_can_be_managed() {
        let mut provider = provider();

        provider.mark_unavailable();

        assert_eq!(provider.status(), ProviderStatus::Unavailable);
        assert!(!provider.is_available());

        provider.mark_rate_limited();

        assert_eq!(provider.status(), ProviderStatus::RateLimited);

        provider.disable();

        assert_eq!(provider.status(), ProviderStatus::Disabled);

        provider.activate();

        assert!(provider.is_available());
    }

    #[test]
    fn capabilities_can_be_managed() {
        let mut provider = provider();

        assert!(provider.add_capability(ProviderCapability::Blocks,));

        assert!(!provider.add_capability(ProviderCapability::Blocks,));

        assert!(provider.supports(ProviderCapability::Blocks,));

        assert_eq!(provider.capability_count(), 1);

        assert!(provider.remove_capability(ProviderCapability::Blocks,));

        assert!(!provider.supports(ProviderCapability::Blocks,));
    }

    #[test]
    fn archive_capability_is_detected() {
        let mut provider = provider();

        assert!(!provider.is_archive_capable());

        provider.add_capability(ProviderCapability::ArchiveQueries);

        assert!(provider.is_archive_capable());

        provider.remove_capability(ProviderCapability::ArchiveQueries);

        provider.set_kind(ProviderKind::ArchiveNode);

        assert!(provider.is_archive_capable());
    }

    #[test]
    fn timeout_must_be_greater_than_zero() {
        let mut provider = provider();

        assert!(provider.set_request_timeout_ms(0).is_err());

        provider.set_request_timeout_ms(5_000).unwrap();

        assert_eq!(provider.request_timeout_ms(), 5_000);
    }

    #[test]
    fn rate_limit_can_be_managed() {
        let mut provider = provider();

        assert!(provider.set_rate_limit_per_second(0).is_err());

        provider.set_rate_limit_per_second(25).unwrap();

        assert_eq!(provider.rate_limit_per_second(), Some(25));

        provider.clear_rate_limit();

        assert_eq!(provider.rate_limit_per_second(), None);
    }

    #[test]
    fn trust_state_can_be_managed() {
        let mut provider = provider();

        assert!(!provider.is_trusted());

        provider.mark_trusted();

        assert!(provider.is_trusted());

        provider.clear_trusted();

        assert!(!provider.is_trusted());
    }

    #[test]
    fn label_can_be_managed() {
        let mut provider = provider();

        provider.set_label("Primary production node").unwrap();

        assert_eq!(provider.label(), Some("Primary production node"));

        provider.clear_label();

        assert_eq!(provider.label(), None);
    }

    #[test]
    fn authentication_can_be_configured() {
        let mut provider = provider();

        provider.set_authentication(ProviderAuthentication::ApiKeyHeader);

        assert_eq!(
            provider.authentication(),
            ProviderAuthentication::ApiKeyHeader
        );
    }

    #[test]
    fn explicit_kind_and_transport_are_preserved() {
        let provider = Provider::with_kind(
            NetworkId::new(),
            "Custom IPC",
            "unix:///tmp/origin-observer.ipc",
            ProviderKind::FullNode,
            ProviderTransport::Ipc,
        )
        .unwrap();

        assert_eq!(provider.kind(), ProviderKind::FullNode);

        assert_eq!(provider.transport(), ProviderTransport::Ipc);
    }
}
