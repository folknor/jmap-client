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

use crate::{core::get::GetObject, core::set::SetObject, Get, Set};

use super::Quota;

impl Quota<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn resource_type(&self) -> Option<&str> {
        self.resource_type.as_deref()
    }

    pub fn used(&self) -> Option<u64> {
        self.used
    }

    pub fn hard_limit(&self) -> Option<u64> {
        self.hard_limit
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn types(&self) -> Option<&[String]> {
        self.types.as_deref()
    }

    pub fn warn_limit(&self) -> Option<Option<u64>> {
        self.warn_limit
    }

    pub fn soft_limit(&self) -> Option<Option<u64>> {
        self.soft_limit
    }

    pub fn description(&self) -> Option<Option<&str>> {
        self.description.as_ref().map(|d| d.as_deref())
    }
}

impl GetObject for Quota<Set> {
    type GetArguments = ();
}

impl GetObject for Quota<Get> {
    type GetArguments = ();
}

/// Quota is read-only — SetObject is implemented only to satisfy trait
/// bounds required by the framework (GetResponse, ChangesResponse).
impl SetObject for Quota<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        Quota {
            _create_id,
            _state: Default::default(),
            id: None,
            resource_type: None,
            used: None,
            hard_limit: None,
            scope: None,
            name: None,
            types: None,
            warn_limit: None,
            soft_limit: None,
            description: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}

impl SetObject for Quota<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
