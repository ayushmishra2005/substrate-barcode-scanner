use sp_inherents::{InherentData, InherentIdentifier, ProvideInherentData};
use std::cell::RefCell;
use barcode_scanner_runtime::SLOT_DURATION;
use sp_timestamp::InherentError;

/// Provide a mock duration starting at 0 in millisecond for timestamp inherent.
/// Each call will increment timestamp by slot_duration making block importer
/// think time has passed.
pub struct MockTimestampInherentDataProvider;

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"timstap0";

thread_local!(static TIMESTAMP: RefCell<u64> = RefCell::new(0));

impl ProvideInherentData for MockTimestampInherentDataProvider {
	fn inherent_identifier(&self) -> &'static InherentIdentifier {
		&INHERENT_IDENTIFIER
	}

	fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), sp_inherents::Error> {
		TIMESTAMP.with(|x| {
			*x.borrow_mut() += SLOT_DURATION;
			inherent_data.put_data(INHERENT_IDENTIFIER, &*x.borrow())
		})
	}

	fn error_to_string(&self, error: &[u8]) -> Option<String> {
		InherentError::try_from(&INHERENT_IDENTIFIER, error).map(|e| format!("{:?}", e))
	}
}
