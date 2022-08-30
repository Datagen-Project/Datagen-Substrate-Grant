use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn crate_random_number() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(RandomNodeSelector::create_random_number(Origin::signed(1)));
	});
}
