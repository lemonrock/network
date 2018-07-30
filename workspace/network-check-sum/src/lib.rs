// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn)]


//! # RFC 1141 / RFC 1071 check sums, psuedo-headers and cryptographic authentication.
//!


extern crate digest;
extern crate network_endian;
extern crate serde;
#[macro_use] extern crate serde_derive;


pub use ::digest::Digest;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::size_of;
use ::std::mem::zeroed;
use ::std::ptr::NonNull;
use ::network_endian::*;


include!("InternetCheckSum.rs");
include!("InternetProtocolVersion4PseudoHeader.rs");
include!("InternetProtocolVersion6PseudoHeader.rs");
include!("Layer4ProtocolNumber.rs");
include!("Rfc1141CompliantCheckSum.rs");
