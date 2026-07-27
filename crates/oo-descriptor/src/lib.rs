// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-descriptor/src/lib.rs
// Purpose : Build validated blockchain asset and contract descriptors.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Build validated blockchain asset and contract descriptors.

pub mod address;
pub mod asset;
pub mod chain;
pub mod contract;
pub mod interface;
pub mod metadata;
pub mod network;
pub mod standard;
pub mod validation;

pub use address::AddressDescriptor;
pub use asset::AssetDescriptor;
pub use chain::ChainDescriptor;
pub use contract::ContractDescriptor;
pub use interface::{ContractInterface, InterfaceDescriptor};
pub use metadata::MetadataDescriptor;
pub use network::NetworkDescriptor;
pub use standard::StandardDescriptor;
pub use validation::{require_non_empty, DescriptorValidation};

#[cfg(test)]
mod tests {
    use oo_model::address::Address;
    use oo_model::asset::Asset;
    use oo_model::blockchain::{Blockchain, BlockchainKind};
    use oo_model::network::{Network, NetworkKind};

    use super::*;

    #[test]
    fn extracts_address_descriptor() {
        let network = oo_core::NetworkId::new();
        let address = Address::new(network, "0xAABB").unwrap();
        let descriptor = AddressDescriptor::from_address(&address);

        assert_eq!(descriptor.canonical_value(), "0xaabb");
        assert!(descriptor.validate().is_valid());
    }

    #[test]
    fn extracts_native_asset_descriptor() {
        let asset = Asset::native(oo_core::NetworkId::new(), "Ether", "ETH", 18).unwrap();
        let descriptor = AssetDescriptor::from_asset(&asset);

        assert!(descriptor.is_native());
        assert!(descriptor.validate().is_valid());
    }

    #[test]
    fn extracts_chain_and_network_descriptors() {
        let chain = Blockchain::new(BlockchainKind::Ethereum, "Ethereum", "ETH", "Ether");
        let mut network = Network::new(chain.id(), NetworkKind::Mainnet, "Ethereum", "eth");
        network.set_chain_id(1);

        let chain_descriptor = ChainDescriptor::from_blockchain(&chain);
        let network_descriptor = NetworkDescriptor::from_network(&network);

        assert!(chain_descriptor.is_evm_compatible());
        assert_eq!(network_descriptor.chain_id(), Some(1));
    }
}
