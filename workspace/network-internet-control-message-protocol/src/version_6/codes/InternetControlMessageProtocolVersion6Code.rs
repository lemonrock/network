// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An untagged union representing codes.
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union InternetControlMessageProtocolVersion6Code
{
	/// Destination Unreachable.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::DestinationUnreachable`.
	pub destination_unreachable: InternetControlMessageProtocolVersion6CodeDestinationUnreachable,
	
	/// Packet too big.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::PacketTooBig`.
	pub packet_too_big: InternetControlMessageProtocolVersion6CodePacketTooBig,
	
	/// Time exceeded.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::TimeExceeded`.
	pub time_exceeded: InternetControlMessageProtocolVersion6CodeTimeExceeded,
	
	/// Parameter problem.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::TimeExceeded`.
	pub parameter_problem: InternetControlMessageProtocolVersion6CodeParameterProblem,
	
	/// Echo request ('ping').
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::EchoRequest`.
	pub echo_request: InternetControlMessageProtocolVersion6CodeEchoRequest,
	
	/// Echo reply ('pong').
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::EchoReply`.
	pub echo_reply: InternetControlMessageProtocolVersion6CodeEchoReply,
	
	/// Multicast listener query.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastListenerQuery`.
	pub multicast_listener_query: InternetControlMessageProtocolVersion6CodeMulticastListenerQuery,
	
	/// Multicast listener report.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastListenerReport`.
	pub multicast_listener_report: InternetControlMessageProtocolVersion6CodeMulticastListenerReport,
	
	/// Multicast listener done.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastListenerDone`.
	pub multicast_listener_done: InternetControlMessageProtocolVersion6CodeMulticastListenerDone,
	
	/// Router solicitation.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RouterSolicitation`.
	pub router_solicitation: InternetControlMessageProtocolVersion6CodeRouterSolicitation,
	
	/// Router advertisement.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RouterAdvertisement`.
	pub router_advertisement: InternetControlMessageProtocolVersion6CodeRouterAdvertisement,
	
	/// Neighbor solicitation.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NeighborSolicitation`.
	pub neighbor_solicitation: InternetControlMessageProtocolVersion6CodeNeighborSolicitation,
	
	/// Neighbor advertisement.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NeighborAdvertisement`.
	pub neighbor_advertisement: InternetControlMessageProtocolVersion6CodeNeighborAdvertisement,
	
	/// Redirect message.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RedirectMessage`.
	pub redirect_message: InternetControlMessageProtocolVersion6CodeRedirectMessage,
	
	/// Router renumbering.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RouterRenumbering`.
	pub router_renumbering: InternetControlMessageProtocolVersion6CodeRouterRenumbering,
	
	/// Node information query.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NodeInformationQuery`.
	pub node_information_query: InternetControlMessageProtocolVersion6CodeNodeInformationQuery,
	
	/// Node information response.
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NodeInformationResponse`.
	pub node_information_response: InternetControlMessageProtocolVersion6CodeNodeInformationResponse,
	
	/// Many `type_` values in the packet header are experimental, reserved, obscure or lacking in information.
	///
	/// Or their may be just an unexpected code value present.
	pub undifferentiated: u8,
}

impl Serialize for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		(unsafe { self.undifferentiated }).serialize(serializer)
	}
}

impl<'deserialize> Deserialize<'deserialize> for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
	{
		Ok
		(
			Self
			{
				undifferentiated: u8::deserialize(deserializer)?,
			}
		)
		
	}
}

impl From<u8> for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn from(undifferentiated: u8) -> Self
	{
		InternetControlMessageProtocolVersion6Code
		{
			undifferentiated
		}
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn into(self) -> u8
	{
		unsafe { self.undifferentiated }
	}
}

impl Display for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl Debug for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", unsafe { self.undifferentiated })
	}
}

impl PartialOrd for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.undifferentiated.partial_cmp(&other.undifferentiated) }
	}
}

impl Ord for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.undifferentiated.cmp(&other.undifferentiated) }
	}
}

impl PartialEq for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.undifferentiated == other.undifferentiated }
	}
}

impl Eq for InternetControlMessageProtocolVersion6Code
{
}

impl Hash for InternetControlMessageProtocolVersion6Code
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		hasher.write_u8(unsafe { self.undifferentiated })
	}
}
