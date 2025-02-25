// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

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

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 33.0.0
//! DATE: 2024-08-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `rust-2`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release-unoptimized/kilt-parachain
// benchmark
// pallet
// --template=.maintain/runtime-weight-template.hbs
// --header=HEADER-GPL
// --wasm-execution=compiled
// --heap-pages=4096
// --steps=50
// --repeat=20
// --chain
// dev
// --pallet
// pallet-assets
// --extrinsic=*
// --output=./runtimes/peregrine/src/weights/pallet_assets.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `4273`
		// Minimum execution time: 7_511_000 picoseconds.
		Weight::from_parts(8_022_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `4273`
		// Minimum execution time: 7_389_000 picoseconds.
		Weight::from_parts(7_993_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `4273`
		// Minimum execution time: 7_724_000 picoseconds.
		Weight::from_parts(8_024_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:1001 w:1000)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1000 w:1000)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (212 ±0)`
		//  Estimated: `4273 + c * (3207 ±0)`
		// Minimum execution time: 10_650_000 picoseconds.
		Weight::from_parts(11_066_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 6_907
			.saturating_add(Weight::from_parts(9_817_244, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 3207).saturating_mul(c.into()))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Approvals` (r:1001 w:1000)
	/// Proof: `Fungibles::Approvals` (`max_values`: None, `max_size`: Some(746), added: 3221, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447 + a * (86 ±0)`
		//  Estimated: `4273 + a * (3221 ±0)`
		// Minimum execution time: 10_767_000 picoseconds.
		Weight::from_parts(10_982_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 3_058
			.saturating_add(Weight::from_parts(3_935_847, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 3221).saturating_mul(a.into()))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Metadata` (r:1 w:0)
	/// Proof: `Fungibles::Metadata` (`max_values`: None, `max_size`: Some(646), added: 3121, mode: `MaxEncodedLen`)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 9_031_000 picoseconds.
		Weight::from_parts(9_270_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 16_960_000 picoseconds.
		Weight::from_parts(17_381_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4273`
		// Minimum execution time: 22_651_000 picoseconds.
		Weight::from_parts(23_338_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:2 w:2)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `7404`
		// Minimum execution time: 33_107_000 picoseconds.
		Weight::from_parts(33_858_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:2 w:2)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `7404`
		// Minimum execution time: 29_877_000 picoseconds.
		Weight::from_parts(30_769_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:2 w:2)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `7404`
		// Minimum execution time: 32_722_000 picoseconds.
		Weight::from_parts(33_735_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4273`
		// Minimum execution time: 10_858_000 picoseconds.
		Weight::from_parts(11_325_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4273`
		// Minimum execution time: 10_834_000 picoseconds.
		Weight::from_parts(11_284_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `4273`
		// Minimum execution time: 7_493_000 picoseconds.
		Weight::from_parts(7_911_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `4273`
		// Minimum execution time: 7_613_000 picoseconds.
		Weight::from_parts(7_951_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Metadata` (r:1 w:0)
	/// Proof: `Fungibles::Metadata` (`max_values`: None, `max_size`: Some(646), added: 3121, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 9_365_000 picoseconds.
		Weight::from_parts(9_594_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 7_928_000 picoseconds.
		Weight::from_parts(8_388_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Metadata` (r:1 w:1)
	/// Proof: `Fungibles::Metadata` (`max_values`: None, `max_size`: Some(646), added: 3121, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 4]`.
	/// The range of component `s` is `[0, 4]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 9_177_000 picoseconds.
		Weight::from_parts(9_513_737, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 4_348
			.saturating_add(Weight::from_parts(92_631, 0).saturating_mul(n.into()))
			// Standard Error: 4_348
			.saturating_add(Weight::from_parts(76_577, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Metadata` (r:1 w:1)
	/// Proof: `Fungibles::Metadata` (`max_values`: None, `max_size`: Some(646), added: 3121, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `4273`
		// Minimum execution time: 9_824_000 picoseconds.
		Weight::from_parts(10_333_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Metadata` (r:1 w:1)
	/// Proof: `Fungibles::Metadata` (`max_values`: None, `max_size`: Some(646), added: 3121, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 4]`.
	/// The range of component `s` is `[0, 4]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `4273`
		// Minimum execution time: 8_095_000 picoseconds.
		Weight::from_parts(8_730_911, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			// Standard Error: 4_722
			.saturating_add(Weight::from_parts(39_054, 0).saturating_mul(n.into()))
			// Standard Error: 4_722
			.saturating_add(Weight::from_parts(70_013, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Metadata` (r:1 w:1)
	/// Proof: `Fungibles::Metadata` (`max_values`: None, `max_size`: Some(646), added: 3121, mode: `MaxEncodedLen`)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `4273`
		// Minimum execution time: 9_583_000 picoseconds.
		Weight::from_parts(9_811_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 7_572_000 picoseconds.
		Weight::from_parts(7_975_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Approvals` (r:1 w:1)
	/// Proof: `Fungibles::Approvals` (`max_values`: None, `max_size`: Some(746), added: 3221, mode: `MaxEncodedLen`)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `4273`
		// Minimum execution time: 12_806_000 picoseconds.
		Weight::from_parts(13_339_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Approvals` (r:1 w:1)
	/// Proof: `Fungibles::Approvals` (`max_values`: None, `max_size`: Some(746), added: 3221, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:2 w:2)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `604`
		//  Estimated: `7404`
		// Minimum execution time: 41_589_000 picoseconds.
		Weight::from_parts(42_096_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Approvals` (r:1 w:1)
	/// Proof: `Fungibles::Approvals` (`max_values`: None, `max_size`: Some(746), added: 3221, mode: `MaxEncodedLen`)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `480`
		//  Estimated: `4273`
		// Minimum execution time: 14_768_000 picoseconds.
		Weight::from_parts(15_253_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Approvals` (r:1 w:1)
	/// Proof: `Fungibles::Approvals` (`max_values`: None, `max_size`: Some(746), added: 3221, mode: `MaxEncodedLen`)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `480`
		//  Estimated: `4273`
		// Minimum execution time: 14_912_000 picoseconds.
		Weight::from_parts(15_381_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 8_518_000 picoseconds.
		Weight::from_parts(8_741_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 13_139_000 picoseconds.
		Weight::from_parts(13_604_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4273`
		// Minimum execution time: 12_159_000 picoseconds.
		Weight::from_parts(12_811_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `402`
		//  Estimated: `4273`
		// Minimum execution time: 11_554_000 picoseconds.
		Weight::from_parts(11_897_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Asset` (r:1 w:1)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `435`
		//  Estimated: `4273`
		// Minimum execution time: 11_645_000 picoseconds.
		Weight::from_parts(11_851_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Fungibles::Asset` (r:1 w:0)
	/// Proof: `Fungibles::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `Fungibles::Account` (r:1 w:1)
	/// Proof: `Fungibles::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4273`
		// Minimum execution time: 10_550_000 picoseconds.
		Weight::from_parts(10_923_000, 0)
			.saturating_add(Weight::from_parts(0, 4273))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_create() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_force_create() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_start_destroy() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_destroy_accounts() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_destroy_approvals() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_finish_destroy() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_mint() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_burn() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_transfer() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7404
		);
	}
	#[test]
	fn test_transfer_keep_alive() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7404
		);
	}
	#[test]
	fn test_force_transfer() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7404
		);
	}
	#[test]
	fn test_freeze() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_thaw() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_freeze_asset() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_thaw_asset() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_transfer_ownership() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_set_team() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_set_metadata() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_clear_metadata() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_force_set_metadata() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_force_clear_metadata() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_force_asset_status() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_approve_transfer() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_transfer_approved() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 7404
		);
	}
	#[test]
	fn test_cancel_approval() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_force_cancel_approval() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_set_min_balance() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_touch() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_touch_other() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_refund() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_refund_other() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
	#[test]
	fn test_block() {
		assert!(
			<crate::Runtime as frame_system::Config>::BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<sp_weights::Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4273
		);
	}
}
