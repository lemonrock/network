// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An arena of Magic Ringe Buffers.
#[derive(Debug)]
pub struct MagicRingBuffersArena
{
	number_of_elements: Bytes,
	available: RefCell<Vec<(VirtualAddress, PhysicalAddress)>>,
}

impl Drop for MagicRingBuffersArena
{
	#[inline(always)]
	fn drop(&mut self)
	{
		for (virtual_address, _physical_address) in self.available.borrow().iter()
		{
			MemoryMap::wrap_and_drop(*virtual_address, self.number_of_elements)
		}
	}
}

impl MagicRingBuffersArena
{
	/// Creates a new instance.
	pub fn new(number_of_magic_ring_buffers: usize, number_of_elements: usize) -> Result<Rc<Self>, io::Error>
	{
		debug_assert_ne!(number_of_magic_ring_buffers, 0, "number_of_magic_ring_buffers can not be zero");
		
		debug_assert_ne!(number_of_elements, 0, "number_of_elements can not be zero");
		
		let number_of_elements = number_of_elements.into();
		
		let mut memory_maps = Vec::with_capacity(number_of_magic_ring_buffers);
		for _ in 0 .. number_of_magic_ring_buffers
		{
			memory_maps.push(MemoryMap::allocate_mirrored_and_not_swappable_from_dev_shm("magic-ring-buffer", number_of_elements)?)
		}
		
		let this = Rc::new
		(
			Self
			{
				number_of_elements,
				available: RefCell::new(Vec::with_capacity(number_of_magic_ring_buffers)),
			}
		);
		
		VirtualAddress::virtual_addresses_to_physical_addresses(memory_maps.iter_mut(), |memory_map, virtual_address, physical_address|
		{
			let mut borrowed = this.available.borrow_mut();
			memory_map.do_not_unmap_on_drop();
			borrowed.push((virtual_address, physical_address))
		});
		Self::avoid_expensive_drop_which_is_no_longer_needed(memory_maps);
		
		Ok(this)
	}
	
	#[inline(always)]
	fn avoid_expensive_drop_which_is_no_longer_needed(mut memory_maps: Vec<MemoryMap>)
	{
		unsafe { memory_maps.set_len(0) };
		drop(memory_maps)
	}
	
	/// Allocate (unchecked) a magic ring buffer from the arena.
	#[inline(always)]
	pub fn allocate(this: &Rc<Self>) -> MagicRingBuffer
	{
		let (virtual_address, physical_address) = this.available.borrow_mut().pop().unwrap();
		
		MagicRingBuffer
		{
			virtual_address,
			physical_address,
			size: this.number_of_elements,
			write_offset: Bytes::default(),
			read_offset: Bytes::default(),
			arena: this.clone(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn deallocate(&self, addresses: (VirtualAddress, PhysicalAddress))
	{
		self.available.borrow_mut().push(addresses)
	}
}
