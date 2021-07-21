//! Autogenerated weights for xpallet_gateway_bitcoin_v2_pallet
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-20, STEPS: `[]`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE: 128

// Executed Command:
// target/release/chainx
// benchmark
// --pallet=xpallet_gateway_bitcoin_v2_pallet
// --output
// .
// --extrinsic
// *

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn update_exchange_rate() -> Weight;
    fn register_vault() -> Weight;
    fn add_extra_collateral() -> Weight;
    fn request_issue() -> Weight;
    fn execute_issue() -> Weight;
    fn cancel_issue() -> Weight;
    fn request_redeem() -> Weight;
    fn execute_redeem() -> Weight;
    fn cancel_redeem() -> Weight;
    fn force_update_exchange_rate() -> Weight;
    fn force_update_oracles() -> Weight;
    fn update_issue_griefing_fee() -> Weight;
}

/// Weight functions for xpallet_gateway_bitcoin_v2_pallet.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn update_exchange_rate() -> Weight {
        (90_300_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn register_vault() -> Weight {
        (351_900_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn add_extra_collateral() -> Weight {
        (194_500_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn request_issue() -> Weight {
        (408_900_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn execute_issue() -> Weight {
        (772_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(12 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn cancel_issue() -> Weight {
        (270_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn request_redeem() -> Weight {
        (686_600_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn execute_redeem() -> Weight {
        (592_800_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(12 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn cancel_redeem() -> Weight {
        (533_200_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn force_update_exchange_rate() -> Weight {
        (63_600_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn force_update_oracles() -> Weight {
        (44_300_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn update_issue_griefing_fee() -> Weight {
        (41_400_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
