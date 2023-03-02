#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, sp_runtime::AccountId32};
    use frame_system::{pallet_prelude::*, Account};
    use frame_support::traits::{Currency, Randomness};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Color {
        Red,
        Yellow,
        Blue,
        Green
    }

    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Collectible<T: Config> {
        // Unsigned integers of 16 bytes to represent a unique identifier
        pub unique_id: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub color: Color,
        pub owner: T::AccountId,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId>;
        type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;
        
        #[pallet::constant]
        type MaximumOwned: Get<u32>;
    
    }

    #[pallet::storage]
    pub(super) type CollectiblesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
}