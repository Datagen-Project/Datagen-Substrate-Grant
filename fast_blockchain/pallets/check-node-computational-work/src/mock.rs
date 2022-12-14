use crate as pallet_check_node_computational_work;
use frame_support::{traits::{ConstU16, ConstU64, OnFinalize, OnInitialize}};
use frame_system as system;
use sp_core::{H256, sr25519, crypto::{Public, Pair}};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Verify, IdentifyAccount, Hash}, MultiSignature,
};
use sp_runtime::ConsensusEngineId;
use frame_support::traits::FindAuthor;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type BlockNumber = u64;

use sp_runtime::SaturatedConversion;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		ComputationalWork: pallet_computational_work,
		CheckNodeComputationalWork: pallet_check_node_computational_work,
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
	type FindAuthor = AuthorGiven;
}

impl pallet_check_node_computational_work::Config for Test {
	type Event = Event;
	type FindAuthor = AuthorGiven;
}

pub struct AuthorGiven;
impl FindAuthor<AccountId> for AuthorGiven {
    fn find_author<'a, I>(_digests: I) -> Option<AccountId>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        Some(set_author())
    }
}

// Simulate the author of the blocks.
pub fn set_author() -> AccountId {
	let n = System::block_number().saturated_into::<u32>();
	match n % 4 {
		0 => get_account_id_from_seed::<sr25519::Public>("Alice"),
		1 => get_account_id_from_seed::<sr25519::Public>("Bob"),
		2 => get_account_id_from_seed::<sr25519::Public>("Charlie"),
		_ => get_account_id_from_seed::<sr25519::Public>("Dave"),
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
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
		CheckNodeComputationalWork::on_finalize(System::block_number());
	  System::on_finalize(System::block_number());
	 }
	 System::set_block_number(System::block_number() + 1);
	 System::on_initialize(System::block_number());
	 ComputationalWork::on_initialize(System::block_number());
	 CheckNodeComputationalWork::on_initialize(System::block_number());
	}
}

/// Helper function that hash a number to a H256.
pub fn hash_number(n: u32) -> H256 {
	BlakeTwo256::hash_of(&n)
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