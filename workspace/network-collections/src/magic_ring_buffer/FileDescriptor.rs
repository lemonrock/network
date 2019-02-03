// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct FileDescriptor(RawFd);

impl Drop for FileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.0 != -1
		{
			unsafe { close(self.0) };
		}
	}
}

impl FileDescriptor
{
	const InvalidFileDescriptor: i32 = -1;
	
	#[inline(always)]
	pub(crate) fn create_temporary_file_and_remove_it(temporary_directory_path: impl AsRef<Path>, file_extension: &str) -> Result<Self, io::Error>
	{
		let temporary_file_path_template =
		{
			let path = temporary_directory_path.as_ref();
			let joined = path.join(format!("XXXXXXXXXXXX.{}", file_extension));
			CString::new(joined.as_os_str().as_bytes()).unwrap().into_raw()
		};
		
		const DotOfFileExtension: usize = 1;
		let file_descriptor_or_error = unsafe { mkstemps(temporary_file_path_template, (DotOfFileExtension + file_extension.len()) as i32) };
		
		let temporary_file_path = unsafe { CString::from_raw(temporary_file_path_template) };
		
		if file_descriptor_or_error == Self::InvalidFileDescriptor
		{
			return last_error()
		}
		
		let this = FileDescriptor(file_descriptor_or_error);
		
		if unsafe { unlink(temporary_file_path.as_ptr()) } == -1
		{
			return last_error()
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	pub(crate) fn truncate(&self, length: Bytes) -> Result<(), io::Error>
	{
		debug_assert_ne!(self.0, Self::InvalidFileDescriptor, "closed");
		
		if unsafe { ftruncate(self.0, length.into()) } == Failure
		{
			return last_error()
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn close(mut self) -> Result<(), io::Error>
	{
		debug_assert_ne!(self.0, Self::InvalidFileDescriptor, "closed");
		
		let file_descriptor = self.file_descriptor();
		self.0 = Self::InvalidFileDescriptor;
		
		if unsafe { close(file_descriptor) } == Failure
		{
			last_error()
		}
		else
		{
			Ok(())
		}
	}
	
	
	#[inline(always)]
	fn file_descriptor(&self) -> i32
	{
		self.0
	}
}
