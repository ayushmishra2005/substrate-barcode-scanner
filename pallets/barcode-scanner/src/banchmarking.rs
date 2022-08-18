
use super::*;

#[allow(unused)]
use crate::Pallet as BarcodeScanner;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

benchmarks! {
    add_product {
        let caller: T::AccountId = whitelisted_caller();
        let barcode: T::Hash = T::Hash::default();
        let id: T::Hash = T::Hash::default();
    }: _(RawOrigin::Root, caller.clone(), barcode, "Test".encode(), id)
    verify {
        assert_last_event::<T>(Event::<T>::ProductInformationStored(caller, barcode).into());
    }

	impl_benchmark_test_suite!(BarcodeScanner, crate::mock::new_test_ext(), crate::mock::Test);
}
