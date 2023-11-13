use crate::pallet::Error;
use codec::{Decode, Encode};
use ddc_primitives::{ClusterId, ClusterPricingParams};
use frame_support::{pallet_prelude::*, parameter_types};
use scale_info::TypeInfo;
use sp_runtime::Perbill;

parameter_types! {
	pub MaxClusterParamsLen: u16 = 2048;
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct Cluster<AccountId> {
	pub cluster_id: ClusterId,
	pub manager_id: AccountId,
	pub reserve_id: AccountId,
	pub props: ClusterProps<AccountId>,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct ClusterProps<AccountId> {
	pub node_provider_auth_contract: AccountId,
}

// ClusterParams includes Governance non-sensetive parameters only
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct ClusterParams<AccountId> {
	pub node_provider_auth_contract: AccountId,
}

// ClusterGovParams includes Governance sensetive parameters
#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
#[scale_info(skip_type_params(Balance, BlockNumber, T))]
pub struct ClusterGovParams<Balance, BlockNumber> {
	pub treasury_share: Perbill,
	pub validators_share: Perbill,
	pub cluster_reserve_share: Perbill,
	pub cdn_bond_size: Balance,
	pub cdn_chill_delay: BlockNumber,
	pub cdn_unbonding_delay: BlockNumber,
	pub storage_bond_size: Balance,
	pub storage_chill_delay: BlockNumber,
	pub storage_unbonding_delay: BlockNumber,
	pub pricing: ClusterPricingParams,
}

impl<AccountId> Cluster<AccountId> {
	pub fn new(
		cluster_id: ClusterId,
		manager_id: AccountId,
		reserve_id: AccountId,
		cluster_params: ClusterParams<AccountId>,
	) -> Result<Cluster<AccountId>, ClusterError> {
		Ok(Cluster {
			cluster_id,
			manager_id,
			reserve_id,
			props: ClusterProps {
				node_provider_auth_contract: cluster_params.node_provider_auth_contract,
			},
		})
	}

	pub fn set_params(
		&mut self,
		cluster_params: ClusterParams<AccountId>,
	) -> Result<(), ClusterError> {
		self.props = ClusterProps {
			node_provider_auth_contract: cluster_params.node_provider_auth_contract,
		};
		Ok(())
	}
}

pub enum ClusterError {
	ClusterParamsExceedsLimit,
}

impl<T> From<ClusterError> for Error<T> {
	fn from(error: ClusterError) -> Self {
		match error {
			ClusterError::ClusterParamsExceedsLimit => Error::<T>::ClusterParamsExceedsLimit,
		}
	}
}
