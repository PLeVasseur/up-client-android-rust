/********************************************************************************
 * Copyright (c) 2024 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

#[macro_use]
extern crate log;

mod aidl;
mod binder_impl;

pub mod binder_impls {
    pub use crate::binder_impl::{IUBus, IUListener};
}
pub mod parcelable_stubs {
    pub use crate::aidl::parcelable_stubs::*;
}
