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

use serde::Serialize;

use crate::{
    core::query::{self, QueryObject},
    Set,
};

use super::CalendarEventNotification;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    Type {
        #[serde(rename = "type")]
        value: String,
    },
    CalendarEventId {
        #[serde(rename = "calendarEventId")]
        value: String,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "created")]
    Created,
}

impl Filter {
    pub fn type_(value: impl Into<String>) -> Self {
        Filter::Type {
            value: value.into(),
        }
    }

    pub fn calendar_event_id(value: impl Into<String>) -> Self {
        Filter::CalendarEventId {
            value: value.into(),
        }
    }
}

impl Comparator {
    pub fn created() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Created)
    }
}

impl QueryObject for CalendarEventNotification<Set> {
    type QueryArguments = ();
    type Filter = Filter;
    type Sort = Comparator;
}
