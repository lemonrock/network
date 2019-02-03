// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Also known as a Virtual Ring Buffer, a Virtual Circular Buffer or Mirrored Queue.
#[derive(Debug)]
pub struct MagicRingBuffer
{
	virtual_address: VirtualAddress,
	physical_address: PhysicalAddress,
	size: Bytes,
	write_offset: Bytes,
	read_offset: Bytes,
	arena: Rc<MagicRingBuffersArena>,
}

impl Drop for MagicRingBuffer
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.arena.deallocate((self.virtual_address, self.physical_address))
	}
}

impl MagicRingBuffer
{
	/// How many bytes have been used, ie how many bytes can be read?
	#[inline(always)]
	pub fn used(&self) -> Bytes
	{
		debug_assert!(self.write_offset >= self.read_offset, "read_offset '{}' exceeds write_offset '{}'", self.read_offset, self.write_offset);
		
		self.write_offset - self.read_offset
	}
	
	/// How many bytes are available, ie how many bytes can be written to?
	#[inline(always)]
	pub fn available(&self) -> Bytes
	{
		debug_assert!(self.used() <= self.size, "used '{}' exceeds size '{}'", self.used(), self.size);
		
		self.size - self.used()
	}
	
	/// A read buffer.
	#[inline(always)]
	pub fn read_buffer(&self) -> &[u8]
	{
		self.read_buffer_slice(self.used())
	}
	
	/// A read buffer slice.
	#[inline(always)]
	pub fn read_buffer_slice(&self, maximum: Bytes) -> &[u8]
	{
		debug_assert!(maximum <= self.used(), "maximum '{}' exceeds used '{}'", maximum, self.used());
		
		unsafe { from_raw_parts(self.read_address().into(), maximum.into()) }
	}
	
	/// Sadly, sometimes, one needs physical read buffers when working with DPDK with an IOMMU-less configuration.
	///
	/// This is the case for the DPDK kernel modules `uio` and `kni` (and maybe `igb_uio`).
	#[inline(always)]
	pub fn physical_read_buffers(&self) -> ((PhysicalAddress, Bytes), Option<(PhysicalAddress, Bytes)>)
	{
		self.physical_read_buffers_slice(self.used())
	}
	
	/// Sadly, sometimes, one needs physical read buffers when working with DPDK with an IOMMU-less configuration.
	///
	/// This is the case for the DPDK kernel modules `uio` and `kni` (and maybe `igb_uio`).
	#[inline(always)]
	pub fn physical_read_buffers_slice(&self, maximum: Bytes) -> ((PhysicalAddress, Bytes), Option<(PhysicalAddress, Bytes)>)
	{
		debug_assert!(maximum <= self.used(), "maximum '{}' exceeds used '{}'", maximum, self.used());
		
		// NOTE: `self.read_offset < self.size` is ALWAYS true because of (a) the logic in `read_commit()` and (b) no other code changes `self.read_offset`.
		
		if self.read_offset + maximum <= self.size
		{
			((self.read_physical_address(), maximum), None)
		}
		else
		{
			let amount_to_get_to_end_of_unmirrored_buffer = self.size - self.read_offset;
			let amount_over = maximum - amount_to_get_to_end_of_unmirrored_buffer;
			((self.start_physical_address(), amount_over), Some((self.read_physical_address(), amount_to_get_to_end_of_unmirrored_buffer)))
		}
	}
	
	/// Commit a read.
	#[inline(always)]
	pub fn read_commit(&mut self, read: Bytes)
	{
		debug_assert!(read <= self.used(), "read '{}' exceeds used '{}'", read, self.used());
		
		self.read_offset += read;
		
		if self.read_offset >= self.size
		{
			debug_assert!(self.write_offset >= self.size, "write_offset '{}' is less than size '{}'", self.write_offset, self.size);
			
			self.read_offset -= self.size;
			self.write_offset -= self.size;
		}
	}
	
	/// A write buffer.
	#[inline(always)]
	pub fn write_buffer(&self) -> &mut [u8]
	{
		unsafe { from_raw_parts_mut(self.write_address().into(), self.available().into()) }
	}
	
	/// Commit a write.
	#[inline(always)]
	pub fn write_commit(&mut self, wrote: Bytes)
	{
		debug_assert!(wrote <= self.available(), "wrote '{}' exceeds available '{}'", wrote, self.available());
		
		self.write_offset += wrote
	}
	
	#[inline(always)]
	fn read_address(&self) -> VirtualAddress
	{
		self.address(self.read_offset)
	}
	
	#[inline(always)]
	fn write_address(&self) -> VirtualAddress
	{
		self.address(self.write_offset)
	}
	
	#[inline(always)]
	fn start_physical_address(&self) -> PhysicalAddress
	{
		self.physical_address(Bytes::default())
	}
	
	#[inline(always)]
	fn read_physical_address(&self) -> PhysicalAddress
	{
		self.physical_address(self.read_offset)
	}
	
	#[inline(always)]
	fn address(&self, offset: Bytes) -> VirtualAddress
	{
		debug_assert!(offset <= self.size * 2, "offset '{}' exceeds self.size * 2 '{}'", offset, self.size * 2);
		
		self.virtual_address + offset.into()
	}
	
	#[inline(always)]
	fn physical_address(&self, offset: Bytes) -> PhysicalAddress
	{
		debug_assert!(offset <= self.size * 2, "offset '{}' exceeds self.size * 2 '{}'", offset, self.size * 2);
		
		let bytes: usize = offset.into();
		self.physical_address + bytes
	}
}
