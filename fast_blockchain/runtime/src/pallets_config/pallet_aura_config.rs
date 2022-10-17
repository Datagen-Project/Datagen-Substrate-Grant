use crate::*;
use pallet_aura::Config;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

parameter_types! {
	pub MaxAuthorities: u32 = 10;
}

impl Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

