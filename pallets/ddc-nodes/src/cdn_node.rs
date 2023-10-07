use crate::{
	node::{Node, NodeError, NodeParams, NodePropsRef, NodePubKeyRef, NodeTrait, NodeType},
	ClusterId,
};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{AccountId32, RuntimeDebug};
use sp_std::prelude::*;

pub type CDNNodePubKey = AccountId32;

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct CDNNode<ProviderId> {
	pub pub_key: CDNNodePubKey,
	pub provider_id: ProviderId,
	pub cluster_id: Option<ClusterId>,
	pub props: CDNNodeProps,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct CDNNodeProps {
	pub url: Vec<u8>,
	pub location: Vec<u8>,
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq)]
pub struct CDNNodeParams {
	pub pub_key: CDNNodePubKey,
	pub url: Vec<u8>,
	pub location: Vec<u8>,
}

impl<ProviderId> NodeTrait<ProviderId> for CDNNode<ProviderId> {
	fn get_pub_key<'a>(&'a self) -> NodePubKeyRef<'a> {
		NodePubKeyRef::CDNPubKeyRef(&self.pub_key)
	}
	fn get_provider_id(&self) -> &ProviderId {
		&self.provider_id
	}
	fn get_props<'a>(&'a self) -> NodePropsRef<'a> {
		NodePropsRef::CDNPropsRef(&self.props)
	}
	fn get_cluster_id(&self) -> &Option<ClusterId> {
		&self.cluster_id
	}
	fn set_cluster_id(&mut self, cluster_id: ClusterId) {
		self.cluster_id = Some(cluster_id);
	}
	fn get_type(&self) -> NodeType {
		NodeType::CDN
	}
	fn from_params(
		provider_id: ProviderId,
		params: NodeParams,
	) -> Result<Node<ProviderId>, NodeError> {
		match params {
			NodeParams::CDNParams(params) => Ok(Node::CDN(CDNNode::<ProviderId> {
				provider_id,
				pub_key: params.pub_key,
				cluster_id: None,
				props: CDNNodeProps { url: params.url, location: params.location },
			})),
			_ => Err(NodeError::InvalidCDNNodeParams),
		}
	}
}
