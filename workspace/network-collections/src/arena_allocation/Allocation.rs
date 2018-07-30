// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


trait Allocation<T>: Sized + Copy
{
	#[inline(always)]
	fn allocate(number_of_entries: usize) -> NonNull<Entry<T>>
	{
		let system = System;
		let allocation = unsafe { system.alloc(Self::layout(number_of_entries)) };
		if allocation.is_null()
		{
			panic!("Could not allocate enough memory")
		}
		unsafe { NonNull::new_unchecked(allocation as *mut Entry<T>) }
	}
	
	#[inline(always)]
	fn initialize(self, number_of_entries: usize)
	{
		let mut index = 0;
		self.iterate(number_of_entries, |entry_pointer|
		{
			unsafe { entry_pointer.write(Entry::new(index)) };
			index += 1;
		})
	}
	
	#[inline(always)]
	fn iterate<F: FnMut(*mut Entry<T>)>(self, number_of_entries: usize, mut callback: F)
	{
		let mut entry_pointer_usize = self.inclusive_start_pointer_usize();
		let exclusive_end_pointer = self.exclusive_end_pointer_usize(number_of_entries);
		
		while entry_pointer_usize != exclusive_end_pointer
		{
			callback(entry_pointer_usize as *mut Entry<T>);
			
			entry_pointer_usize += Self::entry_size()
		}
	}
	
	#[inline(always)]
	fn deallocate(self, number_of_entries: usize)
	{
		let system = System;
		unsafe { system.dealloc(self.inclusive_start_pointer() as *mut u8, Self::layout(number_of_entries)) }
	}
	
	#[inline(always)]
	fn layout(number_of_entries: usize) -> Layout
	{
		Layout::from_size_align(Self::size_in_bytes(number_of_entries), Self::entry_alignment()).unwrap()
	}
	
	#[inline(always)]
	fn entry_mutable_reference<'a>(self, index: usize) -> &'a mut Entry<T>
	{
		unsafe { &mut * self.entry_mutable_pointer(index) }
	}
	
	#[inline(always)]
	fn entry_mutable_pointer(self, index: usize) -> *mut Entry<T>
	{
		(self.inclusive_start_pointer_usize() + index * Self::entry_size()) as *mut Entry<T>
	}
	
	#[inline(always)]
	fn exclusive_end_pointer_usize(self, number_of_entries: usize) -> usize
	{
		self.inclusive_start_pointer_usize() + Self::size_in_bytes(number_of_entries)
	}
	
	#[inline(always)]
	fn pointer_usize_is_one_of_ours(self, pointer_usize: usize, number_of_entries: usize) -> bool
	{
		self.inclusive_start_pointer_usize() <= pointer_usize && pointer_usize < self.exclusive_end_pointer_usize(number_of_entries)
	}
	
	#[inline(always)]
	fn pointer_usize_to_index(self, pointer_usize: usize) -> usize
	{
		(pointer_usize - self.inclusive_start_pointer_usize()) / Self::entry_size()
	}
	
	#[inline(always)]
	fn size_in_bytes(number_of_entries: usize) -> usize
	{
		Self::entry_size() * number_of_entries
	}
	
	#[inline(always)]
	fn entry_size() -> usize
	{
		size_of::<Entry<T>>()
	}
	
	#[inline(always)]
	fn entry_alignment() -> usize
	{
		align_of::<Entry<T>>()
	}
	
	#[inline(always)]
	fn inclusive_start_pointer_usize(self) -> usize
	{
		self.inclusive_start_pointer() as usize
	}
	
	#[inline(always)]
	fn inclusive_start_pointer(self) -> *mut Entry<T>;
}

impl<T> Allocation<T> for NonNull<Entry<T>>
{
	#[inline(always)]
	fn inclusive_start_pointer(self) -> *mut Entry<T>
	{
		self.as_ptr()
	}
}
