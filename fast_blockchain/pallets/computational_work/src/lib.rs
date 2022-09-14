#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Hash;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	/// A tuple with raw hashed data and elaborated hashed data.
	#[pallet::storage]
	#[pallet::getter(fn raw_and_elaborated_data)]
	pub type RawAndElaboratedData<T: Config> =
	StorageValue<_, (T::Hash, T::Hash)>;

	// Events of the pallet.

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a new data is hashed.
		/// [raw_data, elaborated_data, raw_hash, elaborated_hash]
		RawAndElaboratedData {
			raw_data: u32,
			elaborated_data: u32,
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Hashes the raw data and elaborated data.
		#[pallet::weight(100)]
		pub fn hash_work(
			origin: OriginFor<T>,
			number: u32,
		) -> DispatchResult{
			let _sender = ensure_signed(origin)?;

			let elaborated_math_work = Self::math_work_testing(number);

			let raw_hashed_data = T::Hashing::hash_of(&number);
			let elaborated_hashed_data = T::Hashing::hash_of(&elaborated_math_work);

			<RawAndElaboratedData<T>>::put((raw_hashed_data, elaborated_hashed_data));

			Self::deposit_event(Event::RawAndElaboratedData {
				raw_data: number,
				elaborated_data: elaborated_math_work,
				raw_hash: raw_hashed_data,
				elaborated_hash: elaborated_hashed_data,
			});

			Ok(())
		}
	}
}

impl <T: Config> Pallet<T> {

	/// A function that does some math work, fibonacci sequence, for testing purposes.
	fn math_work_testing(n: u32) -> u32 {
		match n {
			0 => 0,
			1 => 1,
			_ => Self::math_work_testing(n - 1) + Self::math_work_testing(n - 2),
		}
	}
}


