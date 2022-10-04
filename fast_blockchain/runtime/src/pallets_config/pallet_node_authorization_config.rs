use crate::*;

// Add ensure root with sudo for testing
use frame_system::EnsureRoot;

parameter_types! {
	// Add parameter const for node-authorization pallet
	pub const MaxWellKnownNodes: u32 = 10;
	pub const MaxPeerIdLength: u32 = 128;
}

/// Configure the pallet-node-authorization
impl pallet_node_authorization::Config for Runtime {
	type Event = Event;
	type MaxWellKnownNodes = MaxWellKnownNodes;
	type MaxPeerIdLength = MaxPeerIdLength;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}