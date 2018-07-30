// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn)]
#![feature(try_from)]


//! # network-internet-control-message-protocol
//!
//! A domain model of Internet Control Message Protocol (ICMP) packets and associated types.
//!
//! This crate has the optional feature `dpdk-sys`, which adds support for converting Into and From DPDK representations.
//!
//! It has an experimental feature `libc`, which does not compile as of libc 0.2.42 (libc is missing essential definitions).


#[cfg(feature = "dpdk-sys")] extern crate dpdk_sys;
#[cfg(feature = "libc")] extern crate libc;
extern crate network_endian;
extern crate network_ethernet;
extern crate network_internet_protocol;
extern crate serde;
#[macro_use] extern crate serde_derive;


#[cfg(feature = "dpdk-sys")] use dpdk_sys::*;
#[cfg(feature = "libc")] use libc::*;
use ::network_endian::*;
use ::network_ethernet::MaximumTransmissionUnitSize;
use ::network_internet_protocol::*;
use ::network_internet_protocol::version_4::*;
use ::std::cmp::Ordering;
use ::std::convert::TryFrom;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::marker::PhantomData;
#[cfg(any(feature = "dpdk-sys", feature = "libc"))] use ::std::mem::transmute;
#[cfg(any(feature = "dpdk-sys", feature = "libc"))] use ::std::ptr::NonNull;


include!("InternetControlMessageProtocolPacket.rs");
include!("InternetControlMessageProtocolPacketHeader.rs");
include!("InternetControlMessageProtocolPacketPayload.rs");
include!("InternetControlMessageProtocolType.rs");
include!("IdentAndSequence.rs");
include!("PathMaximumTransmissionUnit.rs");
include!("RestOfHeader.rs");
include!("RouterAdvertisement.rs");
