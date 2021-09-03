#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*, sp_io,
		traits::Randomness
	};
	use frame_system::pallet_prelude::*;
	use codec::{Encode, Decode};
	use sp_io::hashing::blake2_128;

	#[derive(Encode, Decode)]
	pub struct Kitty(pub [u8;16]);

	type KittyIndex = u32;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotKittyOwner,
		KittiesNotExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_kitty(
			origin: OriginFor<T>
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => { 0 }
			};

			let dna = Self::random_value(&who);

			Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittiesCount::<T>::put(kitty_id + 1);

			Self::deposit_event(Event::KittyCreated(who, kitty_id));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			new_owner: T::AccountId,
			kitty_id: KittyIndex
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(Kitties::<T>::get(kitty_id).is_some(), Error::<T>::KittiesNotExist);

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotKittyOwner);

			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}
	}
}
