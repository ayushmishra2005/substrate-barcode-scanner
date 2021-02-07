#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, ensure, traits::EnsureOrigin,
};
use sp_runtime::DispatchResult;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod banchmarking;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type ManufactureOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, Debug)]
pub struct Product<AccountId, Hash> {
	id: Hash,
	name: Vec<u8>,
	manufacturer: AccountId,
}

pub type ProductOf<T> = Product<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::Hash>;

decl_storage! {
	trait Store for Module<T: Config> as BarcodeScanner {
		ProductInformation get(fn product_information):
		map hasher(blake2_128_concat) T::Hash => ProductOf<T>;
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

decl_module! {
	pub struct Module<T: Config> for enum Call
	where
		origin: T::Origin,

	{
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10000]
        fn add_product(origin, barcode: T::Hash, name: Vec<u8>, id: T::Hash) -> DispatchResult {

            // The dispatch origin of this call must be `ManufactureOrigin`.
            let sender = T::ManufactureOrigin::ensure_origin(origin)?;

            // Verify whether barcode has been created
            ensure!(!ProductInformation::<T>::contains_key(&barcode), Error::<T>::BarcodeAlreadyExists);

            let product = Product {
                id,
                name,
                manufacturer: sender.clone(),
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
