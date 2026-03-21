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
use crate::core::set::string_not_set;
use crate::core::Object;
use crate::{Get, Set};

use crate::calendar_event::Alert;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "string_not_set")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,

    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<u32>,

    #[serde(rename = "defaultAlertsWithTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_alerts_with_time: Option<Option<AHashMap<String, Alert>>>,

    #[serde(rename = "defaultAlertsWithoutTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_alerts_without_time: Option<Option<AHashMap<String, Alert>>>,

    #[serde(rename = "isSubscribed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,

    #[serde(rename = "isVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Option<String>>,

    #[serde(rename = "shareWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_with: Option<Option<AHashMap<String, ShareRights>>>,

    #[serde(rename = "myRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_rights: Option<ShareRights>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareRights {
    #[serde(rename = "mayReadFreeBusy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_read_free_busy: Option<bool>,

    #[serde(rename = "mayReadItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_read_items: Option<bool>,

    #[serde(rename = "mayAddItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_add_items: Option<bool>,

    #[serde(rename = "mayUpdatePrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_update_private: Option<bool>,

    #[serde(rename = "mayRSVP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_rsvp: Option<bool>,

    #[serde(rename = "mayUpdateOwn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_update_own: Option<bool>,

    #[serde(rename = "mayUpdateAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_update_all: Option<bool>,

    #[serde(rename = "mayRemoveOwn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_remove_own: Option<bool>,

    #[serde(rename = "mayRemoveAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_remove_all: Option<bool>,

    #[serde(rename = "mayAdmin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_admin: Option<bool>,

    #[serde(rename = "mayDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_delete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "sortOrder")]
    SortOrder,
    #[serde(rename = "defaultAlertsWithTime")]
    DefaultAlertsWithTime,
    #[serde(rename = "defaultAlertsWithoutTime")]
    DefaultAlertsWithoutTime,
    #[serde(rename = "isSubscribed")]
    IsSubscribed,
    #[serde(rename = "isVisible")]
    IsVisible,
    #[serde(rename = "timeZone")]
    TimeZone,
    #[serde(rename = "shareWith")]
    ShareWith,
    #[serde(rename = "myRights")]
    MyRights,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Name => write!(f, "name"),
            Property::Description => write!(f, "description"),
            Property::Color => write!(f, "color"),
            Property::SortOrder => write!(f, "sortOrder"),
            Property::DefaultAlertsWithTime => write!(f, "defaultAlertsWithTime"),
            Property::DefaultAlertsWithoutTime => write!(f, "defaultAlertsWithoutTime"),
            Property::IsSubscribed => write!(f, "isSubscribed"),
            Property::IsVisible => write!(f, "isVisible"),
            Property::TimeZone => write!(f, "timeZone"),
            Property::ShareWith => write!(f, "shareWith"),
            Property::MyRights => write!(f, "myRights"),
        }
    }
}

impl Object for Calendar<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Calendar<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Calendar<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for Calendar<Get> {
    type ChangesResponse = ();
}
