use crate as pallet_computational_work;
use frame_support::traits::{ConstU16, ConstU64, OnFinalize, OnInitialize, FindAuthor};
use frame_system as system;
use sp_core::{H256, sr25519, crypto::{Public, Pair}};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, Verify, IdentifyAccount, Hash}, MultiSignature,
	BuildStorage, 
	ConsensusEngineId
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type BlockNumber = u64;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test 
	{
		System: frame_system,
		ComputationalWork: pallet_computational_work,
	}
);

impl system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = u64;
    type RuntimeCall = RuntimeCall;
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block ;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
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
	type RuntimeEvent = RuntimeEvent;
	type ComputationalWorkFindAuthor = AuthorGiven;
}

pub struct AuthorGiven;
impl FindAuthor<AccountId> for AuthorGiven {
    fn find_author<'a, I>(_digests: I) -> Option<AccountId>
    where
        I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
    {
        Some(get_account_id_from_seed::<sr25519::Public>("Alice"))
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
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