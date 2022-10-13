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

	#[pallet::type_value]
	pub fn DefaultCheckAuthor<T: Config>() -> bool {
		true
	}

	/// The storage of the last computational work.
	///
	/// -  `raw_hashed_data` - The raw hashed data.
	/// -  `elaborated_hashed_data` - The elaborated hashed data.
	/// -  `author` - The author of the block.
	/// -  `block_number` - The block number.
	/// -  `is_checked` - Is true if the last computational work has been checked.
	///
	/// This storage is picked up by the heavy blockchain every x (to define and implement in the M2) blocks for checking the computational work.
	#[pallet::storage]
	#[pallet::getter(fn last_computational_work)]
	pub type LastComputationalWork<T: Config> =
	StorageValue<_, (T::Hash, T::Hash, T::AccountId, u32)>;

	#[pallet::storage]
	#[pallet::getter(fn last_computational_work_is_checked)]
	pub type LastComputationalWorkIsChecked<T: Config> = StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;


	#[pallet::storage]
	#[pallet::getter(fn raw_data)]
	pub type RawData<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Events of the pallet.

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a new data is hashed.
		/// The data will be only hashed at production time, the not hashed data is shown for testing purposes.
		/// [raw_data, elaborated_data, raw_hash, elaborated_hash]
		ResultsComputationalWork {
			raw_data: u32,
			elaborated_data: u32,
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
			author: T::AccountId,
			block_height: u32,
		},

		/// Event emitted showing the results of the last computational work.
		LastComputationalWork {
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
			author: T::AccountId,
			block_height: u32,
			is_checked: bool,
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
		) -> DispatchResult{
			let _sender = ensure_signed(origin)?;

			// Get the block height.
			let block_height = <frame_system::Pallet<T>>::block_number().saturated_into::<u32>();

			// The n number of the fibonacci sequence is calculated.
			let elaborated_math_work = Self::math_work_testing(block_height);

			// Hashing the raw data and elaborated data.
			let raw_hashed_data = T::Hashing::hash_of(&block_height);
			let elaborated_hashed_data = T::Hashing::hash_of(&elaborated_math_work);

			// Get the block author.
			let block_digest = <frame_system::Pallet<T>>::digest();
			let digests = block_digest.logs.iter().filter_map(|d| d.as_pre_runtime());
			let author = T::FindAuthor::find_author(digests).unwrap();



			// Store data for possible checks.
			<LastComputationalWork<T>>::put((raw_hashed_data, elaborated_hashed_data, author.clone(), block_height.saturated_into::<u32>()));

			// Set the checked value to false.
			<LastComputationalWorkIsChecked<T>>::put(false);

			// Store the row data for testing purposes.
			<RawData<T>>::put(block_height);

			// Emit an event.
			Self::deposit_event(Event::ResultsComputationalWork {
				raw_data: block_height,
				elaborated_data: elaborated_math_work,
				raw_hash: raw_hashed_data,
				elaborated_hash: elaborated_hashed_data,
				author,
				block_height,
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
			Self::deposit_event(Event::LastComputationalWork {
				raw_hash: last_computational_work.0,
				elaborated_hash: last_computational_work.1,
				author: last_computational_work.2,
				block_height: last_computational_work.3,
				is_checked: false,
			});

			Ok(())
		}
	}
}

impl <T: Config> Pallet<T> {

	/// This function provides the n number of the fibonacci sequence.
	pub fn fibonacci(n: u32) -> u32 {
		match n {
			0 => 0,
			1 => 1,
			_ => Self::fibonacci(n - 1) + Self::fibonacci(n - 2),
		}
	}

	/// This function create a mathematical work to be done, based on block height.
	pub fn math_work_testing(block: u32) -> u32 {
		match block % 3 {
			0 => Self::fibonacci(1),
			1 => Self::fibonacci(2),
			// This is wrong on purpose, to test the check.
			2 => 0,
			_ => 0,
		}
	}


	/// A function that make wrong calculus for testing purposes on block that are multiples of 10.
	pub fn wrong_math_work_testing(block: u32) -> u32 {
		match block % 10 {
			0 => 0,
			_ => {
				match block % 3 {
					0 => Self::fibonacci(1),
					1 => Self::fibonacci(2),
					2 => Self::fibonacci(3),
					_ => 0,
				}
			}
		}
	}

	pub fn set_last_computational_work_is_checked(b: bool) {
		<LastComputationalWorkIsChecked<T>>::put(b);
	}
}
