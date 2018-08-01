// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn)]
#![feature(reverse_bits)]
#![feature(try_from)]
#![feature(untagged_unions)]


//! # network-packet
//!
//! A domain model of network packets.


use ::std::fmt::Debug;
use ::std::ptr::NonNull;


include!("IncomingNetworkPacket.rs");
include!("IncomingNetworkPacketDropReason.rs");
include!("IncomingNetworkPacketDropObserver.rs");
