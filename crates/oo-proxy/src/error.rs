// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/error.rs
// Purpose : Proxy resolution error types.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Proxy resolution error types.

use thiserror::Error;

/// Proxy crate result type.
pub type ProxyResult<T> = Result<T, ProxyError>;

/// Errors produced while detecting or resolving a proxy.
#[derive(Debug, Error)]
pub enum ProxyError {
    /// An address string was not a valid `0x`-prefixed 20-byte address.
    #[error("invalid contract address: {0}")]
    InvalidAddress(String),

    /// Reading the contract's bytecode failed.
    #[error("could not read bytecode: {0}")]
    Bytecode(String),

    /// Decoding bytecode into instructions failed.
    #[error("could not decode bytecode: {0}")]
    BytecodeDecode(#[from] oo_bytecode::BytecodeError),

    /// Reading a storage slot failed.
    #[error("could not read storage: {0}")]
    Storage(#[from] oo_storage::StorageError),

    /// Building or decoding an ABI call failed.
    #[error("could not build ABI call: {0}")]
    Abi(#[from] oo_abi::AbiError),

    /// The underlying RPC call failed.
    #[error("RPC call failed: {0}")]
    Rpc(String),
}
