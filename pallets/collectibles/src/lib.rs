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
    #[scale_info(skip_type_params(T))]
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
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type MaximumOwned: Get<u32>;
    }

    #[pallet::storage]
    pub(super) type CollectiblesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
    
    /// Maps the Collectible struct to the unique_id.
    #[pallet::storage]
    pub(super) type CollectibleMap<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Collectible<T>>;

    /// Track the collectibles owned by each account.
    #[pallet::storage]
    pub(super) type OwnerOfCollectibles<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaximumOwned>,
        ValueQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        /// Each collectible must have a unique identifier
        DuplicateCollectible,
        /// An account can't exceed the `MaximumOwned` constant
        MaximumCollectiblesOwned,
        /// The total supply of collectibles can't exceed the u64 limit
        BoundsOverflow,
        /// The collectible doesn't exist
        NoCollectible,
        /// You are not the owner
        NotOwner,
        /// Trying to transfer a collectible to yourself
        TransferToSelf,
    }
    
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
	  /// A new collectible was successfully created.
		CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
    }

    // Pallet callable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        /// Create a new unique collectible.
        ///
        /// The actual collectible creation is done in the `mint()` function.
        #[pallet::weight(0)]
        pub fn create_collectible(origin: OriginFor<T>) -> DispatchResult {
            // Make sure the caller is from a signed origin
                let sender = ensure_signed(origin)?;
                
                // Generate the unique_id and color using a helper function
                let (collectible_gen_unique_id, color) = Self::gen_unique_id();
                
                // Write new collectible to storage by calling helper function
                Self::mint(&sender, collectible_gen_unique_id, color)?;
                
                Ok(())
            }
    }

    // Pallet internal functions
    impl<T: Config> Pallet<T> {
        // Generates and returns the unique_id and color
        fn gen_unique_id() -> ([u8; 16], Color) {
            // Create randomness
            let random = T::CollectionRandomness::random(&b"unique_id"[..]).0;
            
            // Create randomness payload. Multiple collectibles can be generated in the same block,
            // retaining uniqueness.
            let unique_payload = (
                random,
                frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),frame_system::Pallet::<T>::block_number(),
        );
        
        // Turns into a byte array
        let encoded_payload = unique_payload.encode();
        let hash = frame_support::Hashable::blake2_128(&encoded_payload);
        
        // Generate Color 
        if hash[0] % 2 == 0 {
                (hash, Color::Red)
        } else {
                (hash, Color::Yellow)
        } 
        }

        // Function to mint a collectible
        pub fn mint(
            owner: &T::AccountId,
            unique_id: [u8; 16],
            color: Color,
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let collectible = Collectible::<T> { unique_id, price: None, color, owner: owner.clone() };
            
            // Check if the collectible exists in the storage map
            ensure!(!CollectibleMap::<T>::contains_key(&collectible.unique_id), Error::<T>::DuplicateCollectible);
            
            // Check that a new collectible can be created
            let count = CollectiblesCount::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::BoundsOverflow)?;
            
            // Append collectible to OwnerOfCollectibles map
            OwnerOfCollectibles::<T>::try_append(&owner, collectible.unique_id)
                .map_err(|_| Error::<T>::MaximumCollectiblesOwned)?;
            
            // Write new collectible to storage and update the count
            CollectibleMap::<T>::insert(collectible.unique_id, collectible);
            CollectiblesCount::<T>::put(new_count);
            
            // Deposit the "Collectiblereated" event.
            Self::deposit_event(Event::CollectibleCreated { collectible: unique_id, owner: owner.clone() });
            
            // Returns the unique_id of the new collectible if this succeeds
            Ok(unique_id)
        }
    }
}