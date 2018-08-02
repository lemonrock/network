// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn, core_intrinsics, repr128, try_from)]


//! # network-internet-protocol
//!
//! A domain model of internet protocol version 4 and version 6 host addresses, masks, network addresses and packets and associated types.
//!
//! This crate has the optional feature `dpdk-sys`, which adds support for converting Into and From DPDK representations.
//!
//! This crate also has the default features:-
//!
//! * `drop-packets-with-ipv4-options`: Drops internet protocol (IP) version 4 packets with IP options. Violates RFC 791 but IP options are very rarely used, the few that are used have potential to be used as attack vectors and none have any meaning to this library.
//! * `drop-ipv4-packets-with-do-not-fragment-and-non-zero-identification`: Drops internet protocol (IP) version 4 packets with the Do Not Fragment (DF) flag set and a non-zero (fragment) identification. Violates RFC 6864 Section 4.1 paragraph 5 but there is no good reason to send IP packets with DF set and a non-zero (fragment) identification. In particular, the identification field in these cases can be used as a covert channel and to infer the number of devices behind a NAT proxy.
//! * `drop-ipv6-packets-with-non-zero-flow-label`: Drop internet protocol (IP) version 6 packets whose flow label is not zero. There is no good reason to be receiving such packets for ICMP, TCP and UDP flows.
//! * `drop-ipv6-fragments-when-first-reserved-field-is-not-zero`: Drop internet protocol (IP) version 6 packets whose fragment extension header has a non-zero first reserved field (8-bits). Violates RFC 8200 Section 4.5 but there is no good reason to be receiving such packets.
//! * `drop-ipv6-fragments-when-second-reserved-field-is-not-zero`: Drop internet protocol (IP) version 6 packets whose fragment extension header has a non-zero second reserved field (2-bits). Violates RFC 8200 Section 4.5 but there is no good reason to be receiving such packets.
//! * `drop-overly-small-ipv6-fragments-aggresively`: We always drop fragments smaller than they need to be (444 byte MSS). This setting drops fragments (except the last) smaller than that possible using a 1280 byte MSS. Violates RFC 8200 Section 4.5 but there is no good reason to be receiving such packets.


#[macro_use] extern crate arrayref;
extern crate digest;
#[cfg(feature = "dpdk-sys")] extern crate dpdk_sys;
extern crate libc;
#[macro_use] extern crate likely;
extern crate network_check_sum;
extern crate network_endian;
extern crate network_ethernet;
extern crate network_packet;
extern crate serde;
#[macro_use] extern crate serde_derive;


/// Incoming network packet processing.
pub mod packet_processing;


/// Routing.
pub mod routing;


#[allow(dead_code)] mod treebitmap;


/// Internet Protocol (IP) version 4.
pub mod version_4;


/// Internet Protocol (IP) version 6.
pub mod version_6;


use self::packet_processing::*;
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
use ::network_packet::IncomingNetworkPacketProcessingDropReason;
use ::serde::Serialize;
use ::serde::Serializer;
use ::serde::de::DeserializeOwned;
use ::std::cmp::min;
use ::std::cmp::Ordering;
use ::std::collections::HashMap;
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
use ::treebitmap::address::Address as TreeBitmapAddress;
use ::treebitmap::tree_bitmap::TreeBitmap;


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
