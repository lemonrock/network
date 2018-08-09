// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents an Ethernet incoming network packet.
pub trait EthernetIncomingNetworkPacket: IncomingNetworkPacket
{
	/// Packet length less ethernet header.
	#[inline(always)]
	fn packet_length_if_contiguous_less_ethernet_packet_header(self) -> u16
	{
		self.packet_length_if_contiguous() - EthernetPacketHeader::SizeU16
	}
	
	/// Is too short to be an ethernet packet?
	#[inline(always)]
	fn is_too_short_to_be_an_ethernet_packet(self) -> bool
	{
		self.packet_length_if_contiguous() < EthernetPacketHeader::SizeU16
	}
	
	/// Is too short to be an IEEE 802.1Q Virtual LAN packet?
	#[inline(always)]
	fn is_too_short_to_be_a_vlan_ethernet_packet(self) -> bool
	{
		const Overhead: u16 = VirtualLanPacketHeader::IEEE_802_1Q_SizeU16;
		
		self.packet_length_if_contiguous() < (EthernetPacketHeader::SizeU16 + Overhead)
	}
	
	/// Is too short to be an IEEE 802.1ad QinQ Virtual LAN packet?
	#[inline(always)]
	fn is_too_short_to_be_a_qinq_vlan_ethernet_packet(self) -> bool
	{
		const Overhead: u16 = VirtualLanPacketHeader::IEEE_802_1ad_SizeU16 + VirtualLanPacketHeader::IEEE_802_1Q_SizeU16;
		
		self.packet_length_if_contiguous() < (EthernetPacketHeader::SizeU16 + Overhead)
	}
	
	/// Was VLAN tag control information (TCI) stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	#[inline(always)]
	fn was_vlan_tag_control_information_stripped(self) -> bool;
	
	/// Stripped VLAN tag control information (TCI).
	#[inline(always)]
	fn stripped_vlan_tag_control_information(self) -> TagControlInformation;
	
	/// Was VLAN QinQ tag control information (TCI) stripped (ie did the hardware pull it out of the received packet and put it into this structure)?
	#[inline(always)]
	fn was_vlan_qinq_tag_control_information_stripped(self) -> bool;
	
	/// Stripped VLAN QinQ tag control information (TCI) (outer and inner).
	#[inline(always)]
	fn stripped_vlan_qinq_tag_control_information(self) -> (TagControlInformation, TagControlInformation);
	
	/// Ethernet packet.
	///
	/// No checking of data length is made; be careful dereferencing this value.
	/// Call one of `is_too_short_to_be_an_ethernet_packet()`, `is_too_short_to_be_a_vlan_ethernet_packet()` or `is_too_short_to_be_a_qinq_vlan_ethernet_packet()` first.
	#[inline(always)]
	fn ethernet_packet<'a>(self) -> &'a EthernetPacket
	{
		self.offset_into_data_reference::<'a, EthernetPacket>(0)
	}
}
