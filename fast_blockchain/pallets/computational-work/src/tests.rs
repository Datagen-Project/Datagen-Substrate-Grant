use crate::mock::*;
use sp_core::sr25519;
use frame_support::assert_ok;

// Testing x_block getter function of the storage CheckEveryXBlocks and set_check_every_x_blocks.

/// Has to check the default value of x_block, should be 0.
#[test]
fn check_default_x_block() {
	new_test_ext().execute_with(|| {
		let x_block = ComputationalWork::x_block();
		assert_eq!(x_block, 0);
	});
}

/// Has to set x_block to 9, this means that the checking of the computational work is done every 10 blocks.
#[test]
fn set_check_every_x_blocks() {
	new_test_ext().execute_with(|| {

		// Check if function set_check_every_x_blocks works.
		assert_ok!(ComputationalWork::set_check_every_x_blocks(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 10));

		// Should be 9 because is counting from 0.
		assert_eq!(ComputationalWork::x_block(), 9);
	});
}

/// Ha to set x_block to 5, this means that the checking of the computational work is done every 6 blocks.
#[test]
fn set_check_every_x_blocks_2() {
	new_test_ext().execute_with(|| {

		// Check if function set_check_every_x_blocks works.
		assert_ok!(ComputationalWork::set_check_every_x_blocks(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 6));

		// Should be 5 because is counting from 0.
		assert_eq!(ComputationalWork::x_block(), 5);
	});
}

// Testing x_block_index function of the storage CheckEveryXBlocksIndex.

/// Has to check the default value of x_block_index, should be 0.
#[test]
fn check_default_x_block_index() {
	new_test_ext().execute_with(|| {
		let x_block_index = ComputationalWork::x_block_index();
		assert_eq!(x_block_index, 0);
	});
}


/// Has to change the index by 1 when hash_work is called.
#[test]
fn check_x_block_index_change_by_1() {
	new_test_ext().execute_with(|| {

		run_to_block(10);

		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		let x_block_index = ComputationalWork::x_block_index();

		// Should be 1 because is counting from 0.
		assert_eq!(x_block_index, 1);
	})
}


