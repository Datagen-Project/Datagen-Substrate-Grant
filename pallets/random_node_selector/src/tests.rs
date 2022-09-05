use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn create_random_number() {
	new_test_ext().execute_with(|| {

		// Dispatch a signed extrinsic.
		assert_ok!(RandomNodeSelector::create_random_number(Origin::signed(1)));
	});
}

#[test]
fn create_random_hash() {
	new_test_ext().execute_with(|| {

		// Dispatch a signed extrinsic.
		assert_ok!(RandomNodeSelector::create_random_hash(Origin::signed(1)));
	});
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
