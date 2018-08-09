// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct MemoryMap
{
	address: *mut c_void,
	length: Bytes,
	should_unmap_on_drop: bool,
}

impl Drop for MemoryMap
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.should_unmap_on_drop
		{
			unsafe { munmap(self.address, self.length.into()) };
		}
	}
}

impl<'a> HasVirtualAddress for &'a mut MemoryMap
{
	#[inline(always)]
	fn virtual_address(&self) -> VirtualAddress
	{
		self.address.into()
	}
}

impl MemoryMap
{
	#[inline(always)]
	pub(crate) fn allocate_mirrored_and_not_swappable_from_dev_shm(file_extension: &str, buffer_size_not_page_aligned: Bytes) -> Result<Self, io::Error>
	{
		Self::allocate_mirrored_and_not_swappable("/dev/shm", file_extension, buffer_size_not_page_aligned)
	}
	
	pub(crate) fn allocate_mirrored_and_not_swappable(temporary_directory_path: impl AsRef<Path>, file_extension: &str, buffer_size_not_page_aligned: Bytes) -> Result<Self, io::Error>
	{
		debug_assert_ne!(file_extension.len(), 0, "file_extension can not be empty");
		debug_assert_ne!(buffer_size_not_page_aligned, Bytes::default(), "buffer_size_not_page_aligned can not be zero");
		
		let temporary_file = FileDescriptor::create_temporary_file_and_remove_it(temporary_directory_path, file_extension)?;
		temporary_file.truncate(buffer_size_not_page_aligned)?;
		
		let buffer_size = buffer_size_not_page_aligned.round_up_to_page_size();
		let mirror_length = buffer_size * 2;
		
		let address = Self::memory_map(null_mut(), mirror_length, PROT_NONE, MAP_NORESERVE | MAP_ANONYMOUS | MAP_PRIVATE, None, 0)?;
		let this = Self
		{
			address,
			length: mirror_length,
			should_unmap_on_drop: true,
		};
		debug_assert!(!address.is_null(), "Memory mapping address is null");
		
		let address_of_real_memory = Self::memory_map(address, buffer_size, PROT_READ | PROT_WRITE, MAP_NORESERVE | MAP_FIXED | MAP_SHARED, Some(&temporary_file), 0)?;
		debug_assert_eq!(address, address_of_real_memory, "First fixed mapping failed");
		
		let address_of_mirrored_memory = Self::memory_map(Self::mirror_address(address, buffer_size), buffer_size, PROT_READ | PROT_WRITE, MAP_NORESERVE | MAP_FIXED | MAP_SHARED, Some(&temporary_file), 0)?;
		debug_assert_eq!(address, address_of_mirrored_memory, "Second fixed mapping failed");
		
		temporary_file.close()?;
		
		Self::memory_lock(address, mirror_length)?;
		
		Ok(this)
	}
	
	#[inline(always)]
	pub(crate) fn wrap_and_drop(virtual_address: VirtualAddress, buffer_size_not_page_aligned: Bytes)
	{
		debug_assert_ne!(buffer_size_not_page_aligned, Bytes::default(), "buffer_size_not_page_aligned can not be zero");
		let buffer_size = buffer_size_not_page_aligned.round_up_to_page_size();
		let mirror_length = buffer_size * 2;
		drop
		(
			Self
			{
				address: virtual_address.into(),
				length: mirror_length,
				should_unmap_on_drop: true,
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn do_not_unmap_on_drop(&mut self)
	{
		self.should_unmap_on_drop = false
	}
	
	#[inline(always)]
	fn memory_map(address: *mut c_void, length: Bytes, protection: i32, flags: i32, file_descriptor: Option<&FileDescriptor>, offset: usize) -> Result<*mut c_void, io::Error>
	{
		let address = unsafe { mmap(address, length.into(), protection, flags, file_descriptor.map(|file_descriptor| file_descriptor.file_descriptor()).unwrap_or(-1), offset as i64) };
		if unlikely!(address == MAP_FAILED)
		{
			last_error()
		}
		else
		{
			Ok(address)
		}
	}
	
	#[inline(always)]
	fn memory_lock(address: *mut c_void, length: Bytes) -> Result<(), io::Error>
	{
		if unsafe { mlock(address, length.into()) } == Failure
		{
			last_error()
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn mirror_address(address: *mut c_void, offset: Bytes) -> *mut c_void
	{
		unsafe { address.offset(offset.into()) }
	}
}
