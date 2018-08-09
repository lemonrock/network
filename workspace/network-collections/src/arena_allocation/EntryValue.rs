// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[repr(C)]
union EntryValue<T>
{
	next_available_slot_index: usize,
	value: ManuallyDrop<T>,
}

impl<T> Debug for EntryValue<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		write!(f, "EntryValue()")
	}
}

impl<T> EntryValue<T>
{
	#[inline(always)]
	fn set_value_and_return_mutable_reference_to_it(&mut self, value: T) -> &mut T
	{
		unsafe
		{
			self.value = ManuallyDrop::new(value);
			&mut self.value
		}
	}
	
	#[inline(always)]
	fn drop_value(&mut self)
	{
		unsafe { ManuallyDrop::drop(&mut self.value) }
	}
	
	#[inline(always)]
	fn set_next_available_slot_index(&mut self, next_available_slot_index: usize)
	{
		self.next_available_slot_index = next_available_slot_index
	}
}
