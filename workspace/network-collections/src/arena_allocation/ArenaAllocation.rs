// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An arena allocation.
#[derive(Debug)]
pub struct ArenaAllocation<T: Sized>
{
	number_of_entries: usize,
	
	/// Is equal to number_of_entries when full.
	next_available_slot_index: usize,
	
	allocation: NonNull<Entry<T>>,
}

impl<T: Sized> Drop for ArenaAllocation<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.allocation.iterate(self.number_of_entries, |entry_pointer| unsafe { entry_pointer.drop_in_place() });
		
		self.allocation.deallocate(self.number_of_entries)
	}
}

impl<T: Sized> ArenaAllocation<T>
{
	/// Is full?
	#[inline(always)]
	pub fn is_full(&self) -> bool
	{
		self.next_available_slot_index == self.number_of_entries
	}
	
	/// Create a new instance for `number_of_entries`.
	#[inline(always)]
	pub fn new(number_of_entries: usize) -> Self
	{
		assert_ne!(number_of_entries, ::std::usize::MAX, "number_of_entries can not be ::std::usize::MAX");
		
		let allocation = NonNull::<Entry<T>>::allocate(number_of_entries);
		allocation.initialize(number_of_entries);
		
		Self
		{
			number_of_entries,
			next_available_slot_index: 0,
			allocation,
		}
	}
	
	/// Allocate unchecked.
	#[inline(always)]
	pub fn allocate_unchecked(&mut self, value: T) -> &mut T
	{
		debug_assert!(!self.is_full(), "full");
		
		let entry = self.allocation.entry_mutable_reference(self.next_available_slot_index);
		
		entry.set_value_and_return_mutable_reference_to_it(value)
	}
	
	/// Free unchecked.
	#[inline(always)]
	pub fn free_unchecked(&mut self, entry_value: NonNull<T>)
	{
		// NOTE: Works because entry.entry_value is the FIRST field in the Entry struct.
		let entry_pointer_usize = entry_value.as_ptr() as usize;
		debug_assert!(self.allocation.pointer_usize_is_one_of_ours(entry_pointer_usize, self.number_of_entries), "entry_value was not from this ArenaAllocation");
		
		let entry = unsafe { &mut * (entry_pointer_usize as *mut Entry<T>) };
		entry.reuse(self.next_available_slot_index);
		self.next_available_slot_index = self.allocation.pointer_usize_to_index(entry_pointer_usize)
	}
}
