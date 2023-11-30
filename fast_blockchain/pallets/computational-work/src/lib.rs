#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Hash;
    use frame_support::traits::FindAuthor;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::SaturatedConversion;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type FindAuthor: FindAuthor<<Self as frame_system::Config>::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        XBlockCannotBeZero,
    }

    // Some default values
    #[pallet::type_value]
    pub fn DefaultCheckAuthor<T: Config>() -> bool {
        true
    }

    #[pallet::type_value]
    pub fn DefaultXBlock<T: Config>() -> u32 {
        0
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

    /// The status of the last computational work to check.
    /// -  `true` - The last computational work has been checked a new one can be submitted.
    /// -  `false` - The last computational work has not been checked yet.
    #[pallet::storage]
    #[pallet::getter(fn last_computational_work_is_checked)]
    pub type LastComputationalWorkIsChecked<T: Config> =
        StorageValue<_, bool, ValueQuery, DefaultCheckAuthor<T>>;

    #[pallet::storage]
    #[pallet::getter(fn x_work_index)]
    pub type CheckEveryXWorkIndex<T: Config> = StorageValue<_, u32, ValueQuery, DefaultXBlock<T>>;

    #[pallet::storage]
    #[pallet::getter(fn x_work)]
    pub type CheckEveryXWorks<T: Config> = StorageValue<_, u32, ValueQuery, DefaultXBlock<T>>;

    // Events of the pallet.

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a new data is hashed.
        /// The data will be only hashed at production time, the not hashed data is shown for testing purposes.
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
        },

        /// Event emitted when the x block is set.
        XBlockSet { x_block: u32 },
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Hashes the raw data and elaborated and store them in the storage with the author and the block number
        /// for future checks.
        #[pallet::weight(100)]
        pub fn hash_work(origin: OriginFor<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Get the block height.
            let block_height = <frame_system::Pallet<T>>::block_number().saturated_into::<u32>();

            // The n number of the fibonacci sequence is calculated.
            let elaborated_math_work = Self::math_work_testing(block_height);

            // Hashing the raw data and elaborated data.
            let raw_hashed_data = T::Hashing::hash_of(&block_height);
            let elaborated_hashed_data = T::Hashing::hash_of(&elaborated_math_work);

            if Self::last_computational_work_is_checked() {
                if Self::x_work_index() == Self::x_work() {
                    // Store data for possible checks.
                    <LastComputationalWork<T>>::put((
                        raw_hashed_data,
                        elaborated_hashed_data,
                        sender.clone(),
                        block_height.saturated_into::<u32>(),
                    ));

                    // Set the checked value to false.
                    <LastComputationalWorkIsChecked<T>>::put(false);

                    // Reset the x block index.
                    <CheckEveryXWorkIndex<T>>::put(0);
                } else {
                    <CheckEveryXWorkIndex<T>>::mutate(|x| *x += 1);
                }
            }

            // Emit an event.
            Self::deposit_event(Event::ResultsComputationalWork {
                raw_data: block_height,
                elaborated_data: elaborated_math_work,
                raw_hash: raw_hashed_data,
                elaborated_hash: elaborated_hashed_data,
                author: sender,
                block_height,
            });

            Ok(())
        }

        /// Set the check every x works value.
        /// Must be more than 0.
        #[pallet::weight(100)]
        pub fn set_check_every_x_works(origin: OriginFor<T>, x: u32) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            // Make sure the x value is more than 0
            ensure!(x > 0, Error::<T>::XBlockCannotBeZero);

            // Change value by 1 to make more human friendly interaction.
            let x_one_based = x - 1;

            // Set the check every x blocks value.
            <CheckEveryXWorks<T>>::put(x_one_based);

            // Reset the x block index.
            <CheckEveryXWorkIndex<T>>::put(0);

            // Emit an event.
            Self::deposit_event(Event::XBlockSet { x_block: x });

            Ok(())
        }

        /// Get the last computational work.
        #[pallet::weight(100)]
        pub fn get_last_computational_work(origin: OriginFor<T>) -> DispatchResult {
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

impl<T: Config> Pallet<T> {
    /// Provides the n number of the fibonacci sequence, for testing purposes.
    pub fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => Self::fibonacci(n - 1) + Self::fibonacci(n - 2),
        }
    }

    /// Create simple mathematical work to be done, based on block height.
    /// If the work is done on a block height that is a multiple of 5 the result is 0, this is to test the voting system and simulate malicious behavior.
    pub fn math_work_testing(block: u32) -> u32 {
        match block % 5 {
            0 => 0,
            _ => Self::fibonacci(10),
        }
    }

    /// Setter function for the last computational work.
    pub fn set_last_computational_work_is_checked(b: bool) {
        <LastComputationalWorkIsChecked<T>>::put(b);
    }
}
