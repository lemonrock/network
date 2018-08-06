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


extern crate serde;
#[macro_use] extern crate serde_derive;


use ::std::fmt::Debug;
use ::std::ptr::NonNull;


include!("HardwareOffloadCategorisedInternetProtocolHeaderOptionsAndExtensionsLayer3PacketType.rs");
include!("HardwareOffloadCategorisedLayer2PacketType.rs");
include!("HardwareOffloadCheckSumStatus.rs");
include!("HardwareOffloadLayer2PacketType.rs");
include!("HardwareOffloadLayer3PacketType.rs");
include!("HardwareOffloadLayer4PacketType.rs");
include!("HardwareOffloadTunnelPacketType.rs");
include!("IncomingNetworkPacket.rs");
include!("IncomingNetworkPacketDropReason.rs");
include!("IncomingNetworkPacketDropObserver.rs");
