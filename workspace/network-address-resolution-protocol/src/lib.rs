// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn, core_intrinsics, untagged_unions)]


//! # network-internet-control-message-protocol
//!
//! A domain model of Address Resolution Protocol (ARP) packets and associated types.
//!
//! This crate has the optional feature `dpdk-sys`, which adds support for converting Into and From DPDK representations.


#[cfg(feature = "dpdk-sys")] extern crate dpdk_sys;
#[macro_use] extern crate likely;
extern crate network_endian;
extern crate network_ethernet;
extern crate network_internet_protocol;
extern crate serde;
#[macro_use] extern crate serde_derive;


use self::AddressResolutionProtocolIncomingNetworkPacketDropReason::*;
#[cfg(feature = "dpdk-sys")] use dpdk_sys::*;
#[cfg(feature = "libc")] use libc::*;
use ::network_endian::*;
use ::network_ethernet::EthernetAddresses;
use ::network_ethernet::EtherType;
use ::network_ethernet::packet_processing::EthernetIncomingNetworkPacket;
use ::network_ethernet::packet_processing::EthernetIncomingNetworkPacketDropReason;
use ::network_ethernet::MediaAccessControlAddress;
use ::network_internet_protocol::*;
use ::network_internet_protocol::packet_processing::Layer3PacketProcessingImpl;
use ::network_internet_protocol::version_4::*;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
#[cfg(feature = "dpdk-sys")] use ::std::mem::transmute;
#[cfg(feature = "dpdk-sys")] use ::std::ptr::NonNull;


include!("drop.rs");


//include!("AddressResolutionProtocolAddressConflictState.rs");
include!("AddressResolutionProtocolIncomingNetworkPacketDropReason.rs");
include!("AddressResolutionProtocolPacket.rs");
include!("AddressResolutionProtocolPacketHeader.rs");
include!("AddressResolutionProtocolPacketInternetProtocolVersion4Payload.rs");
include!("AddressResolutionProtocolPacketPayload.rs");
include!("HardwareType.rs");
include!("Operation.rs");
