// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternetControlMessageProtocolPacketHeader
{
	/// Type.
	pub type_: InternetControlMessageProtocolType,
	
	/// The meaning of code depends on type.
	pub code: u8,
	
	/// The checksum includes the payload.
	pub checksum: InternetCheckSum,
	
	/// Rest-of-header.
	pub rest_of_header: RestOfHeader,
}

impl Display for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

#[cfg(feature = "dpdk-sys")]
impl Into<icmp_hdr> for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> icmp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a icmp_hdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a icmp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<NonNull<icmp_hdr>> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<icmp_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut InternetControlMessageProtocolPacketHeader as *mut icmp_hdr) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*const icmp_hdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> *const icmp_hdr
	{
		self as *const InternetControlMessageProtocolPacketHeader as *const _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*mut icmp_hdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut icmp_hdr
	{
		self as *mut InternetControlMessageProtocolPacketHeader as *mut _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a mut icmp_hdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut icmp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl From<icmp_hdr> for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: icmp_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a icmp_hdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: &'a icmp_hdr) -> &'a InternetControlMessageProtocolPacketHeader
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a mut icmp_hdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut icmp_hdr) -> &'a mut InternetControlMessageProtocolPacketHeader
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "libc")]
impl Into<icmphdr> for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> icmphdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "libc")]
impl<'a> Into<&'a icmphdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a icmphdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "libc")]
impl<'a> Into<NonNull<icmphdr>> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> NonNull<icmphdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut InternetControlMessageProtocolPacketHeader as *mut icmphdr) }
	}
}

#[cfg(feature = "libc")]
impl<'a> Into<*const icmphdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> *const icmphdr
	{
		self as *const InternetControlMessageProtocolPacketHeader as *const _
	}
}

#[cfg(feature = "libc")]
impl<'a> Into<*mut icmphdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> *mut icmphdr
	{
		self as *mut InternetControlMessageProtocolPacketHeader as *mut _
	}
}

#[cfg(feature = "libc")]
impl<'a> Into<&'a mut icmphdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn into(self) -> &'a mut icmphdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "libc")]
impl From<icmphdr> for InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: icmphdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "libc")]
impl<'a> From<&'a icmphdr> for &'a InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: &'a icmphdr) -> &'a InternetControlMessageProtocolPacketHeader
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "libc")]
impl<'a> From<&'a mut icmphdr> for &'a mut InternetControlMessageProtocolPacketHeader
{
	#[inline(always)]
	fn from(value: &'a mut icmphdr) -> &'a mut InternetControlMessageProtocolPacketHeader
	{
		unsafe { transmute(value) }
	}
}
