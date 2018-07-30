// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(const_fn)]


//! # network-internet-control-message-protocol
//!
//! A domain model of Internet Control Message Protocol (ICMP) packets and associated types.


#[cfg(feature = "dpdk-sys")] extern crate dpdk_sys;
extern crate network_endian;
extern crate network_internet_protocol;
extern crate serde;
#[macro_use] extern crate serde_derive;


#[cfg(feature = "dpdk-sys")] use dpdk_sys::*;
use ::network_endian::*;
use ::network_internet_protocol::*;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
#[cfg(feature = "dpdk-sys")] use ::std::mem::transmute;
#[cfg(feature = "dpdk-sys")] use ::std::ptr::NonNull;


include!("InternetControlMessageProtocolPacket.rs");
include!("InternetControlMessageProtocolPacketHeader.rs");
include!("InternetControlMessageProtocolPacketPayload.rs");
include!("InternetControlMessageProtocolType.rs");
include!("RestOfHeader.rs");
