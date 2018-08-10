// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// An untagged union representing codes.
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union InternetControlMessageProtocolVersion6Code
{
	/// Destination Unreachable (RFC 4443).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::DestinationUnreachable`.
	pub destination_unreachable: InternetControlMessageProtocolVersion6CodeDestinationUnreachable,
	
	/// Packet Too Big (RFC 4443).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::PacketTooBig`.
	pub packet_too_big: InternetControlMessageProtocolVersion6CodePacketTooBig,
	
	/// Time Exceeded (RFC 4443).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::TimeExceeded`.
	pub time_exceeded: InternetControlMessageProtocolVersion6CodeTimeExceeded,
	
	/// Parameter Problem (RFC 4443).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::TimeExceeded`.
	pub parameter_problem: InternetControlMessageProtocolVersion6CodeParameterProblem,
	
	/// Echo Request ('ping') (RFC 4443).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::EchoRequest`.
	pub echo_request: InternetControlMessageProtocolVersion6CodeEchoRequest,
	
	/// Echo Reply ('pong') (RFC 4443).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::EchoReply`.
	pub echo_reply: InternetControlMessageProtocolVersion6CodeEchoReply,
	
	/// Multicast Listener Query (RFC 2720).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastListenerQuery`.
	pub multicast_listener_query: InternetControlMessageProtocolVersion6CodeMulticastListenerQuery,
	
	/// Multicast Listener Report (RFC 2720).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastListenerReport`.
	pub multicast_listener_report: InternetControlMessageProtocolVersion6CodeMulticastListenerReport,
	
	/// Multicast Listener Done (RFC 2720).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastListenerDone`.
	pub multicast_listener_done: InternetControlMessageProtocolVersion6CodeMulticastListenerDone,
	
	/// Router Solicitation (RFC 4861).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RouterSolicitation`.
	pub router_solicitation: InternetControlMessageProtocolVersion6CodeRouterSolicitation,
	
	/// Router Advertisement (RFC 4861).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RouterAdvertisement`.
	pub router_advertisement: InternetControlMessageProtocolVersion6CodeRouterAdvertisement,
	
	/// Neighbor Solicitation (RFC 4861).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NeighborSolicitation`.
	pub neighbor_solicitation: InternetControlMessageProtocolVersion6CodeNeighborSolicitation,
	
	/// Neighbor Advertisement (RFC 4861).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NeighborAdvertisement`.
	pub neighbor_advertisement: InternetControlMessageProtocolVersion6CodeNeighborAdvertisement,
	
	/// Redirect (RFC 4861).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::Redirect`.
	pub redirect: InternetControlMessageProtocolVersion6CodeRedirect,
	
	/// Router Renumbering (RFC 2894).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RouterRenumbering`.
	pub router_renumbering: InternetControlMessageProtocolVersion6CodeRouterRenumbering,
	
	/// Node Information Query (RFC 4620).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NodeInformationQuery`.
	pub node_information_query: InternetControlMessageProtocolVersion6CodeNodeInformationQuery,
	
	/// Node Information Response (RFC 4620).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::NodeInformationResponse`.
	pub node_information_response: InternetControlMessageProtocolVersion6CodeNodeInformationResponse,
	
	/// Inverse Neighbor Discovery Solicitation (RFC 3112).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::InverseNeighborDiscoverySolicitation`.
	pub inverse_neighbor_discovery_solicitation: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation,
	
	/// Inverse Neighbor Discovery Advertisement (RFC 3112).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::InverseNeighborDiscoveryAdvertisement`.
	pub inverse_neighbor_discovery_advertisement: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoveryAdvertisement,
	
	/// Version 2 Multicast Listener Report (RFC 3810).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::Version2MulticastListenerReport`.
	pub version_2_multicast_listener_report: InternetControlMessageProtocolVersion6CodeVersion2MulticastListenerReport,

	/// Home Agent Address Discovery Request (RFC 3775).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::HomeAgentAddressDiscoveryRequest`.
	pub home_agent_address_discovery_request: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryRequest,
	
	/// Home Agent Address Discovery Reply (RFC 3775).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::HomeAgentAddressDiscoveryReply`.
	pub home_agent_address_discovery_reply: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryReply,
	
	/// Mobile Prefix Solicitation (RFC 3775).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MobilePrefixSolicitation`.
	pub mobile_prefix_solicitation: InternetControlMessageProtocolVersion6CodeMobilePrefixSolicitation,
	
	/// Mobile Prefix Advertisement (RFC 3775).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MobilePrefixAdvertisement`.
	pub mobile_prefix_advertisement: InternetControlMessageProtocolVersion6CodeMobilePrefixAdvertisement,
	
	/// Certification Path Solicitation (RFC 3971).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::CertificationPathSolicitation`.
	pub certification_path_solicitation: InternetControlMessageProtocolVersion6CodeCertificationPathSolicitation,
	
	/// Certification Path Advertisement (RFC 3971).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::CertificationPathAdvertisement`.
	pub certification_path_advertisement: InternetControlMessageProtocolVersion6CodeCertificationPathAdvertisement,
	
	/// Utilized by experimental mobility protocols such as Seamoby (RFC 4065).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::ExperimentalMobilityProtocol`.
	pub experimental_mobility_protocol: InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol,
	
	/// Multicast Router Advertisement (RFC 4286).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastRouterAdvertisement`.
	pub multicast_router_advertisement: InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement,
	
	/// Multicast Router Solicitation (RFC 4286).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastRouterSolicitation`.
	pub multicast_router_solicitation: InternetControlMessageProtocolVersion6CodeMulticastRouterSolicitation,
	
	/// Multicast Router Termination (RFC 4286).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::MulticastRouterTermination`.
	pub multicast_router_termination: InternetControlMessageProtocolVersion6CodeMulticastRouterTermination,

	/// Fast handovers for Mobile Internet Protocol (IP) version 6, aka FMIPv6 (RFC 5568).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::FastHandoversForMobileInternetProtocolVersion6`.
	pub fast_handovers_for_mobile_internet_protocol_version_6: InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6,
	
	/// Internet Protocol (IP) version 6 Routing Protocol for Low-Power and Lossy Networks Control (RPL) (RFC 6550).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::RoutingProtocolForLowPowerAndLossyNetworksControl`.
	pub routing_protocol_for_low_power_and_lossy_networks_control: InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl,
	
	
	
	
	
	
	
	
	/// Extended Echo Request ('ping') (RFC 8335).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::ExtendedEchoRequest`.
	pub extended_echo_request: InternetControlMessageProtocolVersion6CodeExtendedEchoRequest,
	
	/// Extended Echo Reply ('pong') (RFC 8335).
	///
	/// The `type_` field of the packet header's `type_and_code` field is `InternetControlMessageProtocolVersion6Type::ExtendedEchoReply`.
	pub extended_echo_reply: InternetControlMessageProtocolVersion6CodeExtendedEchoReply,
	
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
