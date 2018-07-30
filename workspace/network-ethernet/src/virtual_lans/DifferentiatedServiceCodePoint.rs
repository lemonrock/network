// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Differentiated Service Code Point (DSCP) value.
///
/// Defaults to `DefaultForwarding`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct DifferentiatedServiceCodePoint(u8);

impl Display for DifferentiatedServiceCodePoint
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "0b{:06b}", self.0)
	}
}

impl Default for DifferentiatedServiceCodePoint
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::DefaultForwarding
	}
}

impl From<u8> for DifferentiatedServiceCodePoint
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		DifferentiatedServiceCodePoint(value)
	}
}

impl Into<u8> for DifferentiatedServiceCodePoint
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl DifferentiatedServiceCodePoint
{
	/// Also known as Best Effort.
	pub const DefaultForwarding: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b000000);
	
	/// Expedited Forwarding.
	pub const ExpeditedForwarding: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b101110);
	
	/// Voice Admit.
	pub const VoiceAdmit: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b101100);
	
	/// Also known as AF11.
	pub const AssuredForwardingClass1LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b001010);
	
	/// Also known as AF12.
	pub const AssuredForwardingClass1MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b001100);
	
	/// Also known as AF13.
	pub const AssuredForwardingClass1HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b001110);
	
	/// Also known as AF21.
	pub const AssuredForwardingClass2LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b010010);
	
	/// Also known as AF22.
	pub const AssuredForwardingClass2MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b010100);
	
	/// Also known as AF23.
	pub const AssuredForwardingClass2HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b010110);
	
	/// Also known as AF31.
	pub const AssuredForwardingClass3LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b011010);
	
	/// Also known as AF32.
	pub const AssuredForwardingClass3MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b011100);
	
	/// Also known as AF43.
	pub const AssuredForwardingClass3HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b011110);
	
	/// Also known as AF41.
	pub const AssuredForwardingClass4LowDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b100010);
	
	/// Also known as AF42.
	pub const AssuredForwardingClass4MediumDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b100100);
	
	/// Also known as AF43.
	pub const AssuredForwardingClass4HighDropProbability: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b100110);
	
	/// ?
	pub const InterNetworkControl: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b110000);
	
	/// ?
	pub const NetworkControl: DifferentiatedServiceCodePoint = DifferentiatedServiceCodePoint(0b111000);
	
	/// Is this code point for local or experimental use?
	#[inline(always)]
	pub fn is_for_local_or_experimental_use(self) -> bool
	{
		self.0 & 0b11 != 0
	}
}
