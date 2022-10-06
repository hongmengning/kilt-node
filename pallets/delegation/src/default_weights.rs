// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

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

//! Autogenerated weights for delegation
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-22, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=delegation
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/delegation/src/default_weights.rs
// --template=.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for delegation.
pub trait WeightInfo {
	fn create_hierarchy() -> Weight;
	fn add_delegation() -> Weight;
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight;
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight;
	fn remove_delegation(r: u32, ) -> Weight;
	fn reclaim_deposit(r: u32, ) -> Weight;
	fn can_attest() -> Weight;
	fn can_revoke(c: u32, ) -> Weight;
	fn can_remove(c: u32, ) -> Weight;
	fn transfer_deposit() -> Weight;
}

/// Weights for delegation using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		Weight::from_ref_time(37_014_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		Weight::from_ref_time(44_986_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(19_082_000 as u64)
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(13_360_000 as u64).saturating_mul(r as u64))
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(169_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(32_789_000 as u64)
			// Standard Error: 41_000
			.saturating_add(Weight::from_ref_time(13_000 as u64).saturating_mul(r as u64))
			// Standard Error: 41_000
			.saturating_add(Weight::from_ref_time(5_095_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	fn remove_delegation(r: u32, ) -> Weight {
		Weight::from_ref_time(51_863_000 as u64)
			// Standard Error: 59_000
			.saturating_add(Weight::from_ref_time(23_235_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	fn reclaim_deposit(r: u32, ) -> Weight {
		Weight::from_ref_time(44_669_000 as u64)
			// Standard Error: 56_000
			.saturating_add(Weight::from_ref_time(23_133_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		Weight::from_ref_time(12_484_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_revoke(c: u32, ) -> Weight {
		Weight::from_ref_time(8_127_000 as u64)
			// Standard Error: 38_000
			.saturating_add(Weight::from_ref_time(5_164_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_remove(c: u32, ) -> Weight {
		Weight::from_ref_time(7_991_000 as u64)
			// Standard Error: 35_000
			.saturating_add(Weight::from_ref_time(5_193_000 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	fn transfer_deposit( ) -> Weight {
		Weight::from_ref_time(7_991_000 as u64)
			// Standard Error: 35_000
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	// Storage: Ctype Ctypes (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationNodes (r:0 w:1)
	fn create_hierarchy() -> Weight {
		Weight::from_ref_time(37_014_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn add_delegation() -> Weight {
		Weight::from_ref_time(44_986_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Delegation DelegationNodes (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_root_child(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(19_082_000 as u64)
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(13_360_000 as u64).saturating_mul(r as u64))
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(169_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:6 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn revoke_delegation_leaf(r: u32, c: u32, ) -> Weight {
		Weight::from_ref_time(32_789_000 as u64)
			// Standard Error: 41_000
			.saturating_add(Weight::from_ref_time(13_000 as u64).saturating_mul(r as u64))
			// Standard Error: 41_000
			.saturating_add(Weight::from_ref_time(5_095_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:1 w:1)
	fn remove_delegation(r: u32, ) -> Weight {
		Weight::from_ref_time(51_863_000 as u64)
			// Standard Error: 59_000
			.saturating_add(Weight::from_ref_time(23_235_000 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Delegation DelegationHierarchies (r:0 w:1)
	fn reclaim_deposit(r: u32, ) -> Weight {
		Weight::from_ref_time(44_669_000 as u64)
			// Standard Error: 56_000
			.saturating_add(Weight::from_ref_time(23_133_000 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Delegation DelegationNodes (r:1 w:0)
	// Storage: Delegation DelegationHierarchies (r:1 w:0)
	fn can_attest() -> Weight {
		Weight::from_ref_time(12_484_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_revoke(c: u32, ) -> Weight {
		Weight::from_ref_time(8_127_000 as u64)
			// Standard Error: 38_000
			.saturating_add(Weight::from_ref_time(5_164_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	// Storage: Delegation DelegationNodes (r:2 w:0)
	fn can_remove(c: u32, ) -> Weight {
		Weight::from_ref_time(7_991_000 as u64)
			// Standard Error: 35_000
			.saturating_add(Weight::from_ref_time(5_193_000 as u64).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
	}
	fn transfer_deposit( ) -> Weight {
		Weight::from_ref_time(7_991_000 as u64)
			// Standard Error: 35_000
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
}
