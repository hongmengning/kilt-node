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

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-27, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=1
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/did/src/default_weights.rs
// --template=.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for did.
pub trait WeightInfo {
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight;
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight;
	fn create_ecdsa_keys(n: u32, c: u32, ) -> Weight;
	fn delete(c: u32, ) -> Weight;
	fn reclaim_deposit(c: u32, ) -> Weight;
	fn submit_did_call_ed25519_key() -> Weight;
	fn submit_did_call_sr25519_key() -> Weight;
	fn submit_did_call_ecdsa_key() -> Weight;
	fn set_ed25519_authentication_key() -> Weight;
	fn set_sr25519_authentication_key() -> Weight;
	fn set_ecdsa_authentication_key() -> Weight;
	fn set_ed25519_delegation_key() -> Weight;
	fn set_sr25519_delegation_key() -> Weight;
	fn set_ecdsa_delegation_key() -> Weight;
	fn remove_ed25519_delegation_key() -> Weight;
	fn remove_sr25519_delegation_key() -> Weight;
	fn remove_ecdsa_delegation_key() -> Weight;
	fn set_ed25519_attestation_key() -> Weight;
	fn set_sr25519_attestation_key() -> Weight;
	fn set_ecdsa_attestation_key() -> Weight;
	fn remove_ed25519_attestation_key() -> Weight;
	fn remove_sr25519_attestation_key() -> Weight;
	fn remove_ecdsa_attestation_key() -> Weight;
	fn add_ed25519_key_agreement_key() -> Weight;
	fn add_sr25519_key_agreement_key() -> Weight;
	fn add_ecdsa_key_agreement_key() -> Weight;
	fn remove_ed25519_key_agreement_key() -> Weight;
	fn remove_sr25519_key_agreement_key() -> Weight;
	fn remove_ecdsa_key_agreement_key() -> Weight;
	fn add_service_endpoint() -> Weight;
	fn remove_service_endpoint() -> Weight;
	fn signature_verification_sr25519(l: u32, ) -> Weight;
	fn signature_verification_ed25519(l: u32, ) -> Weight;
	fn signature_verification_ecdsa(l: u32, ) -> Weight;
	fn transfer_deposit() -> Weight;
}

/// Weights for did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_ed25519_keys(_n: u32, c: u32, ) -> Weight {
		(436_583_000_u64)
			// Standard Error: 1_760_000
			.saturating_add((9_755_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		(354_663_000_u64)
			// Standard Error: 649_000
			.saturating_add((3_972_000_u64).saturating_mul(n as Weight))
			// Standard Error: 243_000
			.saturating_add((19_126_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_ecdsa_keys(_n: u32, c: u32, ) -> Weight {
		(706_144_000_u64)
			// Standard Error: 434_000
			.saturating_add((10_592_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn delete(c: u32, ) -> Weight {
		(37_058_000_u64)
			// Standard Error: 22_000
			.saturating_add((2_646_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn reclaim_deposit(c: u32, ) -> Weight {
		(75_498_000_u64)
			// Standard Error: 90_000
			.saturating_add((1_372_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(94_698_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(88_897_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(250_190_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		(86_282_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		(124_704_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		(121_388_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		(113_934_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		(99_958_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		(121_038_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		(106_690_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		(110_408_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		(103_364_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		(114_004_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		(102_743_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		(99_226_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		(106_430_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		(101_260_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		(102_934_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		(106_259_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		(109_486_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		(100_199_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		(99_136_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		(114_495_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		(74_631_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn add_service_endpoint() -> Weight {
		(52_909_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn remove_service_endpoint() -> Weight {
		(61_756_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		(123_502_000_u64)
			// Standard Error: 0
			.saturating_add((5_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		(130_465_000_u64)
			// Standard Error: 0
			.saturating_add((3_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		(286_956_000_u64)
			// Standard Error: 0
			.saturating_add((1_000_u64).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn transfer_deposit() -> Weight {
		(61_756_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_ed25519_keys(_n: u32, c: u32, ) -> Weight {
		(436_583_000_u64)
			// Standard Error: 1_760_000
			.saturating_add((9_755_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		(354_663_000_u64)
			// Standard Error: 649_000
			.saturating_add((3_972_000_u64).saturating_mul(n as Weight))
			// Standard Error: 243_000
			.saturating_add((19_126_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn create_ecdsa_keys(_n: u32, c: u32, ) -> Weight {
		(706_144_000_u64)
			// Standard Error: 434_000
			.saturating_add((10_592_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn delete(c: u32, ) -> Weight {
		(37_058_000_u64)
			// Standard Error: 22_000
			.saturating_add((2_646_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn reclaim_deposit(c: u32, ) -> Weight {
		(75_498_000_u64)
			// Standard Error: 90_000
			.saturating_add((1_372_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(94_698_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(88_897_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(250_190_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ed25519_authentication_key() -> Weight {
		(86_282_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_sr25519_authentication_key() -> Weight {
		(124_704_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_authentication_key() -> Weight {
		(121_388_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ed25519_delegation_key() -> Weight {
		(113_934_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_sr25519_delegation_key() -> Weight {
		(99_958_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_delegation_key() -> Weight {
		(121_038_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_delegation_key() -> Weight {
		(106_690_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_delegation_key() -> Weight {
		(110_408_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_delegation_key() -> Weight {
		(103_364_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ed25519_attestation_key() -> Weight {
		(114_004_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_sr25519_attestation_key() -> Weight {
		(102_743_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn set_ecdsa_attestation_key() -> Weight {
		(99_226_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_attestation_key() -> Weight {
		(106_430_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_attestation_key() -> Weight {
		(101_260_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_attestation_key() -> Weight {
		(102_934_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_ed25519_key_agreement_key() -> Weight {
		(106_259_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_sr25519_key_agreement_key() -> Weight {
		(109_486_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_ecdsa_key_agreement_key() -> Weight {
		(100_199_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ed25519_key_agreement_key() -> Weight {
		(99_136_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_sr25519_key_agreement_key() -> Weight {
		(114_495_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn remove_ecdsa_key_agreement_key() -> Weight {
		(74_631_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn add_service_endpoint() -> Weight {
		(52_909_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn remove_service_endpoint() -> Weight {
		(61_756_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		(123_502_000_u64)
			// Standard Error: 0
			.saturating_add((5_000_u64).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		(130_465_000_u64)
			// Standard Error: 0
			.saturating_add((3_000_u64).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		(286_956_000_u64)
			// Standard Error: 0
			.saturating_add((1_000_u64).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn transfer_deposit() -> Weight {
		(52_909_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
