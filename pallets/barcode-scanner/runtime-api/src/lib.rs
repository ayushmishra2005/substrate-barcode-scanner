#![cfg_attr(not(feature = "std"), no_std)]
use codec::Codec;

sp_api::decl_runtime_apis! {
    /// The API to verify barcode
    pub trait BarcodeScannerApi<Hash> where
		Hash: Codec, {
		/// Verify barcode
        fn is_valid_barcode(barcode: Hash) -> bool;
    }
}
