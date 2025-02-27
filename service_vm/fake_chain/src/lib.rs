/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Provides functions to build a test chain for non-protected rialto and tests.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// `client_vm` builds DICE artifacts related to Microdroid, which is not relevant
// to the nostd build used in rialto.
#[cfg(feature = "std")]
pub mod client_vm;
pub mod service_vm;
