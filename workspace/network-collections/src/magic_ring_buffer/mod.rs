// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


use super::*;
use ::dpdk_unix::page_size;
use ::dpdk_unix::android_linux::page_table::HasVirtualAddress;
use ::dpdk_unix::memory_information::PhysicalAddress;
use ::dpdk_unix::memory_information::VirtualAddress;
use ::libc::c_void;
use ::libc::close;
use ::libc::ftruncate;
use ::libc::mkstemps;
use ::libc::mlock;
use ::libc::mmap;
use ::libc::PROT_NONE;
use ::libc::PROT_READ;
use ::libc::PROT_WRITE;
use ::libc::MAP_ANONYMOUS;
use ::libc::MAP_FAILED;
use ::libc::MAP_FIXED;
use ::libc::MAP_NORESERVE;
use ::libc::MAP_PRIVATE;
use ::libc::MAP_SHARED;
use ::libc::munmap;
use ::libc::unlink;
use ::std::cell::RefCell;
use ::std::ffi::CString;
use ::std::fmt::Display;
use ::std::io;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Mul;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
use ::std::path::Path;
use ::std::rc::Rc;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;


include!("Bytes.rs");
include!("Failure.rs");
include!("FileDescriptor.rs");
include!("last_error.rs");
include!("MagicRingBuffer.rs");
include!("MagicRingBuffersArena.rs");
include!("MemoryMap.rs");
