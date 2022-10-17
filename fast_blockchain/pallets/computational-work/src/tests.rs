use crate::mock::*;
use sp_core::sr25519;
use frame_support::assert_ok;

#[test]
fn set_check_every_x_blocks() {
	new_test_ext().execute_with(|| {
		assert_ok!(ComputationalWork::set_check_every_x_blocks(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 10));
	});
}