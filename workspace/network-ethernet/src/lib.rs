// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn)]
#![feature(reverse_bits)]
#![feature(try_from)]
#![feature(untagged_unions)]


//! # network-ethernet
//!
//! A domain model of ethernet and virtual LAN packets and associated types.
//!
//! This crate has the optional feature `dpdk-sys`, which adds support for converting Into and From DPDK representations.
//!
//! It has an experimental feature `libc`, which does not compile as of libc 0.2.42 (libc is missing essential definitions).


#[macro_use] extern crate arrayref;
#[cfg(feature = "dpdk-sys")] extern crate dpdk_sys;
extern crate hyper_thread_random;
#[cfg(feature = "libc")] extern crate libc;
extern crate network_endian;
extern crate serde;
#[macro_use] extern crate serde_derive;


use self::virtual_lans::*;
#[cfg(feature = "dpdk-sys")] use ::dpdk_sys::*;
use ::hyper_thread_random::generate_hyper_thread_safe_random_u64;
use ::network_endian::NetworkEndian;
use ::network_endian::NetworkEndianU16;
use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::Serialize;
use ::serde::Serializer;
use ::serde::de;
use ::serde::de::Visitor;
use ::std::collections::HashSet;
use ::std::cmp::Ordering;
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
use ::std::mem::uninitialized;
use ::std::ptr::copy_nonoverlapping;
#[cfg(feature = "dpdk-sys")] use ::std::ptr::NonNull;
use ::std::str::SplitN;


/// Virtual LANs (VLANs).
pub mod virtual_lans;


include!("EthernetAddresses.rs");
include!("EthernetDestination.rs");
include!("EthernetFrameLength.rs");
include!("EthernetPacket.rs");
include!("EthernetPacketHeader.rs");
include!("EthernetPacketPayload.rs");
include!("EtherType.rs");
include!("EtherTypeOrLegacyEthernetFrameSize.rs");
include!("Layer3Packet.rs");
include!("LegacyEthernetFrameSize.rs");
include!("MaximumTransmissionUnitSize.rs");
include!("MediaAccessControlAddress.rs");
include!("MediaAccessControlAddressList.rs");
include!("OrganizationallyUniqueIdentifier.rs");
include!("SizeU16OfEthernetCyclicRedundancyCheck.rs");
