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

use std::collections::HashMap;

use crate::{core::get::GetObject, Get, Set};

use super::ParticipantIdentity;

impl ParticipantIdentity<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn send_to(&self) -> Option<&HashMap<String, String>> {
        self.send_to.as_ref()
    }

    pub fn is_default(&self) -> Option<bool> {
        self.is_default
    }
}

impl GetObject for ParticipantIdentity<Set> {
    type GetArguments = ();
}

impl GetObject for ParticipantIdentity<Get> {
    type GetArguments = ();
}
