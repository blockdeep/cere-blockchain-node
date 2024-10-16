//! Autogenerated weights for pallet_ddc_clusters
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-07-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bench`, CPU: `AMD EPYC-Milan Processor`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/cere
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_ddc_clusters
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/frame-weight-template.hbs
// --output=pallets/ddc-clusters/weights.rs

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
	fn validate_node() -> Weight;
}

/// Weights for pallet_ddc_clusters using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: `DdcClusters::Clusters` (r:1 w:1)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:0 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersGovParams` (r:0 w:1)
	// Proof: `DdcClusters::ClustersGovParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_cluster() -> Weight {
		Weight::from_parts(35_617_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:0)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Nodes` (r:1 w:0)
	// Proof: `DdcStaking::Nodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Storages` (r:1 w:0)
	// Proof: `DdcStaking::Storages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Bonded` (r:1 w:0)
	// Proof: `DdcStaking::Bonded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Ledger` (r:1 w:0)
	// Proof: `DdcStaking::Ledger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcNodes::StorageNodes` (r:1 w:1)
	// Proof: `DdcNodes::StorageNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `MaxEncodedLen`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `MaxEncodedLen`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	// Storage: `System::EventTopics` (r:2 w:2)
	// Proof: `System::EventTopics` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodes` (r:0 w:1)
	// Proof: `DdcClusters::ClustersNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: UNKNOWN KEY `0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3` (r:1 w:0)
	// Storage: UNKNOWN KEY `0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957` (r:1 w:0)
	// Proof: UNKNOWN KEY `0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957` (r:1 w:0)
	fn add_node() -> Weight {
		Weight::from_parts(655_639_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(17_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:0)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcNodes::StorageNodes` (r:1 w:1)
	// Proof: `DdcNodes::StorageNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodes` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_node() -> Weight {
		Weight::from_parts(55_664_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:1)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_cluster_params() -> Weight {
		Weight::from_parts(23_734_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:0)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodes` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn validate_node() -> Weight {
		Weight::from_parts(42_951_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: `DdcClusters::Clusters` (r:1 w:1)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:0 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersGovParams` (r:0 w:1)
	// Proof: `DdcClusters::ClustersGovParams` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_cluster() -> Weight {
		Weight::from_parts(35_617_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:0)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Nodes` (r:1 w:0)
	// Proof: `DdcStaking::Nodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Storages` (r:1 w:0)
	// Proof: `DdcStaking::Storages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Bonded` (r:1 w:0)
	// Proof: `DdcStaking::Bonded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcStaking::Ledger` (r:1 w:0)
	// Proof: `DdcStaking::Ledger` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcNodes::StorageNodes` (r:1 w:1)
	// Proof: `DdcNodes::StorageNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Contracts::MigrationInProgress` (r:1 w:0)
	// Proof: `Contracts::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Contracts::ContractInfoOf` (r:1 w:1)
	// Proof: `Contracts::ContractInfoOf` (`max_values`: None, `max_size`: Some(1795), added: 4270, mode: `MaxEncodedLen`)
	// Storage: `Contracts::CodeInfoOf` (r:1 w:0)
	// Proof: `Contracts::CodeInfoOf` (`max_values`: None, `max_size`: Some(93), added: 2568, mode: `MaxEncodedLen`)
	// Storage: `Contracts::PristineCode` (r:1 w:0)
	// Proof: `Contracts::PristineCode` (`max_values`: None, `max_size`: Some(125988), added: 128463, mode: `MaxEncodedLen`)
	// Storage: `Timestamp::Now` (r:1 w:0)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	// Storage: `System::EventTopics` (r:2 w:2)
	// Proof: `System::EventTopics` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodes` (r:0 w:1)
	// Proof: `DdcClusters::ClustersNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: UNKNOWN KEY `0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3` (r:1 w:0)
	// Storage: UNKNOWN KEY `0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957` (r:1 w:0)
	// Proof: UNKNOWN KEY `0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957` (r:1 w:0)
	fn add_node() -> Weight {
		Weight::from_parts(655_639_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(17_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:0)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcNodes::StorageNodes` (r:1 w:1)
	// Proof: `DdcNodes::StorageNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodes` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_node() -> Weight {
		Weight::from_parts(55_664_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:1)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_cluster_params() -> Weight {
		Weight::from_parts(23_734_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: `DdcClusters::Clusters` (r:1 w:0)
	// Proof: `DdcClusters::Clusters` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodesStats` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodesStats` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `DdcClusters::ClustersNodes` (r:1 w:1)
	// Proof: `DdcClusters::ClustersNodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn validate_node() -> Weight {
		Weight::from_parts(42_951_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
