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

#![no_std]
#![no_main]

use common::u256_bytes;
use uapi::{HostFn, HostFnImpl as api};

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {}

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
	let ret_code = match api::call(
		uapi::CallFlags::empty(),
		&[0u8; 20],
		0,
		0,
		None,
		&u256_bytes(100u64),
		&[],
		None,
	) {
		Ok(_) => 0u32,
		Err(code) => code as u32,
	};

	// Exit with success and take transfer return code to the output buffer.
	api::return_value(uapi::ReturnFlags::empty(), &ret_code.to_le_bytes());
}