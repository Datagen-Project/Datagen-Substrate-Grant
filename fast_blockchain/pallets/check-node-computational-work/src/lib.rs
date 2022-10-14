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
	use frame_support::traits::FindAuthor;
	// use frame_support::sp_runtime::traits::Hash;
	// use sp_runtime::traits::SaturatedConversion;
	use scale_info::prelude::vec;


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

	#[pallet::type_value]
	pub fn DefaultCheckAuthor<T: Config>() -> bool {
		false
	}

	// First Author
	#[pallet::storage]
	#[pallet::getter(fn first_author_has_checked)]
	pub type FirstAuthorHasChecked<T: Config> = StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

	#[pallet::storage]
	pub type FirstAuthor<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	pub type FirstAuthorIsPassed<T: Config> = StorageValue<_, bool>;

	// Second Author
	#[pallet::storage]
	#[pallet::getter(fn second_author_has_checked)]
	pub type SecondAuthorHasChecked<T: Config> = StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

	#[pallet::storage]
	pub type SecondAuthor<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	pub type SecondAuthorIsPassed<T: Config> = StorageValue<_, bool>;

	// Third Author
	#[pallet::storage]
	#[pallet::getter(fn third_author_has_checked)]
	pub type ThirdAuthorHasChecked<T: Config> = StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

	#[pallet::storage]
	pub type ThirdAuthor<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	pub type ThirdAuthorIsPassed<T: Config> = StorageValue<_, bool>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// Check result of computational work.
		CheckResult {
			raw_hash: T::Hash,
			elaborated_hash: T::Hash,
			checked_author: T::AccountId,
			block_height: u32,
			current_author: T::AccountId,
			is_passed: bool,
		},

		Test {
			number: T::BlockNumber,
		},

		FinalResult {
			checked_author: T::AccountId,
			controller1: T::AccountId,
			result1: bool,
			controller2: T::AccountId,
			result2: bool,
			controller3: T::AccountId,
			result3: bool,
			is_passed: bool,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}


	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> frame_support::weights::Weight {

			// Check if there is a computational work to check.
			if !pallet_computational_work::Pallet::<T>::last_computational_work_is_checked() {
				let last_computational_work = pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

					if FirstAuthorHasChecked::<T>::get() && SecondAuthorHasChecked::<T>::get() && ThirdAuthorHasChecked::<T>::get() {
						let check_results = vec![
							FirstAuthorIsPassed::<T>::get().unwrap(),
							SecondAuthorIsPassed::<T>::get().unwrap(),
							ThirdAuthorIsPassed::<T>::get().unwrap(),
						];

						let mut votes = 0;
						for check_results in check_results {
							if check_results {
								votes += 1;
							}
						}

						let last_computational_work = pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

						if votes >= 2 {
							Self::deposit_event(Event::FinalResult {
								checked_author: last_computational_work.2,
								controller1: FirstAuthor::<T>::get().unwrap(),
								result1: FirstAuthorIsPassed::<T>::get().unwrap(),
								controller2: SecondAuthor::<T>::get().unwrap(),
								result2: SecondAuthorIsPassed::<T>::get().unwrap(),
								controller3: ThirdAuthor::<T>::get().unwrap(),
								result3: ThirdAuthorIsPassed::<T>::get().unwrap(),
								is_passed: true,
							});
						} else {
							Self::deposit_event(Event::FinalResult {
								checked_author: last_computational_work.2,
								controller1: FirstAuthor::<T>::get().unwrap(),
								result1: FirstAuthorIsPassed::<T>::get().unwrap(),
								controller2: SecondAuthor::<T>::get().unwrap(),
								result2: SecondAuthorIsPassed::<T>::get().unwrap(),
								controller3: ThirdAuthor::<T>::get().unwrap(),
								result3: ThirdAuthorIsPassed::<T>::get().unwrap(),
								is_passed: false,
							});
						}


						// Checks are done, reset the checks.
						FirstAuthorHasChecked::<T>::put(false);
						SecondAuthorHasChecked::<T>::put(false);
						ThirdAuthorHasChecked::<T>::put(false);

						// Reset the votes.
						FirstAuthor::<T>::kill();
						SecondAuthor::<T>::kill();
						ThirdAuthor::<T>::kill();

						pallet_computational_work::Pallet::<T>::set_last_computational_work_is_checked(true);
					}

					// Get the current block author.
					let block_digest = <frame_system::Pallet<T>>::digest();
					let digests = block_digest.logs.iter().filter_map(|d| d.as_pre_runtime());
					let current_author = <T as pallet_computational_work::Config>::FindAuthor::find_author(digests).unwrap();

					if last_computational_work.2 != current_author {
						if !FirstAuthorHasChecked::<T>::get() {

							let (is_passed, block_height) = Self::check_computational_work();

							FirstAuthor::<T>::put(current_author.clone());

							// Set the check result.
							FirstAuthorIsPassed::<T>::put(is_passed);

							// Set the check has been done.
							FirstAuthorHasChecked::<T>::put(true);

							// Emit the check result event.
							Self::deposit_event(Event::CheckResult {
								raw_hash: last_computational_work.0,
								elaborated_hash: last_computational_work.1,
								checked_author: last_computational_work.2,
								block_height,
								current_author,
								is_passed,
							});

						} else if FirstAuthorHasChecked::<T>::get() && !SecondAuthorHasChecked::<T>::get() && FirstAuthor::<T>::get().unwrap() != current_author {

							let (is_passed, block_height) = Self::check_computational_work();

							SecondAuthor::<T>::put(current_author.clone());

							// Set the check result.
							SecondAuthorIsPassed::<T>::put(is_passed);

							// Set the check has been done.
							SecondAuthorHasChecked::<T>::put(true);

							// Emit the check result event.
							Self::deposit_event(Event::CheckResult {
								raw_hash: last_computational_work.0,
								elaborated_hash: last_computational_work.1,
								checked_author: last_computational_work.2,
								block_height,
								current_author,
								is_passed,
							});
						} else if FirstAuthorHasChecked::<T>::get() && SecondAuthorHasChecked::<T>::get() && !ThirdAuthorHasChecked::<T>::get() && FirstAuthor::<T>::get().unwrap() != current_author && SecondAuthor::<T>::get().unwrap() != current_author {

							let (is_passed, block_height) = Self::check_computational_work();

							ThirdAuthor::<T>::put(current_author.clone());

							// Set the check result.
							ThirdAuthorIsPassed::<T>::put(is_passed);

							// Set the check has been done.
							ThirdAuthorHasChecked::<T>::put(true);

							// Emit the check result event.
							Self::deposit_event(Event::CheckResult {
								raw_hash: last_computational_work.0,
								elaborated_hash: last_computational_work.1,
								checked_author: last_computational_work.2,
								block_height,
								current_author,
								is_passed,
							});
						}
					}
			}
			0
		}
	}
}

impl <T: Config> Pallet<T> {
	pub fn check_computational_work() -> (bool, u32) {
		use sp_runtime::traits::SaturatedConversion;
		use frame_support::sp_runtime::traits::Hash;

		let last_computational_work = pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

		let block_height = <frame_system::Pallet<T>>::block_number().saturated_into::<u32>();

		// Check the computational work.
		let check_computational_work = pallet_computational_work::Pallet::<T>::wrong_math_work_testing(block_height);
		let check_computational_work_hashed = T::Hashing::hash_of(&check_computational_work);

		(check_computational_work_hashed == last_computational_work.1, block_height)
	}
}
