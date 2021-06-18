#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, traits::EnsureOrigin,
};
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

mod banchmarking;
#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::WeightInfo;

pub(crate) const LOG_TARGET: &'static str = "runtime::barcode scanner";

// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] 💸 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type ManufactureOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;
    type WeightInfo: WeightInfo;
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug)]
pub struct Product<AccountId, Hash> {
    id: Hash,
    name: Vec<u8>,
    manufacturer: AccountId,
    metadata: Vec<u8>, // new field added
}

pub type ProductOf<T> =
    Product<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::Hash>;

// A value placed in storage that represents the current version of the storage. This value
// is used by the `on_runtime_upgrade` logic to determine whether we run storage migration logic.
// This should match directly with the semantic versions of the Rust crate.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
enum Releases {
	V1_0_0,
	V2_0_0,
}

impl Default for Releases {
	fn default() -> Self {
		Releases::V1_0_0
	}
}

decl_storage! {
    trait Store for Module<T: Config> as BarcodeScanner {
        ProductInformation get(fn product_information):
        map hasher(blake2_128_concat) T::Hash => ProductOf<T>;

		/// True if network has been upgraded to this version.
		/// Storage version of the pallet.
		///
		/// This is set to v2.0.0 for new networks.
		StorageVersion build(|_| Releases::V2_0_0): Releases;
    }
}

decl_event!(
    pub enum Event<T>
    where
        Hash = <T as frame_system::Config>::Hash,
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// Product information has been shored.
        ProductInformationStored(AccountId, Hash),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// This barcode already exists in the chain.
        BarcodeAlreadyExists,
    }
}

mod migrations {
	use super::*;
	use frame_support::traits::Get;

	pub fn migrate_all<T: Config>() -> frame_support::weights::Weight {
		log!(info, "Migrating to Releases::V1_0_0");
		ProductInformation::<T>::translate::<(T::Hash, Vec<u8>, T::AccountId), _>(|_key, (id, name, manufacturer)|
			Some(Product { id, name, manufacturer, metadata: "Testing".as_bytes().to_vec()  })
		);
		StorageVersion::put(Releases::V2_0_0);
		log!(info, "Migration Done......");
		T::BlockWeights::get().max_block
	}
}

decl_module! {
    pub struct Module<T: Config> for enum Call
    where
        origin: T::Origin,

    {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        fn on_runtime_upgrade() -> frame_support::weights::Weight {
			match StorageVersion::get() {
				Releases::V2_0_0 => 0,
				Releases::V1_0_0 => migrations::migrate_all::<T>(),
			}
        }

        #[weight = T::WeightInfo::add_product()]
        fn add_product(origin, barcode: T::Hash, name: Vec<u8>, id: T::Hash, metadata: Vec<u8>) -> DispatchResult {

            // The dispatch origin of this call must be `ManufactureOrigin`.
            let sender = T::ManufactureOrigin::ensure_origin(origin)?;

            // Verify whether barcode has been created
            ensure!(!ProductInformation::<T>::contains_key(&barcode), Error::<T>::BarcodeAlreadyExists);

            let product = Product {
                id,
                name,
                manufacturer: sender.clone(),
                metadata,
            };

            ProductInformation::<T>::insert(&barcode, product);

            // Emit the event that barcode has been added in chain for a product
            Self::deposit_event(RawEvent::ProductInformationStored(sender, barcode));

            // Return a successful DispatchResult
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    pub fn is_valid_barcode(barcode: T::Hash) -> bool {
        ProductInformation::<T>::contains_key(&barcode)
    }
}
