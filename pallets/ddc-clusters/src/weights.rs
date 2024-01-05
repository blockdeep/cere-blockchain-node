//! Autogenerated weights for pallet_ddc_clusters
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bench`, CPU: `DO-Premium-AMD`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/cere
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --pallet=pallet-ddc-clusters
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/frame-weight-template.hbs
// --output=pallets/ddc-clusters/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ddc_clusters.
pub trait WeightInfo {
	fn create_cluster() -> Weight;
	fn add_node() -> Weight;
	fn remove_node() -> Weight;
	fn set_cluster_params() -> Weight;
	fn set_cluster_gov_params() -> Weight;
}

/// Weights for pallet_ddc_clusters using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DdcClusters Clusters (r:1 w:1)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn create_cluster() -> Weight {
		Weight::from_ref_time(243_795_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcStaking Nodes (r:1 w:0)
	// Storage: DdcStaking Storages (r:1 w:0)
	// Storage: DdcStaking Bonded (r:1 w:0)
	// Storage: DdcStaking Ledger (r:1 w:0)
	// Storage: DdcNodes StorageNodes (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Contracts ContractInfoOf (r:1 w:1)
	// Storage: Contracts CodeStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System EventTopics (r:2 w:2)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	// Storage: unknown [0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3] (r:1 w:0)
	// Storage: unknown [0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957] (r:1 w:0)
	fn add_node() -> Weight {
		Weight::from_ref_time(37_168_211_000_u64)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcNodes StorageNodes (r:1 w:1)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	fn remove_node() -> Weight {
		Weight::from_ref_time(290_065_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:1)
	fn set_cluster_params() -> Weight {
		Weight::from_ref_time(161_977_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn set_cluster_gov_params() -> Weight {
		Weight::from_ref_time(164_837_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: DdcClusters Clusters (r:1 w:1)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn create_cluster() -> Weight {
		Weight::from_ref_time(243_795_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcStaking Nodes (r:1 w:0)
	// Storage: DdcStaking Storages (r:1 w:0)
	// Storage: DdcStaking Bonded (r:1 w:0)
	// Storage: DdcStaking Ledger (r:1 w:0)
	// Storage: DdcNodes StorageNodes (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Contracts ContractInfoOf (r:1 w:1)
	// Storage: Contracts CodeStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System EventTopics (r:2 w:2)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	// Storage: unknown [0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3] (r:1 w:0)
	// Storage: unknown [0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957] (r:1 w:0)
	fn add_node() -> Weight {
		Weight::from_ref_time(37_168_211_000_u64)
			.saturating_add(RocksDbWeight::get().reads(14_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcNodes StorageNodes (r:1 w:1)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	fn remove_node() -> Weight {
		Weight::from_ref_time(290_065_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:1)
	fn set_cluster_params() -> Weight {
		Weight::from_ref_time(161_977_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn set_cluster_gov_params() -> Weight {
		Weight::from_ref_time(164_837_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}