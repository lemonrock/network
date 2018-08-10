// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(allocator_api, const_fn, core_intrinsics, specialization, untagged_unions)]


//! # network-collections
//!
//! Collections suitable for use with networking, particularly when a fixed memory usage is required to prevent out-of-memory crashes. Includes two kinds of least recently used caches, a bounded hash map, a magic ring buffer (virtual ring buffer) and an arena allocator.


extern crate arrayvec;
#[cfg(any(os = "android", os = "linux"))] extern crate dpdk_unix;
extern crate libc;
#[macro_use] extern crate likely;
extern crate network_time;
extern crate serde;
#[macro_use] extern crate serde_derive;


include!("array_n.rs");


/// An efficient arena allocator.
pub mod arena_allocation;


/// Least Recently Used caches.
pub mod least_recently_used_cache;


/// Magic ring buffers.
#[cfg(any(os = "android", os = "linux"))] pub mod magic_ring_buffer;


pub use ::arrayvec::ArrayVec;
use ::network_time::MillisecondDuration;
use ::network_time::MonotonicMillisecondTimestamp;
use ::serde::Deserialize;
use ::serde::Deserializer;
use ::serde::Serialize;
use ::serde::Serializer;
use ::serde::de::Error as DeserializeError;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
use ::serde::ser::SerializeTuple;
use ::std::collections::HashMap;
use ::std::cmp::Ordering;
use ::std::cmp::Eq;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::align_of;
use ::std::mem::ManuallyDrop;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::marker::PhantomData;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
#[allow(unused_imports)] use ::std::os::unix::ffi::OsStrExt;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::slice::Iter;
use ::std::slice::IterMut;


include!("Array40.rs");
include!("Array52.rs");
include!("BoundedHashMap.rs");
include!("NonNullUnifiedArrayVecAndVec.rs");
include!("UnifiedArrayVecAndVec.rs");
