#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//TODO: Disable benchmarking to fix CI but to get back once it is added
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::FindAuthor;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_computational_work::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The pallet computational work.
        type FindAuthor: FindAuthor<Self::AccountId>;
    }

    // Set the default value for the author check condition.
    #[pallet::type_value]
    pub fn DefaultCheckAuthor<T: Config>() -> bool {
        false
    }

    // First Author
    // Store if the first author has been checked.
    #[pallet::storage]
    #[pallet::getter(fn first_author_has_checked)]
    pub type FirstAuthorHasChecked<T: Config> =
        StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

    // Store the first author.
    #[pallet::storage]
    #[pallet::getter(fn first_author)]
    pub type FirstAuthor<T: Config> = StorageValue<_, T::AccountId>;

    // Store the check result of the first author.
    #[pallet::storage]
    #[pallet::getter(fn first_author_check_result)]
    pub type FirstAuthorIsPassed<T: Config> = StorageValue<_, bool>;

    // Second Author
    // Store if the second author has been checked.
    #[pallet::storage]
    #[pallet::getter(fn second_author_has_checked)]
    pub type SecondAuthorHasChecked<T: Config> =
        StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

    // Store the second author.
    #[pallet::storage]
    #[pallet::getter(fn second_author)]
    pub type SecondAuthor<T: Config> = StorageValue<_, T::AccountId>;

    // Store the check result of the second author.
    #[pallet::storage]
    #[pallet::getter(fn second_author_check_result)]
    pub type SecondAuthorIsPassed<T: Config> = StorageValue<_, bool>;

    // Third Author
    // Store if the third author has been checked.
    #[pallet::storage]
    #[pallet::getter(fn third_author_has_checked)]
    pub type ThirdAuthorHasChecked<T: Config> =
        StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

    // Store the third author.
    #[pallet::storage]
    #[pallet::getter(fn third_author)]
    pub type ThirdAuthor<T: Config> = StorageValue<_, T::AccountId>;

    // Store the check result of the third author.
    #[pallet::storage]
    #[pallet::getter(fn third_author_check_result)]
    pub type ThirdAuthorIsPassed<T: Config> = StorageValue<_, bool>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emit an event when an author has been checked.
        CheckResult {
            raw_hash: T::Hash,
            elaborated_hash: T::Hash,
            checked_author: T::AccountId,
            block_height: u32,
            current_author: T::AccountId,
            is_passed: bool,
        },

        /// Emit an event whit the final result of the check.
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
                let last_computational_work =
                    pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

                // If all the check are done, emit the final result.
                if FirstAuthorHasChecked::<T>::get()
                    && SecondAuthorHasChecked::<T>::get()
                    && ThirdAuthorHasChecked::<T>::get()
                {
                    // Check the final result and emit the event.
                    Self::check_result();

                    // Reset the check.
                    Self::reset_check_process();
                }

                // Get the current block author.
                let block_digest = <frame_system::Pallet<T>>::digest();
                let digests = block_digest.logs.iter().filter_map(|d| d.as_pre_runtime());
                let current_author =
                    <T as pallet_computational_work::Config>::FindAuthor::find_author(digests)
                        .unwrap();

                if last_computational_work.2 != current_author {
                    if !FirstAuthorHasChecked::<T>::get() {
                        // Check the computational work, get the result and the block height.
                        let (is_passed, block_height) = Self::check_computational_work();

                        // Set the first author.
                        FirstAuthor::<T>::put(current_author.clone());

                        // Set the check result of the first author.
                        FirstAuthorIsPassed::<T>::put(is_passed);

                        // Set the check has been done for the first author.
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
                    } else if FirstAuthorHasChecked::<T>::get()
                        && !SecondAuthorHasChecked::<T>::get()
                        && FirstAuthor::<T>::get().unwrap() != current_author
                    {
                        // Check the computational work, get the result and the block height.
                        let (is_passed, block_height) = Self::check_computational_work();

                        // Set the second author.
                        SecondAuthor::<T>::put(current_author.clone());

                        // Set the check result of the second author.
                        SecondAuthorIsPassed::<T>::put(is_passed);

                        // Set the check has been done for the second author.
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
                    } else if FirstAuthorHasChecked::<T>::get()
                        && SecondAuthorHasChecked::<T>::get()
                        && !ThirdAuthorHasChecked::<T>::get()
                        && FirstAuthor::<T>::get().unwrap() != current_author
                        && SecondAuthor::<T>::get().unwrap() != current_author
                    {
                        // Check the computational work, get the result and the block height.
                        let (is_passed, block_height) = Self::check_computational_work();

                        // Set the third author.
                        ThirdAuthor::<T>::put(current_author.clone());

                        // Set the check result of the third author.
                        ThirdAuthorIsPassed::<T>::put(is_passed);

                        // Set the check has been done for the third author.
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

            frame_support::weights::Weight::from_all(0u64)
            // Set weight to 0 just for testing.
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Check the computational work.
    /// Returns a tuple of (is_passed, block_height).
    /// is_passed: true if the computational work is passed, false otherwise.
    /// block_height: the block height of the checked computational work.
    pub fn check_computational_work() -> (bool, u32) {
        use frame_support::sp_runtime::traits::Hash;
        use sp_runtime::traits::SaturatedConversion;

        // Get the last computational work.
        let last_computational_work =
            pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

        // Get the current block height.
        let block_height = <frame_system::Pallet<T>>::block_number().saturated_into::<u32>();

        // Check the computational work.
        // Calling math_work_testing() for PoC.
        let check_computational_work =
            pallet_computational_work::Pallet::<T>::math_work_testing(block_height);
        // Hash the check computational work.
        let check_computational_work_hashed = T::Hashing::hash_of(&check_computational_work);

        // Compare the check computational work with the last computational work.
        (
            check_computational_work_hashed == last_computational_work.1,
            block_height,
        )
    }

    /// Elaborate the final check result.
    /// If 2/3 of the authors passed the check, the check is passed.
    pub fn check_result() {
        use scale_info::prelude::vec;

        // Get the check results from the storage.
        let check_results = vec![
            FirstAuthorIsPassed::<T>::get().unwrap(),
            SecondAuthorIsPassed::<T>::get().unwrap(),
            ThirdAuthorIsPassed::<T>::get().unwrap(),
        ];

        // Count the number of true values.
        let mut votes = 0;
        for check_results in check_results {
            if check_results {
                votes += 1;
            }
        }

        // Get last computational work for the event.
        let last_computational_work =
            pallet_computational_work::Pallet::<T>::last_computational_work().unwrap();

        // If 2/3 of the authors passed the check, the check is passed.
        // emit the final check result event.
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
    }

    /// Resetting the check process
    pub fn reset_check_process() {
        // Checks are done, reset the checks.
        FirstAuthorHasChecked::<T>::put(false);
        SecondAuthorHasChecked::<T>::put(false);
        ThirdAuthorHasChecked::<T>::put(false);

        // Reset the voters.
        FirstAuthor::<T>::kill();
        SecondAuthor::<T>::kill();
        ThirdAuthor::<T>::kill();

        // Set the last computational work as checked.
        pallet_computational_work::Pallet::<T>::set_last_computational_work_is_checked(true);
    }
}
