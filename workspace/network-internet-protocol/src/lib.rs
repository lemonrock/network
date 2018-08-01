// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(repr128)]
#![feature(try_from)]


//! # network-internet-protocol
//!
//! A domain model of internet protocol version 4 and version 6 host addresses, masks, network addresses and packets and associated types.
//!
//! This crate has the optional feature `dpdk-sys`, which adds support for converting Into and From DPDK representations.


#[macro_use] extern crate arrayref;
extern crate digest;
#[cfg(feature = "dpdk-sys")] extern crate dpdk_sys;
extern crate libc;
extern crate network_check_sum;
extern crate network_endian;
extern crate network_ethernet;
extern crate serde;
#[macro_use] extern crate serde_derive;


include!("arp_unsupported.rs");


/// Incoming network packet processing.
pub mod packet_processing;


/// Internet Protocol (IP) version 4.
pub mod version_4;


/// Internet Protocol (IP) version 6.
pub mod version_6;


use self::version_4::*;
use self::version_6::*;
pub use ::digest::Digest;
#[cfg(feature = "dpdk-sys")]  use ::dpdk_sys::*;
use ::libc::*;
use ::network_check_sum::*;
use ::network_endian::*;
use ::network_ethernet::*;
use ::network_ethernet::virtual_lans::DifferentiatedServiceCodePoint;
use ::network_ethernet::packet_processing::EthernetIncomingNetworkPacket;
use ::network_ethernet::packet_processing::EthernetIncomingNetworkPacketDropObserver;
use ::network_ethernet::packet_processing::EthernetIncomingNetworkPacketDropReason;
use ::network_ethernet::packet_processing::Layer3PacketProcessing;
use ::serde::Serialize;
use ::std::cmp::Ordering;
use ::std::collections::HashSet;
use ::std::convert::TryFrom;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::transmute_copy;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::net::Ipv4Addr;
use ::std::net::Ipv6Addr;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::rc::Rc;


include!("ExplicitCongestionNotification.rs");
include!("InternetHeaderLength.rs");
include!("InternetProtocolHostAddress.rs");
include!("InternetProtocolMaskBits.rs");
include!("InternetProtocolNetworkAddress.rs");
include!("InternetProtocolVersion4OrVersion6OrBoth.rs");
include!("Layer4Packet.rs");
include!("Layer4ProtocolNeedsToSupport.rs");
include!("Layer4ProtocolNumber.rs");
include!("MediaAccessControlAddressAndInternetProtocolAddressOverlap.rs");
include!("TrafficClass.rs");
