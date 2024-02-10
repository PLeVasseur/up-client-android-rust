/*
 * Copyright (c) 2024 General Motors GTO LLC
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 * SPDX-FileType: SOURCE
 * SPDX-FileCopyrightText: 2023 General Motors GTO LLC
 * SPDX-License-Identifier: Apache-2.0
 */

use binder::{
    binder_impl::{BorrowedParcel, UnstructuredParcelable},
    impl_deserialize_for_unstructured_parcelable, impl_serialize_for_unstructured_parcelable,
    StatusCode,
};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "use_proto")]
use protobuf::Message;

#[cfg(feature = "use_proto")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParcelableUMessage(up_rust::uprotocol::UMessage);

#[cfg(feature = "use_proto")]
impl From<up_rust::uprotocol::UMessage> for ParcelableUMessage{
    fn from(item: up_rust::uprotocol::UMessage) -> Self {
        ParcelableUMessage(item)
    }
}

#[cfg(feature = "use_proto")]
impl Deref for ParcelableUMessage {
    type Target = up_rust::uprotocol::UMessage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "use_proto")]
impl DerefMut for ParcelableUMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(not(feature = "use_proto"))]
pub struct ParcelableUMessage;

/// Rust `UnstructuredParcelable` stub for `ParcelableUMessage`
/// True implementation is down below
/// This exists just to allow the Android aidl_interface build process to succeed
#[cfg(not(feature = "use_proto"))]
impl UnstructuredParcelable for ParcelableUMessage {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel) -> Result<(), StatusCode> {
        Ok(())
    }

    fn from_parcel(parcel: &BorrowedParcel) -> Result<Self, StatusCode> {
        Ok(Self {})
    }
}

/// Rust `UnstructuredParcelable` implementation for `ParcelableUMessage` using Protobuf serialization/deserialization
#[cfg(feature = "use_proto")]
impl UnstructuredParcelable for ParcelableUMessage {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel) -> Result<(), StatusCode> {
        Ok(())
    }

    fn from_parcel(parcel: &BorrowedParcel) -> Result<Self, StatusCode> {
        Ok( Self::default() )
    }
}

impl_deserialize_for_unstructured_parcelable!(ParcelableUMessage);
impl_serialize_for_unstructured_parcelable!(ParcelableUMessage);
