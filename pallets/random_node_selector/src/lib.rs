#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::traits::Randomness;
use scale_info::prelude::vec::Vec;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::Randomness;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]

	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New random hash.
		UniqueCreated { hash: T::Hash },

		/// New random number.
		UniqueNumber { number: u32 },
	}

	#[pallet::storage]
	pub(super) type RandomNumber<T: Config> =
		StorageValue<_, T::Hash>;

	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<
		_,
		u64,
		ValueQuery
		>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Create a new random hash.
		#[pallet::weight(100)]
		pub fn crate_random_hash(
			origin: OriginFor<T>
		) -> DispatchResult {
			// Account calling this dispatchable.
			let _sender = ensure_signed(origin)?;
				// Random value.
				let nonce = Self::get_and_increment_nonce();
				let (random_value, _) = T::Randomness::random(&nonce);
			// Write the random value to storage.
			<RandomNumber<T>>::put(random_value);
			Self::deposit_event(Event::UniqueCreated{hash: random_value});

			Ok(())
		}

		/// Crate a new random number.
		#[pallet::weight(100)]
		pub fn create_random_number(
			origin: OriginFor<T>
		) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let random_number = Self::generate_random_number();

			Self::deposit_event(Event::UniqueNumber { number: random_number });

				Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn get_and_increment_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		Nonce::<T>::put(nonce.wrapping_add(1));
		nonce.encode()
	}

	/// Generate a random number
	fn generate_random_number() -> u32 {
		let (random_seed, _) = T::Randomness::random(&Self::get_and_increment_nonce());
		let random_number = <u32>::decode(&mut random_seed.as_ref())
			.expect("secure hashes should always be bigger than u32; qed");
		random_number
	}
}

