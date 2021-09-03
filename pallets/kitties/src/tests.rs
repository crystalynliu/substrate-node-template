use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_kitty_works() {
	new_test_ext().execute_with( || {

		assert_ok!(KittiesModule::create_kitty(Origin::signed(123)));
		assert_eq!(KittiesCount::<Test>::get(), Some(1));
		assert!(Kitties::<Test>::get(0).is_some());
		assert_eq!(Owner::<Test>::get(0), Some(123));
	})
}

#[test]
fn create_kitty_failed_when_kitty_count_overflow() {
	new_test_ext().execute_with( || {
		KittiesCount::<Test>::put(u32::MAX);

		assert_noop!(
			KittiesModule::create_kitty(Origin::signed(123)),
			Error::<Test>::KittiesCountOverflow
		);
	})
}

#[test]
fn transfer_kitty_works() {
	new_test_ext().execute_with( || {
		let _ = KittiesModule::create_kitty(Origin::signed(123));

		assert_ok!(KittiesModule::transfer_kitty(Origin::signed(123), 456, 0));
		assert_eq!(Owner::<Test>::get(0), Some(456));
	})
}

#[test]
fn transfer_kitty_failed_when_kitty_not_exist() {
	new_test_ext().execute_with( || {
		assert_noop!(
			KittiesModule::transfer_kitty(Origin::signed(123), 456, 0),
			Error::<Test>::KittiesNotExist
		);
	})
}

#[test]
fn transfer_kitty_failed_when_not_kitty_owner() {
	new_test_ext().execute_with( || {
		let _ = KittiesModule::create_kitty(Origin::signed(123));

		assert_noop!(
			KittiesModule::transfer_kitty(Origin::signed(1), 456, 0),
			Error::<Test>::NotKittyOwner
		);
	})
}
