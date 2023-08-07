use crate::mock::*;
use frame_support::{assert_ok, assert_noop};
use sp_core::OpaquePeerId;

/// Test the random hash generator.
#[test]
fn create_random_hash() {
	new_test_ext().execute_with(|| {

		// Create random hash.
		assert_ok!(RandomNodeSelector::test_create_random_hash(Origin::signed(1)));
	})
}


/// Test the random number generator.
#[test]
fn create_random_number() {
	new_test_ext().execute_with(|| {

		// Create random number.
		assert_ok!(RandomNodeSelector::test_create_random_number(Origin::signed(1)));

		// Check the event
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::RandomNumber {
			number: 0
		}));

		// Create random number.
		assert_ok!(RandomNodeSelector::test_create_random_number(Origin::signed(1)));
		// Check the event
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::RandomNumber {
			number: 1
		}));
	});
}

#[test]
fn test_total_elements() {
	new_test_ext().execute_with(|| {


		assert_ok!(RandomNodeSelector::test_total_elements(Origin::signed(1)));

		// Check the event
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::TotalItemsInMap {
			total_items: 10
		}));
	});
}

/// Test create_random_number with TestRandomness
#[test]
fn create_random_number_with_test_randomness() {
	new_test_ext().execute_with(|| {

		// Create 3 random numbers.
		// In testing environment, the random number is progressing by 1, starting at 0.
		// random_number = 0
		assert_ok!(RandomNodeSelector::test_create_random_number(Origin::signed(1)));
		// random_number = 1
		assert_ok!(RandomNodeSelector::test_create_random_number(Origin::signed(1)));
		// random_number = 2
		assert_ok!(RandomNodeSelector::test_create_random_number(Origin::signed(1)));

		let random_number = RandomNodeSelector::random_number();

		// Check if the random number is the same as the one in the mock.
		assert_eq!(random_number, 2);
	});
}

/// Test add owner.
#[test]
fn check_add_owner() {
	new_test_ext().execute_with(|| {

		// Has to fail if the the owner already exists.
		assert_noop!(RandomNodeSelector::add_reliable_node(Origin::signed(1), 1, OpaquePeerId(vec![1, 1, 1, 1])), crate::Error::<Test>::AlreadyTakenNode);

		// Add owner.
		// Need to set an free peer id.
		assert_ok!(RandomNodeSelector::add_reliable_node(Origin::signed(1), 1, OpaquePeerId(vec![0, 1, 1, 1])));
	});
}

/// Test remove owner.
#[test]
fn check_remove_owner() {
	new_test_ext().execute_with(|| {

		// Has to fail if the the origin is not the owner.
		assert_noop!(RandomNodeSelector::remove_reliable_node(Origin::signed(2), OpaquePeerId(vec![1, 1, 1, 1])), crate::Error::<Test>::NotOwner);

		// Has to fail if there is no node to remove.
		assert_noop!(RandomNodeSelector::remove_reliable_node(Origin::signed(1), OpaquePeerId(vec![1, 1, 1, 14])), crate::Error::<Test>::NoReliableNodeToCheck);

		// Remove remove the peer id by the owner.
		assert_ok!(RandomNodeSelector::remove_reliable_node(Origin::signed(1), OpaquePeerId(vec![1, 1, 1, 1])));
	})
}

/// Check Genesis Config and get_owner_list function.
#[test]
fn check_initial_owners_list() {
	new_test_ext().execute_with(|| {

		// Dispatch a signed extrinsic.
		assert_ok!(RandomNodeSelector::test_get_owners_list(Origin::signed(1)));
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::ReliableNodeList {
			owners: vec![
				(OpaquePeerId(vec![8, 8, 8, 8]), 8),
				(OpaquePeerId(vec![2, 2, 2, 2]), 2),
				(OpaquePeerId(vec![4, 4, 4, 4]), 4),
				(OpaquePeerId(vec![3, 3, 3, 3]), 3),
				(OpaquePeerId(vec![6, 6, 6, 6]), 6),
				(OpaquePeerId(vec![5, 5, 5, 5]), 5),
				(OpaquePeerId(vec![9, 9, 9, 9]), 9),
				(OpaquePeerId(vec![1, 2, 3, 4]), 10),
				(OpaquePeerId(vec![1, 1, 1, 1]), 1),
				(OpaquePeerId(vec![7, 7, 7, 7]), 7),
			],
		}));
	});
}

/// Test random number generator in a range.
#[test]
fn random_node_range() {
	new_test_ext().execute_with(|| {

		assert_ok!(RandomNodeSelector::test_generate_random_number_range(Origin::signed(1), 2));
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::RandomNumber {
			number: 0
		}));

		assert_ok!(RandomNodeSelector::test_generate_random_number_range(Origin::signed(1), 2));
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::RandomNumber {
			number: 1
		}));

		assert_ok!(RandomNodeSelector::test_generate_random_number_range(Origin::signed(1), 2));
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::RandomNumber {
			number: 0
		}));

		assert_ok!(RandomNodeSelector::test_generate_random_number_range(Origin::signed(1), 2));
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::RandomNumber {
			number: 1
		}));
	})
}

#[test]
fn random_node_to_check() {
	new_test_ext().execute_with(|| {

		// Dispatch a signed extrinsic.
		assert_ok!(RandomNodeSelector::random_node_to_check(Origin::signed(1)));

		// Check the event
		// @dev it's possible that the initial_node_owners map is not in the same order as the one in the mock.
		// if the test fails, it's because of that.
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::ReliableNodeToCheck {
			owner: 8,
			peer_id: OpaquePeerId(vec![8, 8, 8, 8]),
			random_number: 0,
		}));

		// Check the storage ReliableNodeToCheck
		assert_eq!(RandomNodeSelector::reliable_node_to_check(), Some((OpaquePeerId(vec![8, 8, 8, 8]), 8)));
	})
}

#[test]
fn random_checker_node_selector() {
	new_test_ext().execute_with(|| {

		// Has to fail if there in not a node to check.
		assert_noop!(RandomNodeSelector::random_checker_node_selector(Origin::signed(1)), crate::Error::<Test>::NoReliableNodeToCheck);

		// Set a node to check.
		assert_ok!(RandomNodeSelector::random_node_to_check(Origin::signed(1)));

		// Has to select 3 random nodes as a checkers.
		assert_ok!(RandomNodeSelector::random_checker_node_selector(Origin::signed(1)));

		// Check the event
		System::assert_last_event(Event::RandomNodeSelector(crate::Event::Controllers {
			controller_1_account_id: 2,
			controller_1_peer_id: OpaquePeerId(vec![2, 2, 2, 2]),
			random_number_1: 1,
			controller_2_account_id: 4,
			controller_2_peer_id: OpaquePeerId(vec![4, 4, 4, 4]),
			random_number_2: 2,
			controller_3_account_id: 3,
			controller_3_peer_id: OpaquePeerId(vec![3, 3, 3, 3]),
			random_number_3: 3,
		}))
	})
}





