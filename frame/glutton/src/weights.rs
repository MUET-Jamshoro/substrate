// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_glutton
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_glutton
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/glutton/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_glutton.
pub trait WeightInfo {
	fn initialize_pallet_grow(n: u32, ) -> Weight;
	fn initialize_pallet_shrink(n: u32, ) -> Weight;
	fn waste_ref_time_iter(i: u32, ) -> Weight;
	fn waste_proof_size_some(i: u32, ) -> Weight;
	fn on_idle_high_proof_waste() -> Weight;
	fn on_idle_low_proof_waste() -> Weight;
	fn empty_on_idle() -> Weight;
	fn set_compute() -> Weight;
	fn set_storage() -> Weight;
}

/// Weights for pallet_glutton using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Glutton TrashDataCount (r:1 w:1)
	/// Proof: Glutton TrashDataCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:0 w:1000)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn initialize_pallet_grow(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 10_316_000 picoseconds.
		Weight::from_parts(10_509_000, 1489)
			// Standard Error: 1_326
			.saturating_add(Weight::from_parts(1_553_036, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: Glutton TrashDataCount (r:1 w:1)
	/// Proof: Glutton TrashDataCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:0 w:1000)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn initialize_pallet_shrink(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1489`
		// Minimum execution time: 10_775_000 picoseconds.
		Weight::from_parts(11_032_000, 1489)
			// Standard Error: 1_253
			.saturating_add(Weight::from_parts(1_058_319, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// The range of component `i` is `[0, 100000]`.
	fn waste_ref_time_iter(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 735_000 picoseconds.
		Weight::from_parts(8_905_307, 0)
			// Standard Error: 47
			.saturating_add(Weight::from_parts(97_251, 0).saturating_mul(i.into()))
	}
	/// Storage: Glutton TrashData (r:5000 w:0)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn waste_proof_size_some(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119036 + i * (1022 ±0)`
		//  Estimated: `990 + i * (3016 ±0)`
		// Minimum execution time: 721_000 picoseconds.
		Weight::from_parts(780_000, 990)
			// Standard Error: 2_674
			.saturating_add(Weight::from_parts(5_478_644, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 3016).saturating_mul(i.into()))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:1737 w:0)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	fn on_idle_high_proof_waste() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1900466`
		//  Estimated: `5242760`
		// Minimum execution time: 44_911_437_000 picoseconds.
		Weight::from_parts(45_215_724_000, 5242760)
			.saturating_add(T::DbWeight::get().reads(1739_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:5 w:0)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	fn on_idle_low_proof_waste() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9516`
		//  Estimated: `19048`
		// Minimum execution time: 78_846_644_000 picoseconds.
		Weight::from_parts(79_233_500_000, 19048)
			.saturating_add(T::DbWeight::get().reads(7_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2978`
		// Minimum execution time: 4_520_000 picoseconds.
		Weight::from_parts(4_647_000, 2978)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: Glutton Compute (r:0 w:1)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_compute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_044_000 picoseconds.
		Weight::from_parts(9_270_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Glutton Storage (r:0 w:1)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_813_000 picoseconds.
		Weight::from_parts(9_184_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Glutton TrashDataCount (r:1 w:1)
	/// Proof: Glutton TrashDataCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:0 w:1000)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn initialize_pallet_grow(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 10_316_000 picoseconds.
		Weight::from_parts(10_509_000, 1489)
			// Standard Error: 1_326
			.saturating_add(Weight::from_parts(1_553_036, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: Glutton TrashDataCount (r:1 w:1)
	/// Proof: Glutton TrashDataCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:0 w:1000)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn initialize_pallet_shrink(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1489`
		// Minimum execution time: 10_775_000 picoseconds.
		Weight::from_parts(11_032_000, 1489)
			// Standard Error: 1_253
			.saturating_add(Weight::from_parts(1_058_319, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// The range of component `i` is `[0, 100000]`.
	fn waste_ref_time_iter(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 735_000 picoseconds.
		Weight::from_parts(8_905_307, 0)
			// Standard Error: 47
			.saturating_add(Weight::from_parts(97_251, 0).saturating_mul(i.into()))
	}
	/// Storage: Glutton TrashData (r:5000 w:0)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn waste_proof_size_some(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119036 + i * (1022 ±0)`
		//  Estimated: `990 + i * (3016 ±0)`
		// Minimum execution time: 721_000 picoseconds.
		Weight::from_parts(780_000, 990)
			// Standard Error: 2_674
			.saturating_add(Weight::from_parts(5_478_644, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 3016).saturating_mul(i.into()))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:1737 w:0)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	fn on_idle_high_proof_waste() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1900466`
		//  Estimated: `5242760`
		// Minimum execution time: 44_911_437_000 picoseconds.
		Weight::from_parts(45_215_724_000, 5242760)
			.saturating_add(RocksDbWeight::get().reads(1739_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton TrashData (r:5 w:0)
	/// Proof: Glutton TrashData (max_values: Some(65000), max_size: Some(1036), added: 3016, mode: MaxEncodedLen)
	fn on_idle_low_proof_waste() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9516`
		//  Estimated: `19048`
		// Minimum execution time: 78_846_644_000 picoseconds.
		Weight::from_parts(79_233_500_000, 19048)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
	}
	/// Storage: Glutton Storage (r:1 w:0)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Glutton Compute (r:1 w:0)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2978`
		// Minimum execution time: 4_520_000 picoseconds.
		Weight::from_parts(4_647_000, 2978)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: Glutton Compute (r:0 w:1)
	/// Proof: Glutton Compute (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_compute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_044_000 picoseconds.
		Weight::from_parts(9_270_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Glutton Storage (r:0 w:1)
	/// Proof: Glutton Storage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_813_000 picoseconds.
		Weight::from_parts(9_184_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
