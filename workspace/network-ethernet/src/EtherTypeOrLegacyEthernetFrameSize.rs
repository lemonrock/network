// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Ether type or legacy ethernet frame size?
#[repr(C, packed)]
pub union EtherTypeOrLegacyEthernetFrameSize
{
	/// Legacy ethernet frame size.
	pub legacy_ethernet_frame_size: LegacyEthernetFrameSize,
	
	/// Ether Type.
	pub ether_type: EtherType,
}

impl Serialize for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		(unsafe { self.legacy_ethernet_frame_size }).serialize(serializer)
	}
}

impl<'deserialize> Deserialize<'deserialize> for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok
		(
			Self
			{
				legacy_ethernet_frame_size: LegacyEthernetFrameSize::deserialize(deserializer)?,
			}
		)
		
	}
}

impl Into<NetworkEndianU16> for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn into(self) -> NetworkEndianU16
	{
		unsafe
		{
			let value: NetworkEndianU16 = self.legacy_ethernet_frame_size.into();
			transmute(value)
		}
	}
}

impl Into<u16> for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn into(self) -> u16
	{
		let value: NetworkEndianU16 = self.into();
		value.to_native_endian()
	}
}

impl From<NetworkEndianU16> for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn from(value: NetworkEndianU16) -> Self
	{
		Self::new(value)
	}
}

impl From<u16> for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::new(NetworkEndianU16::from_native_endian(value))
	}
}

impl Clone for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			legacy_ethernet_frame_size: unsafe { self.legacy_ethernet_frame_size },
		}
	}
}

impl Copy for EtherTypeOrLegacyEthernetFrameSize
{
}

impl PartialOrd for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.legacy_ethernet_frame_size.partial_cmp(&other.legacy_ethernet_frame_size) }
	}
}

impl Ord for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.legacy_ethernet_frame_size.cmp(&other.legacy_ethernet_frame_size) }
	}
}

impl PartialEq for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.legacy_ethernet_frame_size == other.legacy_ethernet_frame_size }
	}
}

impl Eq for EtherTypeOrLegacyEthernetFrameSize
{
}

impl Hash for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.legacy_ethernet_frame_size.hash(hasher) }
	}
}

impl Debug for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", unsafe { self.legacy_ethernet_frame_size.0.to_native_endian() })
	}
}

impl Display for EtherTypeOrLegacyEthernetFrameSize
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0x{:04X}", unsafe { self.legacy_ethernet_frame_size.0.to_native_endian() })
	}
}

impl EtherTypeOrLegacyEthernetFrameSize
{
	pub(crate) const FirstByteSwitchOverValue: u8 = 0x06;
	
	/// Size of Ether Type or Frame Size field.
	pub const SizeU32: u32 = 2;
	
	/// A potentially invalid Ether type.
	#[inline(always)]
	pub fn potentially_invalid_ether_type(&self) -> EtherType
	{
		unsafe { self.ether_type }
	}
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(value: NetworkEndianU16) -> Self
	{
		unsafe { transmute(value) }
	}
}
