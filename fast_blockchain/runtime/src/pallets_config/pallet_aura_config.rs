use crate::*;
use pallet_aura::Config;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

pub use frame_support::{

	traits::{
		ConstU32,
		ConstBool,
	}, 
};

parameter_types! {
	pub MaxAuthorities: u32 = 10;
}

impl Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<32>;
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
}

