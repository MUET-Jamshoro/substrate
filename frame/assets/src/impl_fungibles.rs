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

//! Implementations for fungibles trait.

use frame_support::traits::{
	fungibles::Credit,
	tokens::{
		Fortitude,
		Precision::{self, BestEffort},
		Preservation::{self, Expendable},
		Provenance::{self, Minted},
	},
};

use super::*;

impl<T: Config<I>, I: 'static> fungibles::Inspect<<T as SystemConfig>::AccountId> for Pallet<T, I> {
	type AssetId = T::AssetId;
	type Balance = T::Balance;

	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		Asset::<T, I>::get(asset).map(|x| x.supply).unwrap_or_else(Zero::zero)
	}

	fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
		Asset::<T, I>::get(asset).map(|x| x.min_balance).unwrap_or_else(Zero::zero)
	}

	fn balance(asset: Self::AssetId, who: &<T as SystemConfig>::AccountId) -> Self::Balance {
		Pallet::<T, I>::balance(asset, who)
	}

	fn total_balance(asset: Self::AssetId, who: &<T as SystemConfig>::AccountId) -> Self::Balance {
		Pallet::<T, I>::balance(asset, who)
	}

	fn reducible_balance(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		keep_alive: Preservation,
		_force: Fortitude,
	) -> Self::Balance {
		Pallet::<T, I>::reducible_balance(asset, who, keep_alive.into()).unwrap_or(Zero::zero())
	}

	fn can_deposit(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
		provenance: Provenance,
	) -> DepositConsequence {
		Pallet::<T, I>::can_increase(asset, who, amount, provenance == Minted)
	}

	fn can_withdraw(
		asset: Self::AssetId,
		who: &<T as SystemConfig>::AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		Pallet::<T, I>::can_decrease(asset, who, amount, false)
	}

	fn asset_exists(asset: Self::AssetId) -> bool {
		Asset::<T, I>::contains_key(asset)
	}
}

impl<T: Config<I>, I: 'static> fungibles::Mutate<<T as SystemConfig>::AccountId> for Pallet<T, I> {}
impl<T: Config<I>, I: 'static> fungibles::Balanced<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	type OnDropCredit = fungibles::DecreaseIssuance<T::AccountId, Self>;
	type OnDropDebt = fungibles::IncreaseIssuance<T::AccountId, Self>;
}

impl<T: Config<I>, I: 'static> fungibles::Unbalanced<T::AccountId> for Pallet<T, I> {
	fn handle_dust(_: fungibles::Dust<T::AccountId, Self>) {
		unreachable!("`decrease_balance` and `increase_balance` have non-default impls; nothing else calls this; qed");
	}
	fn write_balance(
		_: Self::AssetId,
		_: &T::AccountId,
		_: Self::Balance,
	) -> Result<Option<Self::Balance>, DispatchError> {
		unreachable!("write_balance is not used if other functions are impl'd");
	}
	fn set_total_issuance(id: T::AssetId, amount: Self::Balance) {
		Asset::<T, I>::mutate_exists(id, |maybe_asset| {
			if let Some(ref mut asset) = maybe_asset {
				asset.supply = amount
			}
		});
	}
	fn decrease_balance(
		asset: T::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		keep_alive: Preservation,
		_force: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		let f = DebitFlags {
			keep_alive: keep_alive != Expendable,
			best_effort: precision == BestEffort,
		};
		Self::decrease_balance(asset, who, amount, f, |_, _| Ok(()))
	}
	fn increase_balance(
		asset: T::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		_best_effort: Precision,
	) -> Result<Self::Balance, DispatchError> {
		Self::increase_balance(asset, who, amount, |_| Ok(()))?;
		Ok(amount)
	}

	// TODO: #13196 implement deactivate/reactivate once we have inactive balance tracking.
}

impl<T: Config<I>, I: 'static> fungibles::Create<T::AccountId> for Pallet<T, I> {
	fn create(
		id: T::AssetId,
		admin: T::AccountId,
		is_sufficient: bool,
		min_balance: Self::Balance,
	) -> DispatchResult {
		Self::do_force_create(id, admin, is_sufficient, min_balance)
	}
}

impl<T: Config<I>, I: 'static> fungibles::Destroy<T::AccountId> for Pallet<T, I> {
	fn start_destroy(id: T::AssetId, maybe_check_owner: Option<T::AccountId>) -> DispatchResult {
		Self::do_start_destroy(id, maybe_check_owner)
	}

	fn destroy_accounts(id: T::AssetId, max_items: u32) -> Result<u32, DispatchError> {
		Self::do_destroy_accounts(id, max_items)
	}

	fn destroy_approvals(id: T::AssetId, max_items: u32) -> Result<u32, DispatchError> {
		Self::do_destroy_approvals(id, max_items)
	}

	fn finish_destroy(id: T::AssetId) -> DispatchResult {
		Self::do_finish_destroy(id)
	}
}

