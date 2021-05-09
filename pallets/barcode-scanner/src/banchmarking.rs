#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Module::<T>::events();
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
    }: _(RawOrigin::Signed(caller.clone()), barcode, "Test".encode(), id)
    verify {
        assert_last_event::<T>(Event::<T>::ProductInformationStored(caller, barcode).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_add_product::<Test>());
        });
    }
}
