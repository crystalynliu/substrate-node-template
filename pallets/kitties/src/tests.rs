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
