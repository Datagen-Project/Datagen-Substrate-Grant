use crate as pallet_computational_work;
use frame_support::{parameter_types, traits::{ConstU16, ConstU64, GenesisBuild, OnFinalize, OnInitialize}};
use frame_system as system;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{H256, sr25519, crypto::{Public, Pair}};
use sp_runtime::{
	impl_opaque_keys,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Verify, IdentifyAccount}, MultiSignature,
};
use opaque::SessionKeys;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type BlockNumber = u64;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;


pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			// pub grandpa: Grandpa,
		}
	}
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		ComputationalWork: pallet_computational_work,
		Session: pallet_session,
		Aura: pallet_aura,
		Timestamp: pallet_timestamp,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_computational_work::Config for Test {
	type Event = Event;
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
}

pub const MILLISECS_PER_BLOCK: u64 = 1000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

parameter_types! {
	pub MaxAuthorities: u32 = 10;
}

impl pallet_aura::Config for Test {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}


parameter_types! {
	pub Period: u64 = 10 * MINUTES;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Test {
	type Event = Event;
	type ValidatorId = AccountId;
	type ValidatorIdOf = ();
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = ();
	type SessionManager = ();
	type SessionHandler = <opaque::SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_aura::GenesisConfig::<Test> {
		authorities: vec![],
	}
	.assimilate_storage(&mut t).unwrap();
	pallet_session::GenesisConfig::<Test> {
		keys: vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_from_seed::<AuraId>("Bob"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_from_seed::<AuraId>("Charlie"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_from_seed::<AuraId>("Dave"),
			),
		].iter().map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone()))).collect::<Vec<_>>()
	}
	.assimilate_storage(&mut t).unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Helper function to run a block.
#[allow(dead_code)]
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
	 if System::block_number() > 1 {
		ComputationalWork::on_finalize(System::block_number());
	  System::on_finalize(System::block_number());
	 }
	 System::set_block_number(System::block_number() + 1);
	 System::on_initialize(System::block_number());
	 ComputationalWork::on_initialize(System::block_number());
	}
}

pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type Signature = MultiSignature;
type AccountPublic = <Signature as Verify>::Signer;

pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn session_keys(aura: AuraId) -> SessionKeys {
	SessionKeys { aura }
}