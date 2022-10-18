use crate::mock::*;
use sp_core::sr25519;
use frame_support::{assert_ok, assert_noop};

/// Has to check the default value of x_block, should be 0.
#[test]
fn default_x_block() {
	new_test_ext().execute_with(|| {
		let x_block = ComputationalWork::x_block();
		assert_eq!(x_block, 0);
	});
}