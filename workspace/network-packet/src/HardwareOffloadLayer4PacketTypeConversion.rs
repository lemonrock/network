// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Conversion.
pub trait HardwareOffloadLayer4PacketTypeConversion<PacketType>
{
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 11:8 (0x0F00) are significant.
	#[inline(always)]
	fn from_packet_buffer_packet_type(packet_type: u32) -> HardwareOffloadLayer4PacketType;
	
	//noinspection SpellCheckingInspection
	/// From packet buffer's packet type.
	///
	/// Only bits 27:24 are significant.
	#[inline(always)]
	fn inner_layer_4_for_tunnel_from_packet_buffer_packet_type(packet_type: u32) -> HardwareOffloadLayer4PacketType;
}