impl<T: Config<I>, I: 'static> fungibles::metadata::Inspect<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn name(asset: T::AssetId) -> Vec<u8> {
		Metadata::<T, I>::get(asset).name.to_vec()
	}

	fn symbol(asset: T::AssetId) -> Vec<u8> {
		Metadata::<T, I>::get(asset).symbol.to_vec()
	}

	fn decimals(asset: T::AssetId) -> u8 {
		Metadata::<T, I>::get(asset).decimals
	}
}

impl<T: Config<I>, I: 'static> fungibles::metadata::Mutate<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn set(
		asset: T::AssetId,
		from: &<T as SystemConfig>::AccountId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
	) -> DispatchResult {
		Self::do_set_metadata(asset, from, name, symbol, decimals)
	}
}

impl<T: Config<I>, I: 'static> fungibles::approvals::Inspect<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	// Check the amount approved to be spent by an owner to a delegate
	fn allowance(
		asset: T::AssetId,
		owner: &<T as SystemConfig>::AccountId,
		delegate: &<T as SystemConfig>::AccountId,
	) -> T::Balance {
		Approvals::<T, I>::get((asset, &owner, &delegate))
			.map(|x| x.amount)
			.unwrap_or_else(Zero::zero)
	}
}

impl<T: Config<I>, I: 'static> fungibles::approvals::Mutate<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn approve(
		asset: T::AssetId,
		owner: &<T as SystemConfig>::AccountId,
		delegate: &<T as SystemConfig>::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		Self::do_approve_transfer(asset, owner, delegate, amount)
	}

	// Aprove spending tokens from a given account
	fn transfer_from(
		asset: T::AssetId,
		owner: &<T as SystemConfig>::AccountId,
		delegate: &<T as SystemConfig>::AccountId,
		dest: &<T as SystemConfig>::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		Self::do_transfer_approved(asset, owner, delegate, dest, amount)
	}
}

impl<T: Config<I>, I: 'static> fungibles::roles::Inspect<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn owner(asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		Asset::<T, I>::get(asset).map(|x| x.owner)
	}

	fn issuer(asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		Asset::<T, I>::get(asset).map(|x| x.issuer)
	}

	fn admin(asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		Asset::<T, I>::get(asset).map(|x| x.admin)
	}

	fn freezer(asset: T::AssetId) -> Option<<T as SystemConfig>::AccountId> {
		Asset::<T, I>::get(asset).map(|x| x.freezer)
	}
}

impl<T: Config<I>, I: 'static> fungibles::InspectEnumerable<T::AccountId> for Pallet<T, I> {
	type AssetsIterator = KeyPrefixIterator<<T as Config<I>>::AssetId>;

	/// Returns an iterator of the assets in existence.
	///
	/// NOTE: iterating this list invokes a storage read per item.
	fn asset_ids() -> Self::AssetsIterator {
		Asset::<T, I>::iter_keys()
	}
}

impl<T: Config<I>, I: 'static> fungibles::MutateHold<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
}

impl<T: Config<I>, I: 'static> fungibles::InspectHold<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	type Reason = T::HoldIdentifier;

	fn total_balance_on_hold(asset: T::AssetId, who: &T::AccountId) -> T::Balance {
		Zero::zero()
	}
	fn reducible_total_balance_on_hold(
		asset: T::AssetId,
		who: &T::AccountId,
		force: Fortitude,
	) -> T::Balance {
		// // The total balance must never drop below the freeze requirements if we're not forcing:
		// let a = Self::account(who);
		// let unavailable = if force == Force {
		// 	Self::Balance::zero()
		// } else {
		// 	// The freeze lock applies to the total balance, so we can discount the free balance
		// 	// from the amount which the total reserved balance must provide to satisfy it.
		// 	a.frozen.saturating_sub(a.free)
		// };
		// a.reserved.saturating_sub(unavailable)
		Zero::zero()
	}
	fn balance_on_hold(asset: T::AssetId, reason: &Self::Reason, who: &T::AccountId) -> T::Balance {
		// Holds::<T, I>::get(who)
		// 	.iter()
		// 	.find(|x| &x.id == reason)
		// 	.map_or_else(Zero::zero, |x| x.amount)
		Zero::zero()
	}
	fn hold_available(asset: T::AssetId, reason: &Self::Reason, who: &T::AccountId) -> bool {
		false
	}
}

impl<T: Config<I>, I: 'static> fungibles::UnbalancedHold<<T as SystemConfig>::AccountId>
	for Pallet<T, I>
{
	fn set_balance_on_hold(
		asset: T::AssetId,
		reason: &T::HoldIdentifier,
		who: &T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		Ok(())
	}
}

impl<T: Config<I>, I: 'static> fungibles::BalancedHold<T::AccountId> for Pallet<T, I> {
	fn slash(
		asset: T::AssetId,
		reason: &T::HoldIdentifier,
		who: &T::AccountId,
		amount: T::Balance,
	) -> (Credit<T::AccountId, Self>, T::Balance) {
		(<Credit<T::AccountId, Self>>::zero(asset), Zero::zero())
	}
}
