// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2023 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for pallet_web3_names
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-web3-names
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/pallet_web3_names.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_web3_names`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_web3_names::WeightInfo for WeightInfo<T> {
	// Storage: Web3Names Names (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: Web3Names Banned (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn claim(n: u32, ) -> Weight {
		Weight::from_ref_time(51_648_290 as u64)
			// Standard Error: 60_095
			.saturating_add(Weight::from_ref_time(70_012 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Names (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn release_by_owner() -> Weight {
		Weight::from_ref_time(62_893_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Web3Names Names (r:0 w:1)
	fn reclaim_deposit(_n: u32, ) -> Weight {
		Weight::from_ref_time(50_417_739 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Web3Names Names (r:0 w:1)
	fn ban(_n: u32, ) -> Weight {
		Weight::from_ref_time(48_854_708 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	fn unban(n: u32, ) -> Weight {
		Weight::from_ref_time(27_053_667 as u64)
			// Standard Error: 24_239
			.saturating_add(Weight::from_ref_time(22_077 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Web3Names Names (r:1 w:0)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn change_deposit_owner() -> Weight {
		Weight::from_ref_time(57_277_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn update_deposit() -> Weight {
		Weight::from_ref_time(49_384_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
