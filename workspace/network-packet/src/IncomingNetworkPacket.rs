// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents an incoming packet of contiguous data from a network card, with the Frame Check Sequence (FCS) (also known as Cyclic Redundancy Check, CRC) omitted.
pub trait IncomingNetworkPacket: Sized + Copy
{
	/// Optimized routine that only works on direct, contiguous packets with a reference count of 1.
	#[inline(always)]
	fn free_direct_contiguous_packet(self);
	
	/// Packet length if contiguous.
	///
	/// Same as `data_length()` for DPDK.
	#[inline(always)]
	fn packet_length_if_contiguous(self) -> u16;
	
	/// Offset into data, returned as a reference.
	///
	/// No length checks are made, although in debug builds an assertion may check.
	#[inline(always)]
	fn offset_into_data_reference<'a, T: 'a>(self, offset: usize) -> &'a T
	{
		unsafe { & * (self.offset_into_data::<T>(offset).as_ptr() as *const T) }
	}
	
	/// Offset into data, returned as a non-null pointer.
	///
	/// No length checks are made, although in debug builds an assertion may check.
	///
	/// A DPDK implementation would be a wrapper around `rte_pktmbuf_mtod` and `rte_pktmbuf_mtod_offset`.
	///
	/// Compare with DPDK's `io_virtual_address_offset()`.
	#[inline(always)]
	fn offset_into_data<T>(self, offset: usize) -> NonNull<T>;
}
