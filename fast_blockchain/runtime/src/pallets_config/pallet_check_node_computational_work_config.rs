use crate::*;

impl pallet_check_node_computational_work::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
}