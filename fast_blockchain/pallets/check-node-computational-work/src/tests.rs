use crate::mock::*;
use frame_support::assert_ok;
use sp_core::sr25519;

/// Has to check the correct event when hash_work is called, in this case the final event should have true as is_passed value.
#[test]
fn correct_hook_event_when_hash_work_is_called() {
    new_test_ext().execute_with(|| {
        run_to_block(8);

        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Alice")
        )));
        let row_hash_to_check = hash_number(8);
        let elaborated_hash_to_check = hash_number(55);

        System::assert_last_event(Event::ComputationalWork(
            pallet_computational_work::Event::ResultsComputationalWork {
                // The not hashed raw data, it's for testing purposes.
                raw_data: 8,
                // The not hashed elaborated data.
                elaborated_data: 55,
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                author: get_account_id_from_seed::<sr25519::Public>("Alice"),
                block_height: 8,
            },
        ));

        run_to_block(9);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::CheckResult {
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
                block_height: 9,
                current_author: get_account_id_from_seed::<sr25519::Public>("Bob"),
                is_passed: true,
            },
        ));

        run_to_block(10);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::CheckResult {
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
                block_height: 10,
                current_author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                is_passed: false,
            },
        ));

        run_to_block(11);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::CheckResult {
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
                block_height: 11,
                current_author: get_account_id_from_seed::<sr25519::Public>("Dave"),
                is_passed: true,
            },
        ));

        run_to_block(12);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::FinalResult {
                checked_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
                controller1: get_account_id_from_seed::<sr25519::Public>("Bob"),
                result1: true,
                controller2: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                result2: false,
                controller3: get_account_id_from_seed::<sr25519::Public>("Dave"),
                result3: true,
                is_passed: true,
            },
        ));
    });
}

/// Has to check the correct event when hash_work is called, in this case the final event should have false as is_passed value.
#[test]
fn correct_hook_event_when_hash_work_is_called_2() {
    new_test_ext().execute_with(|| {
        run_to_block(10);

        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Alice")
        )));
        let row_hash_to_check = hash_number(10);
        let elaborated_hash_to_check = hash_number(0);

        System::assert_last_event(Event::ComputationalWork(
            pallet_computational_work::Event::ResultsComputationalWork {
                // The not hashed raw data, it's for testing purposes.
                raw_data: 10,
                // The not hashed elaborated data.
                elaborated_data: 0,
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                block_height: 10,
            },
        ));

        run_to_block(11);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::CheckResult {
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                checked_author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                block_height: 11,
                current_author: get_account_id_from_seed::<sr25519::Public>("Dave"),
                is_passed: false,
            },
        ));

        run_to_block(12);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::CheckResult {
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                checked_author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                block_height: 12,
                current_author: get_account_id_from_seed::<sr25519::Public>("Alice"),
                is_passed: false,
            },
        ));

        run_to_block(13);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::CheckResult {
                raw_hash: row_hash_to_check,
                elaborated_hash: elaborated_hash_to_check,
                checked_author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                block_height: 13,
                current_author: get_account_id_from_seed::<sr25519::Public>("Bob"),
                is_passed: false,
            },
        ));

        run_to_block(14);

        System::assert_last_event(Event::CheckNodeComputationalWork(
            crate::Event::FinalResult {
                checked_author: get_account_id_from_seed::<sr25519::Public>("Charlie"),
                controller1: get_account_id_from_seed::<sr25519::Public>("Dave"),
                result1: false,
                controller2: get_account_id_from_seed::<sr25519::Public>("Alice"),
                result2: false,
                controller3: get_account_id_from_seed::<sr25519::Public>("Bob"),
                result3: false,
                is_passed: false,
            },
        ));
    });
}

/// Has to reset the storage process storage when the final result is emitted.
/// Has to set set_last_computational_work_is_checked to true.\
#[test]
fn correct_reset_when_final_result_is_emitted() {
    new_test_ext().execute_with(|| {
        run_to_block(8);

        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Alice")
        )));
        let last_computational_work_is_checked =
            ComputationalWork::last_computational_work_is_checked();
        assert!(!last_computational_work_is_checked);

        run_to_block(9);
        let first_author_has_checked = CheckNodeComputationalWork::first_author_has_checked();
        assert!(first_author_has_checked);

        run_to_block(10);
        let second_author_has_checked = CheckNodeComputationalWork::second_author_has_checked();
        assert!(second_author_has_checked);

        run_to_block(11);
        let third_author_has_checked = CheckNodeComputationalWork::third_author_has_checked();
        assert!(third_author_has_checked);

        run_to_block(14);
        let first_author_has_checked = CheckNodeComputationalWork::first_author_has_checked();
        let second_author_has_checked = CheckNodeComputationalWork::second_author_has_checked();
        let third_author_has_checked = CheckNodeComputationalWork::third_author_has_checked();
        assert!(!first_author_has_checked);
        assert!(!second_author_has_checked);
        assert!(!third_author_has_checked);

        let first_author = CheckNodeComputationalWork::first_author();
        let second_author = CheckNodeComputationalWork::second_author();
        let third_author = CheckNodeComputationalWork::third_author();
        assert_eq!(first_author, None);
        assert_eq!(second_author, None);
        assert_eq!(third_author, None);

        let last_computational_work_is_checked =
            ComputationalWork::last_computational_work_is_checked();
        assert!(last_computational_work_is_checked);
    })
}

/// Has to not change the last computational work if the previous one is not checked.
#[test]
fn not_change_last_computational_work_if_not_checked() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Alice")
        )));

        let last_computational_work = ComputationalWork::last_computational_work();

        run_to_block(11);
        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Bob")
        )));

        assert_eq!(
            last_computational_work,
            ComputationalWork::last_computational_work()
        );
    })
}

///Has to change the last computational work if the previous one is checked.
#[test]
fn change_last_computational_work_if_is_checked() {
    new_test_ext().execute_with(|| {
        run_to_block(8);
        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Alice")
        )));

        let last_computational_work = ComputationalWork::last_computational_work().unwrap();
        assert_eq!(
            last_computational_work.2,
            get_account_id_from_seed::<sr25519::Public>("Alice")
        );

        run_to_block(14);

        assert_ok!(ComputationalWork::hash_work(Origin::signed(
            get_account_id_from_seed::<sr25519::Public>("Alice")
        )));
        let last_computational_work = ComputationalWork::last_computational_work().unwrap();
        assert_eq!(
            last_computational_work.2,
            get_account_id_from_seed::<sr25519::Public>("Charlie")
        );
    })
}
