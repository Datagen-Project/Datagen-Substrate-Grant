use crate as pallet_random_node_selector;
use frame_support::traits::{ConstU16, ConstU64, OnFinalize, OnInitialize};
use frame_support_test::TestRandomness;
use sp_core::OpaquePeerId;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum TestRuntime
    {
        System: frame_system,
        RandomNodeSelector: pallet_random_node_selector,
        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
    }
);

impl frame_system::Config for TestRuntime {
    type AccountId = u64;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Block = Block;
    type DbWeight = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
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
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
}

impl pallet_random_node_selector::Config for TestRuntime {
    type RuntimeEvent = RuntimeEvent;
    type Randomness = TestRandomness<Self>;
}

impl pallet_insecure_randomness_collective_flip::Config for TestRuntime {}

/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<TestRuntime>::default()
        .build_storage()
        .unwrap();
    pallet_random_node_selector::GenesisConfig::<TestRuntime> {
        initial_node_owners: vec![
            (OpaquePeerId(vec![1, 1, 1, 1]), 1),
            (OpaquePeerId(vec![2, 2, 2, 2]), 2),
            (OpaquePeerId(vec![3, 3, 3, 3]), 3),
            (OpaquePeerId(vec![4, 4, 4, 4]), 4),
            (OpaquePeerId(vec![5, 5, 5, 5]), 5),
            (OpaquePeerId(vec![6, 6, 6, 6]), 6),
            (OpaquePeerId(vec![7, 7, 7, 7]), 7),
            (OpaquePeerId(vec![8, 8, 8, 8]), 8),
            (OpaquePeerId(vec![9, 9, 9, 9]), 9),
            (OpaquePeerId(vec![1, 2, 3, 4]), 10),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

/// Helper function to run a block.
#[allow(dead_code)]
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            RandomNodeSelector::on_finalize(System::block_number());
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        RandomNodeSelector::on_initialize(System::block_number());
    }
}
