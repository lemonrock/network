// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]


//! # network-time
//!
//! Abstractions of millisecond durations and timestamps, and related ticks.


#[macro_use] extern crate likely;
extern crate serde;
#[macro_use] extern crate serde_derive;


use ::std::error;
use ::std::cmp::max;
use ::std::fmt;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::read;
use ::std::mem::uninitialized;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Div;
use ::std::ops::Mul;
use ::std::ops::Shl;
use ::std::ops::Sub;
use ::std::thread::sleep;
use ::std::time::Duration;


include!("MillisecondDuration.rs");
include!("MonotonicMillisecondTimestamp.rs");
include!("RetransmissionTimeOutData.rs");
include!("Tick.rs");
include!("TickDuration.rs");
