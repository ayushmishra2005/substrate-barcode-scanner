use std::sync::Arc;

use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

pub use barcode_scanner_runtime_api::BarcodeScannerApi as BarcodeScannerRuntimeApi;

#[rpc]
pub trait BarcodeScannerApi<BlockHash, Hash> {
	#[rpc(name = "barcode_scanner_is_valid_barcode")]
	fn is_valid_barcode(
		&self,
		encoded_hash: Hash,
		at: Option<BlockHash>
	) -> Result<bool>;
}

/// A struct that implements the `BarcodeScannerApi`.
pub struct BarcodeScanner<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, P> BarcodeScanner<C, P> {
	/// Create new `BarcodeScanner` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, Hash> BarcodeScannerApi<
	<Block as BlockT>::Hash,
	Hash,
> for BarcodeScanner<C, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static,
		C: ProvideRuntimeApi<Block>,
		C: HeaderBackend<Block>,
		C::Api: BarcodeScannerRuntimeApi<Block, Hash>,
		Hash: Codec
{
	fn is_valid_barcode(
		&self,
		encoded_hash: Hash,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<bool> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

		api.is_valid_barcode(&at, encoded_hash).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to verify barcode.".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}

