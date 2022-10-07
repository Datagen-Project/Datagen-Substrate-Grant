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
	use sp_runtime::traits::SaturatedConversion;
	use frame_support::traits::FindAuthor;
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
		type FindAuthor: FindAuthor<Self::AccountId>;
	}

	/// The storage of the last computational work.
	///
	/// -  `raw_hashed_data` - The raw hashed data.
	/// -  `elaborated_hashed_data` - The elaborated hashed data.
	/// -  `author` - The author of the block.
	/// -  `block_number` - The block number.
	///
	/// This storage is picked up by the heavy blockchain every x (to define and implement in the M2) blocks for checking the computational work.
	#[pallet::storage]
	#[pallet::getter(fn last_computational_work)]
	pub type LastComputationalWork<T: Config> =
	StorageValue<_, (T::Hash, T::Hash, T::AccountId, u32)>;

	// Events of the pallet.

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a new data is hashed.
		/// The data will be only hashed at production time, the not hashed data is shown for testing purposes.
		/// [raw_data, elaborated_data, raw_hash, elaborated_hash]
		LastComputationalWork {
			raw_data: u32,
			elaborated_data: u32,
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
			author: T::AccountId,
			block_height: u32,
		},

		TestEvent {
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
			author: T::AccountId,
			block_height: u32,
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Hashes the raw data and elaborated and store them in the storage with the author and the block number
		/// for future checks.
		#[pallet::weight(100)]
		pub fn hash_work(
			origin: OriginFor<T>,
			number: u32,
		) -> DispatchResult{
			let _sender = ensure_signed(origin)?;

			// The n number of the fibonacci sequence is calculated.
			let elaborated_math_work = Self::math_work_testing(number);

			// Hashing the raw data and elaborated data.
			let raw_hashed_data = T::Hashing::hash_of(&number);
			let elaborated_hashed_data = T::Hashing::hash_of(&elaborated_math_work);

			// Get the block author.
			let block_digest = <frame_system::Pallet<T>>::digest();
			let digests = block_digest.logs.iter().filter_map(|d| d.as_pre_runtime());
			let author = T::FindAuthor::find_author(digests).unwrap();

			// Get the block height.
			let block_height = <frame_system::Pallet<T>>::block_number();

			// Store data for possible checks.
			<LastComputationalWork<T>>::put((raw_hashed_data, elaborated_hashed_data, author.clone(), block_height.saturated_into::<u32>()));

			// Emit an event.
			Self::deposit_event(Event::LastComputationalWork {
				raw_data: number,
				elaborated_data: elaborated_math_work,
				raw_hash: raw_hashed_data,
				elaborated_hash: elaborated_hashed_data,
				author,
				block_height: block_height.saturated_into::<u32>(),
			});

			Ok(())
		}

		/// Get the last computational work.
		#[pallet::weight(100)]
		pub fn get_last_raw_and_elaborated_data(
			origin: OriginFor<T>,
		) -> DispatchResult{
			let _sender = ensure_signed(origin)?;

			// Get the last computational work from the getter function.
			let last_computational_work = Self::last_computational_work().unwrap();

			// Emit an event.
			Self::deposit_event(Event::TestEvent {
				raw_hash: last_computational_work.0,
				elaborated_hash: last_computational_work.1,
				author: last_computational_work.2,
				block_height: last_computational_work.3,
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


