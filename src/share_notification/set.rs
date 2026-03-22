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

//! ShareNotification is destroy-only — no create or update is permitted
//! (RFC 9670). Only `SetObject` is implemented, not `SetObjectCreatable`,
//! so `create()` and `update()` are unavailable at compile time.

use crate::{core::set::SetObject, Get, Set};

use super::ShareNotification;

impl SetObject for ShareNotification<Set> {
    type SetArguments = ();

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{id}"))
    }
}

impl SetObject for ShareNotification<Get> {
    type SetArguments = ();

    fn create_id(&self) -> Option<String> {
        None
    }
}
