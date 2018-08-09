// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Permitted classes of service.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct PermittedClassesOfService([bool; 8]);

impl Display for PermittedClassesOfService
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Default for PermittedClassesOfService
{
	#[inline(always)]
	fn default() -> Self
	{
		PermittedClassesOfService([true; 8])
	}
}

impl PermittedClassesOfService
{
	/// Permit this `class_of_service`.
	#[inline(always)]
	pub fn permit(&mut self, class_of_service: ClassOfService)
	{
		unsafe { *self.0.get_unchecked_mut(class_of_service as u8 as usize) = true }
	}
	
	/// Deny this `class_of_service`.
	#[inline(always)]
	pub fn deny(&mut self, class_of_service: ClassOfService)
	{
		unsafe { *self.0.get_unchecked_mut(class_of_service as u8 as usize) = false }
	}
	
	/// Is this `class_of_service` permitted?
	#[inline(always)]
	pub fn is_permitted(&self, class_of_service: ClassOfService) -> bool
	{
		unsafe { *self.0.get_unchecked(class_of_service as u8 as usize) }
	}
	
	/// Is this `class_of_service` denied?
	#[inline(always)]
	pub fn is_denied(&self, class_of_service: ClassOfService) -> bool
	{
		!self.is_permitted(class_of_service)
	}
}
