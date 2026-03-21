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
pub mod set;

use std::fmt::Display;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::core::changes::ChangesObject;
use crate::core::set::map_not_set;
use crate::core::Object;
use crate::{Get, Set};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantIdentity<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "sendTo")]
    #[serde(skip_serializing_if = "map_not_set")]
    pub send_to: Option<AHashMap<String, String>>,

    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "sendTo")]
    SendTo,
    #[serde(rename = "isDefault")]
    IsDefault,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Name => write!(f, "name"),
            Property::SendTo => write!(f, "sendTo"),
            Property::IsDefault => write!(f, "isDefault"),
        }
    }
}

impl Object for ParticipantIdentity<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for ParticipantIdentity<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for ParticipantIdentity<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for ParticipantIdentity<Get> {
    type ChangesResponse = ();
}
