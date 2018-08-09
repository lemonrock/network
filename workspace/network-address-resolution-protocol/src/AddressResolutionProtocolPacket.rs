// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// This is a specialized structure designed to represent a buffer of packet data.
#[repr(C, packed)]
#[derive(Debug)]
pub struct AddressResolutionProtocolPacket
{
	/// Header.
	pub header: AddressResolutionProtocolPacketHeader,

	/// Payload.
	pub payload: AddressResolutionProtocolPacketPayload,
}

impl Display for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

#[cfg(feature = "dpdk-sys")]
impl Into<arp_hdr> for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> arp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a arp_hdr> for &'a AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> &'a arp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<&'a mut arp_hdr> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> &'a mut arp_hdr
	{
		unsafe { transmute(self) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<NonNull<arp_hdr>> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> NonNull<arp_hdr>
	{
		unsafe { NonNull::new_unchecked(self as *mut AddressResolutionProtocolPacket as *mut arp_hdr) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*const arp_hdr> for &'a AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> *const arp_hdr
	{
		self as *const AddressResolutionProtocolPacket as *const _
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> Into<*mut arp_hdr> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn into(self) -> *mut arp_hdr
	{
		self as *mut AddressResolutionProtocolPacket as *mut _
	}
}

#[cfg(feature = "dpdk-sys")]
impl From<arp_hdr> for AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn from(value: arp_hdr) -> Self
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a arp_hdr> for &'a AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn from(value: &'a arp_hdr) -> &'a AddressResolutionProtocolPacket
	{
		unsafe { transmute(value) }
	}
}

#[cfg(feature = "dpdk-sys")]
impl<'a> From<&'a mut arp_hdr> for &'a mut AddressResolutionProtocolPacket
{
	#[inline(always)]
	fn from(value: &'a mut arp_hdr) -> &'a mut AddressResolutionProtocolPacket
	{
		unsafe { transmute(value) }
	}
}

impl AddressResolutionProtocolPacket
{
	/// Use this to eliminate invalid traffic.
	#[inline(always)]
	pub fn is_packet_length_too_short(layer_3_length: u16) -> bool
	{
		AddressResolutionProtocolPacketHeader::is_packet_length_too_short(layer_3_length)
	}

	/// Use this to eliminate obsolete ARP traffic.
	#[inline(always)]
	pub fn is_invalid_for_internet_protocol_version_4(&self, layer_3_length: u16) -> bool
	{
		self.is_layer_3_length_invalid_for_internet_protocol_version_4(layer_3_length) || self.header.is_header_invalid_for_internet_protocol_version_4()
	}
	
	/// The only kind of payload commonly encountered.
	#[inline(always)]
	pub fn internet_protocol_version_4_payload(&self) -> &AddressResolutionProtocolPacketInternetProtocolVersion4Payload
	{
		self.payload.internet_protocol_version_4_payload()
	}
	
	/// Is the the layer 3 length invalid for an internet protocol version 4 ARP message?
	#[inline(always)]
	pub fn is_layer_3_length_invalid_for_internet_protocol_version_4(&self, layer_3_length: u16) -> bool
	{
		const PayloadSizeU16: u16 = size_of::<AddressResolutionProtocolPacketInternetProtocolVersion4Payload>() as u16;
		
		layer_3_length != AddressResolutionProtocolPacketHeader::HeaderSizeU16 + PayloadSizeU16
	}
	
	#[inline(always)]
	pub(crate) fn process<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=AddressResolutionProtocolIncomingNetworkPacketDropReason>>(&'lifetime self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, packet_processing: &AddressResolutionPacketProcessing<EINPDO>, layer_3_length: u16, ethernet_addresses: &'lifetime EthernetAddresses)
	{
		if unlikely!(self.is_invalid_for_internet_protocol_version_4(layer_3_length))
		{
			drop!(now, NotSupportedForAnythingOtherThanInternetProtocolVersion4, ethernet_addresses, packet_processing, packet)
		}
		
		self.process_for_internet_protocol_version_4_payload(now, packet, packet_processing, ethernet_addresses)
	}
	
	#[inline(always)]
	fn process_for_internet_protocol_version_4_payload<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=AddressResolutionProtocolIncomingNetworkPacketDropReason>>(&'lifetime self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, packet_processing: &AddressResolutionPacketProcessing<EINPDO>, ethernet_addresses: &'lifetime EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		let header = unsafe { NonNull::new_unchecked(&self.header as *const _ as *mut _) };
		
		debug_assert!(source_ethernet_address.is_valid_unicast(), "source_ethernet_address '{}' is not valid unicast", source_ethernet_address);

		if unlikely!(destination_ethernet_address.is_multicast())
		{
			drop!(now, DestinationEthernetAddressIsMulticast { header }, ethernet_addresses, packet_processing, packet)
		}

		debug_assert!(destination_ethernet_address.is_valid_unicast() || destination_ethernet_address.is_broadcast(), "destination_ethernet_address '{}' is not valid unicast or broadcast()", destination_ethernet_address);

		match self.header.operation
		{
			Operation::Request => self.process_request(now, packet, packet_processing, ethernet_addresses),

			Operation::Reply => self.process_reply(now, packet, packet_processing, ethernet_addresses),

			_ => drop!(now, OperationIsUnsupported { header }, ethernet_addresses, packet_processing, packet),
		}
	}

	#[inline(always)]
	fn process_request<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=AddressResolutionProtocolIncomingNetworkPacketDropReason>>(&'lifetime self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, packet_processing: &AddressResolutionPacketProcessing<EINPDO>, ethernet_addresses: &'lifetime EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		let header = unsafe { NonNull::new_unchecked(&self.header as *const _ as *mut _) };
		
		// RFC 1122 Section 2.3.2.1: "(2) Unicast Poll -- Actively poll the remote host by periodically sending a point-to-point ARP Request to it, and delete the entry if no ARP Reply is received from N successive polls".
		//
		// Thus an ARP request should be either to a broadcast address (normal behaviour) or to an unicast address.
		if destination_ethernet_address.is_multicast()
		{
			drop!(now, RequestIsMulticast { header }, ethernet_addresses, packet_processing, packet)
		}

		let payload = self.internet_protocol_version_4_payload();

		// This violates RFC 5227 which states that requests SHOULD have a non-zero target hardware address.
		if cfg!(feature = "drop-arp-requests-with-non-zero-target-hardware-address")
		{
			if unlikely!(payload.target_hardware_address.is_not_zero())
			{
				drop!(now, RequestTargetHardwareAddressIsZero { header }, ethernet_addresses, packet_processing, packet)
			}
		}

		let sender_hardware_address = &payload.sender_hardware_address;
		if unlikely!(source_ethernet_address != sender_hardware_address)
		{
			drop!(now, HardwareAndPacketSourceEthernetAddressMismatch { header }, ethernet_addresses, packet_processing, packet)
		}

		let target_protocol_address = payload.target_protocol_address;
		if unlikely!(target_protocol_address.is_not_valid_unicast())
		{
			drop!(now, HardwareAndPacketDestinationEthernetAddressMismatch { header }, ethernet_addresses, packet_processing, packet)
		}

		let sender_protocol_address = payload.sender_protocol_address;

		// sender_hardware_address: MUST be valid source ethernet address.
		// sender_protocol_address: MUST be all zeros (unspecified).
		// target_hardware_address: SHOULD be zeros; it is ignored.
		// target_protocol_address: MUST be set to address being probed.
		let is_arp_probe = sender_protocol_address.is_unspecified();
		if is_arp_probe
		{
			let we_own_the_target_protocol_address_so_reply = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(target_protocol_address);
			if unlikely!(we_own_the_target_protocol_address_so_reply)
			{
				packet_processing.reply_to_probe(packet, ethernet_addresses);
				drop!(now, ReuseInReply, ethernet_addresses, packet_processing, packet);
			}
			else
			{
				drop!(now, ProbeIsNotForUs { header }, ethernet_addresses, packet_processing, packet)
			}
		}
		else
		{
			if destination_ethernet_address.is_not_broadcast()
			{
				drop!(now, RequestIsNotAProbeAndIsNotBroadcast { header }, ethernet_addresses, packet_processing, packet)
			}

			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				drop!(now, RequestIsNotAProbeAndSenderProtocolAddressIsNotUnicast { header }, ethernet_addresses, packet_processing, packet)
			}

			let internet_protocol_version_4_host_address_conflict = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(sender_protocol_address);
			if internet_protocol_version_4_host_address_conflict
			{
				packet_processing.internet_protocol_version_4_host_address_conflict(packet, ethernet_addresses);
				drop!(now, ReuseInReply, ethernet_addresses, packet_processing, packet);
			}

			// Also known as a gratuitous ARP request.
			let is_arp_announcement = sender_protocol_address == target_protocol_address;
			if is_arp_announcement
			{
				packet_processing.add_to_address_resolution_cache(sender_hardware_address, sender_protocol_address, packet);
				return
			}

			let we_own_the_target_protocol_address_so_reply = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(target_protocol_address);
			if we_own_the_target_protocol_address_so_reply
			{
				packet_processing.reply_to_broadcast(packet, ethernet_addresses);
				drop!(now, ReuseInReply, ethernet_addresses, packet_processing, packet);
			}
			
			drop!(now, BroadcastIsNotForUs { header }, ethernet_addresses, packet_processing, packet)
		}
	}
	
	#[inline(always)]
	fn process_reply<'lifetime, EINPDO: EthernetIncomingNetworkPacketDropObserver<ARPINPDR=AddressResolutionProtocolIncomingNetworkPacketDropReason>>(&'lifetime self, now: MonotonicMillisecondTimestamp, packet: impl EthernetIncomingNetworkPacket, packet_processing: &AddressResolutionPacketProcessing<EINPDO>, ethernet_addresses: &'lifetime EthernetAddresses)
	{
		let (source_ethernet_address, destination_ethernet_address) = ethernet_addresses.addresses();
		let header = unsafe { NonNull::new_unchecked(&self.header as *const _ as *mut _) };
		
		let payload = self.internet_protocol_version_4_payload();
		let sender_hardware_address = &payload.sender_hardware_address;
		let target_hardware_address = &payload.target_hardware_address;
		let sender_protocol_address = payload.sender_protocol_address;
		let target_protocol_address = payload.target_protocol_address;

		let internet_protocol_version_4_host_address_conflict = packet_processing.is_internet_protocol_version_4_host_address_one_of_ours(sender_protocol_address);
		if internet_protocol_version_4_host_address_conflict
		{
			packet_processing.internet_protocol_version_4_host_address_conflict(packet, ethernet_addresses);
			return
		}

		let sender_and_target_protocol_addresses_are_the_same = sender_protocol_address == target_protocol_address;

		// A gratuitous ARP reply is a reply to which no request has been made.
		// These are less common than a gratuitous ARP request, and not preferred, see RFC 5227 Section 3.
		let is_gratuitous_arp_reply = sender_and_target_protocol_addresses_are_the_same && (target_hardware_address.is_broadcast() || target_hardware_address.is_zero());
		if is_gratuitous_arp_reply
		{
			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				drop!(now, GratuitousReplyIsNotValidUnicast { header }, ethernet_addresses, packet_processing, packet)
			}
		}
		else
		{
			if unlikely!(source_ethernet_address != sender_hardware_address)
			{
				drop!(now, HardwareAndPacketSourceEthernetAddressMismatch { header }, ethernet_addresses, packet_processing, packet)
			}

			if unlikely!(destination_ethernet_address != target_hardware_address)
			{
				drop!(now, HardwareAndPacketDestinationEthernetAddressMismatch { header }, ethernet_addresses, packet_processing, packet)
			}

			if unlikely!(target_hardware_address.is_not_valid_unicast())
			{
				drop!(now, ReplyTargetHardwareAddressIsNotValidUnicast { header }, ethernet_addresses, packet_processing, packet)
			}

			if unlikely!(sender_and_target_protocol_addresses_are_the_same)
			{
				drop!(now, ReplySourceAndTargetProtocolAddressesAreTheSame { header }, ethernet_addresses, packet_processing, packet)
			}

			if unlikely!(sender_protocol_address.is_not_valid_unicast())
			{
				drop!(now, ReplySenderProtocolAddressIsNotValidUnicast { header }, ethernet_addresses, packet_processing, packet)
			}

			if unlikely!(target_protocol_address.is_not_valid_unicast())
			{
				drop!(now, ReplyTargetProtocolAddressIsNotValidUnicast { header }, ethernet_addresses, packet_processing, packet)
			}
		}

		packet_processing.add_to_address_resolution_cache(sender_hardware_address, sender_protocol_address, packet);
	}
}
