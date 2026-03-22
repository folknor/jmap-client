/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

pub mod get;
pub mod helpers;
pub mod query;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};

/// A quota object representing a storage or count limit (RFC 9425).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    #[serde(rename = "used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,

    #[serde(rename = "hardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_limit: Option<u64>,

    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,

    #[serde(rename = "warnLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_limit: Option<Option<u64>>,

    #[serde(rename = "softLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_limit: Option<Option<u64>>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "resourceType")]
    ResourceType,
    #[serde(rename = "used")]
    Used,
    #[serde(rename = "hardLimit")]
    HardLimit,
    #[serde(rename = "scope")]
    Scope,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "types")]
    Types,
    #[serde(rename = "warnLimit")]
    WarnLimit,
    #[serde(rename = "softLimit")]
    SoftLimit,
    #[serde(rename = "description")]
    Description,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::ResourceType => write!(f, "resourceType"),
            Property::Used => write!(f, "used"),
            Property::HardLimit => write!(f, "hardLimit"),
            Property::Scope => write!(f, "scope"),
            Property::Name => write!(f, "name"),
            Property::Types => write!(f, "types"),
            Property::WarnLimit => write!(f, "warnLimit"),
            Property::SoftLimit => write!(f, "softLimit"),
            Property::Description => write!(f, "description"),
        }
    }
}

impl Object for Quota<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Quota<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Quota<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for Quota<Get> {
    type ChangesResponse = ();
}
