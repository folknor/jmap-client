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

use ahash::AHashMap;

use crate::{core::set::SetObject, Get, Set};

use super::ParticipantIdentity;

impl ParticipantIdentity<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn send_to(&mut self, send_to: AHashMap<String, String>) -> &mut Self {
        self.send_to = Some(send_to);
        self
    }

    pub fn is_default(&mut self, is_default: bool) -> &mut Self {
        self.is_default = Some(is_default);
        self
    }
}

impl SetObject for ParticipantIdentity<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        ParticipantIdentity {
            _create_id,
            _state: Default::default(),
            id: None,
            name: None,
            send_to: AHashMap::new().into(),
            is_default: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for ParticipantIdentity<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
