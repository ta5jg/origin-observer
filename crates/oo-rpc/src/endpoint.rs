// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/endpoint.rs
// Purpose : RPC endpoint model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! RPC endpoint model.

use url::Url;

use crate::error::{RpcError, RpcResult};

/// Supported RPC endpoint transport kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RpcEndpointKind {
    /// HTTP endpoint.
    Http,
    /// HTTPS endpoint.
    Https,
}

/// Validated JSON-RPC endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcEndpoint {
    url: Url,
    kind: RpcEndpointKind,
}

impl RpcEndpoint {
    /// Parses and validates an RPC endpoint URL.
    pub fn parse(value: impl AsRef<str>) -> RpcResult<Self> {
        let url = Url::parse(value.as_ref())
            .map_err(|error| RpcError::InvalidEndpoint(error.to_string()))?;

        let kind = match url.scheme() {
            "http" => RpcEndpointKind::Http,
            "https" => RpcEndpointKind::Https,
            scheme => {
                return Err(RpcError::InvalidEndpoint(format!(
                    "unsupported scheme {scheme}"
                )));
            }
        };

        Ok(Self { url, kind })
    }

    /// Returns the parsed URL.
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the endpoint kind.
    #[must_use]
    pub const fn kind(&self) -> RpcEndpointKind {
        self.kind
    }
}
