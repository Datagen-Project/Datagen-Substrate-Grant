use crate::mock::*;
use sp_core::sr25519;
use frame_support::{assert_ok, assert_noop};

/// Has to check the correct event when hash_work is called.
#[test]
fn correct_event_when_hash_work_is_called() {
	new_test_ext().execute_with(|| {
		run_to_block(8);

		assert_ok!(ComputationalWork::hash_work(Origin::signed(get_account_id_from_seed::<sr25519::Public>("Alice"))));
		let row_hash_to_check = hash_number(8);
		let elaborated_hash_to_check = hash_number(55);

		System::assert_last_event(Event::ComputationalWork(pallet_computational_work::Event::ResultsComputationalWork {
			// The not hashed raw data, it's for testing purposes.
			raw_data: 8,
			// The not hashed elaborated data.
			elaborated_data: 55,
			raw_hash: row_hash_to_check,
			elaborated_hash: elaborated_hash_to_check,
			author: get_account_id_from_seed::<sr25519::Public>("Alice"),
			block_height: 8,
		}));

		run_to_block(9);

		System::assert_last_event(Event::CheckNodeComputationalWork(crate::Event::CheckResult {
			raw_hash: row_hash_to_check,
			elaborated_hash: elaborated_hash_to_check,
			checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
			block_height: 9,
			current_author: get_account_id_from_seed::<sr25519::Public>("Bob"),
			is_passed: true,
		}));

		run_to_block(10);

		System::assert_last_event(Event::CheckNodeComputationalWork(crate::Event::CheckResult {
			raw_hash: row_hash_to_check,
			elaborated_hash: elaborated_hash_to_check,
			checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
			block_height: 10,
			current_author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
			is_passed: false,
		}));

		run_to_block(11);

		System::assert_last_event(Event::CheckNodeComputationalWork(crate::Event::CheckResult {
			raw_hash: row_hash_to_check,
			elaborated_hash: elaborated_hash_to_check,
			checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
			block_height: 11,
			current_author: get_account_id_from_seed::<sr25519::Public>("Dave"),
			is_passed: true,
		}));

		run_to_block(12);

		System::assert_last_event(Event::CheckNodeComputationalWork(crate::Event::FinalResult {
			checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
			controller1: get_account_id_from_seed::<sr25519::Public>("Bob"),
			result1: true,
			controller2: get_account_id_from_seed::<sr25519::Public>("Charlie"),
			result2: false,
			controller3: get_account_id_from_seed::<sr25519::Public>("Dave"),
			result3: true,
			is_passed: true,
		}));

	})
}