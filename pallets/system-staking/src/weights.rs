// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for the pallet.
pub trait WeightInfo {
	fn on_initialize() -> Weight;
	fn token_config() -> Weight;
	fn delete_token() -> Weight;
	fn refresh_token_info() -> Weight;
	fn payout() -> Weight;
	fn on_redeem_success() -> Weight;
	fn on_redeemed() -> Weight;
}

/// Weight functions for `bifrost_system_staking`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: SystemStaking TokenList (r:1 w:0)
	// Storage: SystemStaking Round (r:1 w:0)
	// Storage: SystemStaking TokenStatus (r:2 w:0)
	fn on_initialize() -> Weight {
		(8_786_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
	// Storage: SystemStaking TokenStatus (r:1 w:1)
	// Storage: SystemStaking TokenList (r:1 w:1)
	fn token_config() -> Weight {
		(10_259_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: SystemStaking TokenStatus (r:1 w:1)
	// Storage: Farming PoolInfos (r:1 w:0)
	fn refresh_token_info() -> Weight {
		(11_340_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: SystemStaking TokenStatus (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: VtokenMinting TokenPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	fn payout() -> Weight {
		(14_587_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
	fn on_redeem_success() -> Weight {
		(110_000 as Weight)
	}
	fn on_redeemed() -> Weight {
		(100_000 as Weight)
	}
	// Storage: SystemStaking TokenList (r:1 w:1)
	// Storage: SystemStaking TokenStatus (r:0 w:1)
	fn delete_token() -> Weight {
		(2_605_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}