use crate::mock::*;
use sp_core::sr25519;
use frame_support::{assert_ok, assert_noop};
type Origin = RuntimeOrigin;

// Testing x_work getter function of the storage CheckEveryXBlocks and set_check_every_x_works.

/// Has to check the default value of x_work, should be 0.
#[test]
fn default_x_block() {
	new_test_ext().execute_with(|| {
		let x_work = ComputationalWork::x_work();
		assert_eq!(x_work, 0);
	});
}

/// Has to set x_work to 9, this means that the checking of the computational work is done every 10 blocks.
#[test]
fn set_check_every_x_works() {
	new_test_ext().execute_with(|| {

		// Check if function set_check_every_x_works works.
		assert_ok!(ComputationalWork::set_check_every_x_works(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 10));

		// Should be 9 because is counting from 0.
		assert_eq!(ComputationalWork::x_work(), 9);
	});
}

/// Ha to set x_work to 5, this means that the checking of the computational work is done every 6 blocks.
#[test]
fn set_check_every_x_blocks_2() {
	new_test_ext().execute_with(|| {

		// Check if function set_check_every_x_works works.
		assert_ok!(ComputationalWork::set_check_every_x_works(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 6));

		// Should be 5 because is counting from 0.
		assert_eq!(ComputationalWork::x_work(), 5);
	});
}

// Testing x_work_index function of the storage CheckEveryXBlocksIndex.

/// Has to check the default value of x_work_index, should be 0.
#[test]
fn default_x_block_index() {
	new_test_ext().execute_with(|| {
		let x_work_index = ComputationalWork::x_work_index();
		assert_eq!(x_work_index, 0);
	});
}

/// Has to change the index by 1 when hash_work is called.
#[test]
fn x_block_index_change_by_1() {
	new_test_ext().execute_with(|| {

		assert_ok!(ComputationalWork::set_check_every_x_works(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 2));
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		run_to_block(1);

		let x_work_index = ComputationalWork::x_work_index();

		// Should be 1 because is counting from 0.
		assert_eq!(x_work_index, 1);
	})
}

/// Has to change the index by 2 when hash_work is called twice.
#[test]
fn x_block_index_change_by_1_2_times() {
	new_test_ext().execute_with(|| {

		assert_ok!(ComputationalWork::set_check_every_x_works(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 3));
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));
		run_to_block(1);

		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));
		run_to_block(1);

		let x_work_index = ComputationalWork::x_work_index();

		// Should be 2 because is counting from 0.
		assert_eq!(x_work_index, 2);
	})
}

/// Has to reset the index to 0 when the index is equal to x_work.
#[test]
fn x_block_index_reset() {
	new_test_ext().execute_with(|| {

		assert_ok!(ComputationalWork::set_check_every_x_works(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 2));
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));
		run_to_block(1);

		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));
		run_to_block(1);

		let x_work_index = ComputationalWork::x_work_index();

		// Should be 0 because is counting from 0.
		assert_eq!(x_work_index, 0);
	})
}


/// Has to submit a computational work.
#[test]
fn hash_work() {
	new_test_ext().execute_with(|| {

		run_to_block(10);
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));
	})
}

/// Has to submit a computational work and check set LastComputationalWork (correct case).
#[test]
fn hash_work_2() {
	new_test_ext().execute_with(|| {

		run_to_block(8);
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		let last_computational_work = ComputationalWork::last_computational_work().unwrap();

		let row_hash_to_check = hash_number(8);
		let elaborated_hash_to_check = hash_number(55);


		// Should be the hash of the block height, 8.
		assert_eq!(last_computational_work.0, row_hash_to_check);
		// Should be the hash of the 10th number of the Fibonacci sequence, 55.
		assert_eq!(last_computational_work.1, elaborated_hash_to_check);
		// The current author of the block that submitted the computational work.
		assert_eq!(last_computational_work.2, get_account_id_from_seed::<sr25519::Public>("Alice"));
		// The block height of the block where the computational work was submitted.
		assert_eq!(last_computational_work.3, 8);
	})
}

/// Has to submit a computational work and check set LastComputationalWork (malicious case).
#[test]
fn hash_work_3() {
	new_test_ext().execute_with(|| {

		// On multiply of 5 blocks malicious computational work is submitted.
		// In this case a malicious computational work is submitted.
		run_to_block(10);
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		let last_computational_work = ComputationalWork::last_computational_work().unwrap();

		let row_hash_to_check = hash_number(10);
		let elaborated_hash_to_check = hash_number(0);


		// Should be the hash of the block height, 10.
		assert_eq!(last_computational_work.0, row_hash_to_check);
		// Should be the hash of the 10th number of the Fibonacci sequence, 0.
		assert_eq!(last_computational_work.1, elaborated_hash_to_check);
		// The current author of the block that submitted the computational work.
		assert_eq!(last_computational_work.2, get_account_id_from_seed::<sr25519::Public>("Alice"));
		// The block height of the block where the computational work was submitted.
		assert_eq!(last_computational_work.3, 10);
	})
}

/// Has to emit an event when a computational work is submitted.
#[test]
fn hash_work_4() {
	new_test_ext().execute_with(|| {

		// On multiply of 5 blocks malicious computational work is submitted.
		// In this case a correct computational work is submitted.
		run_to_block(8);
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		let row_hash_to_check = hash_number(8);
		let elaborated_hash_to_check = hash_number(55);

		System::assert_last_event(Event::ComputationalWork(crate::Event::ResultsComputationalWork {
				// The not hashed raw data, it's for testing purposes.
				raw_data: 8,
				// The not hashed elaborated data.
				elaborated_data: 55,
				raw_hash: row_hash_to_check,
				elaborated_hash: elaborated_hash_to_check,
				author: get_account_id_from_seed::<sr25519::Public>("Alice"),
				block_height: 8,
			}));
	})
}

/// Has to emit the last computational work ready to be checked. (is_checked = false)
#[test]
fn get_last_computational_work() {
	new_test_ext().execute_with(|| {

		run_to_block(8);
		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		assert_ok!(ComputationalWork::get_last_computational_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));

		let row_hash_to_check = hash_number(8);
		let elaborated_hash_to_check = hash_number(55);

		System::assert_last_event(Event::ComputationalWork(crate::Event::LastComputationalWork {
				raw_hash: row_hash_to_check,
				elaborated_hash: elaborated_hash_to_check,
				author: get_account_id_from_seed::<sr25519::Public>("Alice"),
				block_height: 8,
				is_checked: false,
		}));
	})
}

/// Has to get error when trying to set x_work_index to 0.
#[test]
fn set_x_block_index_1() {
	new_test_ext().execute_with(|| {

		run_to_block(1);
		assert_noop!(ComputationalWork::set_check_every_x_works(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice")), 0), crate::Error::<Test>::XBlockCannotBeZero);
	})
}