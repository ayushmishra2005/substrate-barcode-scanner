#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use sp_std::convert::TryInto;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
#[cfg(feature = "runtime-benchmarks")]
mod banchmarking;

pub use weights::WeightInfo;

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Product<AccountId, Hash> {
	id: Hash,
	name: Vec<u8>,
	manufacturer: AccountId,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ManufactureOrigin: EnsureOrigin<Self::Origin>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn product_information)]
	pub type ProductInformation<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Product<T::AccountId, T::Hash>>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Product information has been shored. [who, hash]
		ProductInformationStored(T::AccountId, T::Hash),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This barcode already exists in the chain.
		BarcodeAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(T::WeightInfo::add_product())]
		pub fn add_product(origin: OriginFor<T>, manufacture: T::AccountId, barcode: T::Hash, name: Vec<u8>, id: T::Hash) -> DispatchResult {
			T::ManufactureOrigin::ensure_origin(origin)?;

			// The dispatch origin of this call must be `ManufactureOrigin`.
			let sender = manufacture;

			// Verify whether barcode has been created
			ensure!(!ProductInformation::<T>::contains_key(&barcode), Error::<T>::BarcodeAlreadyExists);

			let product = Product {
				id,
				name,
				manufacturer: sender.clone(),
			};

			ProductInformation::<T>::insert(&barcode, product);
			// Emit an event.
			Self::deposit_event(Event::ProductInformationStored(sender, barcode));
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn is_valid_barcode(barcode: T::Hash) -> bool {
		ProductInformation::<T>::contains_key(&barcode)
	}
}
