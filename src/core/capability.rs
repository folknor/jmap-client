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

/// A JMAP capability identified by its URN.
pub trait Capability {
    const URI: &'static str;
}

pub struct Core;
impl Capability for Core {
    const URI: &'static str = "urn:ietf:params:jmap:core";
}

pub struct Mail;
impl Capability for Mail {
    const URI: &'static str = "urn:ietf:params:jmap:mail";
}

pub struct Submission;
impl Capability for Submission {
    const URI: &'static str = "urn:ietf:params:jmap:submission";
}

pub struct VacationResponseCap;
impl Capability for VacationResponseCap {
    const URI: &'static str = "urn:ietf:params:jmap:vacationresponse";
}

pub struct Contacts;
impl Capability for Contacts {
    const URI: &'static str = "urn:ietf:params:jmap:contacts";
}

pub struct ContactsParse;
impl Capability for ContactsParse {
    const URI: &'static str = "urn:ietf:params:jmap:contacts:parse";
}

pub struct Calendars;
impl Capability for Calendars {
    const URI: &'static str = "urn:ietf:params:jmap:calendars";
}

pub struct CalendarsParse;
impl Capability for CalendarsParse {
    const URI: &'static str = "urn:ietf:params:jmap:calendars:parse";
}

pub struct Blob;
impl Capability for Blob {
    const URI: &'static str = "urn:ietf:params:jmap:blob";
}

pub struct Quota;
impl Capability for Quota {
    const URI: &'static str = "urn:ietf:params:jmap:quota";
}

pub struct WebSocket;
impl Capability for WebSocket {
    const URI: &'static str = "urn:ietf:params:jmap:websocket";
}

pub struct Sieve;
impl Capability for Sieve {
    const URI: &'static str = "urn:ietf:params:jmap:sieve";
}

pub struct Principals;
impl Capability for Principals {
    const URI: &'static str = "urn:ietf:params:jmap:principals";
}

pub struct PrincipalsOwner;
impl Capability for PrincipalsOwner {
    const URI: &'static str = "urn:ietf:params:jmap:principals:owner";
}
