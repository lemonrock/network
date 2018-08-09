// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[derive(Debug)]
#[repr(C)]
struct Entry<T>
{
	// This field MUST be the first one, so that the deallocation (free) code works.
	entry_value: EntryValue<T>,
	is_occupied: bool,
}

impl<T> Drop for Entry<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.is_occupied
		{
			self.entry_value.drop_value()
		}
	}
}

impl<T> Entry<T>
{
	#[inline(always)]
	const fn new(our_index: usize) -> Self
	{
		Self
		{
			is_occupied: false,
			entry_value: EntryValue
			{
				next_available_slot_index: our_index + 1,
			}
		}
	}
	
	#[inline(always)]
	fn set_value_and_return_mutable_reference_to_it(&mut self, value: T) -> &mut T
	{
		debug_assert!(!self.is_occupied, "entry is occupied");
		
		self.is_occupied = true;
		
		self.entry_value.set_value_and_return_mutable_reference_to_it(value)
	}
	
	#[inline(always)]
	fn reuse(&mut self, next_available_slot_index: usize)
	{
		debug_assert!(self.is_occupied, "entry is unoccupied");
		
		unsafe { (self as *mut Self).drop_in_place() }
		
		self.is_occupied = false;
		
		self.entry_value.set_next_available_slot_index(next_available_slot_index)
	}
}
