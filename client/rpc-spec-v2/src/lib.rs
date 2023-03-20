// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Substrate JSON-RPC interface v2.
//!
//! Specification [document](https://paritytech.github.io/json-rpc-interface-spec/).

#![warn(missing_docs)]
#![deny(unused_crate_dependencies)]

use jsonrpsee::{
	core::Serialize, IntoSubscriptionCloseResponse, SubscriptionCloseResponse, SubscriptionMessage,
};

pub mod chain_head;
pub mod chain_spec;
pub mod transaction;

/// Task executor that is being used by RPC subscriptions.
pub type SubscriptionTaskExecutor = std::sync::Arc<dyn sp_core::traits::SpawnNamed>;

/// ...
pub enum SubscriptionResponse<T> {
	/// The subscription was closed, no further message is sent.
	Closed,
	/// Send out a notification.
	Event(T),
}

impl<T: Serialize> IntoSubscriptionCloseResponse for SubscriptionResponse<T> {
	fn into_response(self) -> SubscriptionCloseResponse {
		match self {
			Self::Closed => SubscriptionCloseResponse::None,
			Self::Event(ev) => {
				let msg = SubscriptionMessage::from_json(&ev)
					.expect("JSON serialization infallible; qed");
				SubscriptionCloseResponse::Some(msg)
			},
		}
	}
}
