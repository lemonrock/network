// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A trait to unify ArrayVec and Vec when dealing with NonNull pointers.
pub trait NonNullUnifiedArrayVecAndVec<T> : UnifiedArrayVecAndVec<NonNull<T>>
{
	/// To FFI data.
	///
	/// Returns a pointer and a length for FFI functions to read from.
	#[inline(always)]
	fn to_ffi_data(&mut self, start_from_index: usize) -> (*mut *mut T, usize)
	{
		let length = self.length();
		debug_assert!(length >= start_from_index, "length '{}' is less than start_from_index '{}'", length, start_from_index);
		
		let number_of_potential_packets = length - start_from_index;
		debug_assert_ne!(number_of_potential_packets, 0, "number_of_potential_packets is zero");
		
		let pointer: *mut *mut T = unsafe { transmute(self.mutable_pointer_at_length(start_from_index) as *mut NonNull<T>) };
		
		(pointer, number_of_potential_packets)
	}
	
	/// To FFI data.
	///
	/// Returns a pointer and a length (u16) for FFI functions to read from.
	#[inline(always)]
	fn to_ffi_data_u16(&mut self, start_from_index: usize) -> (*mut *mut T, u16)
	{
		let (pointer, number_of_potential_packets) = self.to_ffi_data(start_from_index);
		debug_assert!(number_of_potential_packets <= ::std::u16::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u16::MAX '{}'", number_of_potential_packets, ::std::u16::MAX);
		(pointer, number_of_potential_packets as u16)
	}
	
	/// To FFI data.
	///
	/// Returns a pointer and a length (u32) for FFI functions to read from.
	#[inline(always)]
	fn to_ffi_data_u32(&mut self, start_from_index: usize) -> (*mut *mut T, u32)
	{
		let (pointer, number_of_potential_packets) = self.to_ffi_data(start_from_index);
		debug_assert!(number_of_potential_packets <= ::std::u32::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u32::MAX '{}'", number_of_potential_packets, ::std::u32::MAX);
		(pointer, number_of_potential_packets as u32)
	}
	
	/// From FFI data.
	///
	/// Returns a pointer and a length for FFI functions to write to.
	#[inline(always)]
	fn from_ffi_data(&mut self) -> (*mut *mut T, usize)
	{
		let length = self.length();
		let maximum_capacity = self.maximum_capacity();
		debug_assert!(maximum_capacity >= length, "maximum_capacity '{}' is less than length '{}'", maximum_capacity, length);
		
		let number_of_potential_packets = maximum_capacity - length;
		debug_assert_ne!(number_of_potential_packets, 0, "number_of_potential_packets is zero");
		
		let pointer: *mut *mut T = unsafe { transmute(self.mutable_pointer_at_length(length) as *mut NonNull<T>) };
		
		(pointer, number_of_potential_packets)
	}
	
	/// From FFI data.
	///
	/// Returns a pointer and a length (u16) for FFI functions to write to.
	#[inline(always)]
	fn from_ffi_data_u16(&mut self) -> (*mut *mut T, u16)
	{
		let (pointer, number_of_potential_packets) = self.from_ffi_data();
		debug_assert!(number_of_potential_packets <= ::std::u16::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u16::MAX '{}'", number_of_potential_packets, ::std::u16::MAX);
		(pointer, number_of_potential_packets as u16)
	}
	
	/// From FFI data.
	///
	/// Returns a pointer and a length (u32) for FFI functions to write to.
	#[inline(always)]
	fn from_ffi_data_u32(&mut self) -> (*mut *mut T, u32)
	{
		let (pointer, number_of_potential_packets) = self.from_ffi_data();
		debug_assert!(number_of_potential_packets <= ::std::u32::MAX as usize, "number_of_potential_packets '{}' exceeds ::std::u32::MAX '{}'", number_of_potential_packets, ::std::u32::MAX);
		(pointer, number_of_potential_packets as u32)
	}
}

impl<T> NonNullUnifiedArrayVecAndVec<T> for Vec<NonNull<T>>
{
}
