#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
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
	use frame_support::traits::FindAuthor;
	use frame_support::sp_runtime::traits::Hash;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_computational_work::Config + pallet_session::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The pallet computational work.
		type FindAuthor: FindAuthor<Self::AccountId>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(T::AccountId),

		TestEvent {
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
			checked_author: T::AccountId,
			block_height: u32,
			current_author: T::AccountId,
			is_passed: bool,
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(100)]
		pub fn check_computational_work(
			origin: OriginFor<T>,
			number: u32,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let _sender = ensure_signed(origin)?;

			// Get the last computational work from pallet_computational_work.
			let last_computational_work = pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

			// Get the author of the block.
			// Get the block author.
			let block_digest = <frame_system::Pallet<T>>::digest();
			let digests = block_digest.logs.iter().filter_map(|d| d.as_pre_runtime());
			let current_author = <T as pallet_computational_work::Config>::FindAuthor::find_author(digests).unwrap();


			// Use the math function to check if the work is correct.
			let check_computational_work = pallet_computational_work::Pallet::<T>::math_work_testing(number);
			let check_computational_work_hashed = T::Hashing::hash_of(&check_computational_work);

			let is_passed = check_computational_work_hashed == last_computational_work.1;

			// Emit an event.
			Self::deposit_event(Event::TestEvent {
				raw_hash: last_computational_work.0,
				elaborated_hash: last_computational_work.1,
				checked_author: last_computational_work.2,
				block_height: last_computational_work.3,
				current_author,
				is_passed
			});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
