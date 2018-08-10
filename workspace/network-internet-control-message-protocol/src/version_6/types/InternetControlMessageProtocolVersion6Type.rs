// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents common internet control message protocol (ICMP) types.
///
/// Deprecated, unassigned, reserved and experimental types are not provided for.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct InternetControlMessageProtocolVersion6Type(u8);

impl Display for InternetControlMessageProtocolVersion6Type
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<u8> for InternetControlMessageProtocolVersion6Type
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl From<u8> for InternetControlMessageProtocolVersion6Type
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		InternetControlMessageProtocolVersion6Type(value)
	}
}

impl InternetControlMessageProtocolVersion6Type
{
	/// Destination Unreachable (RFC 4443).
	pub const DestinationUnreachable: Self = InternetControlMessageProtocolVersion6Type(1);
	
	/// Packet Too Big (RFC 4443).
	pub const PacketTooBig: Self = InternetControlMessageProtocolVersion6Type(2);
	
	/// Time Exceeded (RFC 4443).
	pub const TimeExceeded: Self = InternetControlMessageProtocolVersion6Type(3);
	
	/// Parameter Problem (RFC 4443).
	pub const ParameterProblem: Self = InternetControlMessageProtocolVersion6Type(4);
	
	/// Private error message experiment (RFC 4443).
	pub const PrivateErrorMessageExperiment1: Self = InternetControlMessageProtocolVersion6Type(100);
	
	/// Private error message experiment (RFC 4443).
	pub const PrivateErrorMessageExperiment2: Self = InternetControlMessageProtocolVersion6Type(101);
	
	/// "reserved for future expansion of the \[error message\] value range if there is a shortage in the future" (RFC 4443).
	pub const ReservedForExpansionOfErrorMessageRange: Self = InternetControlMessageProtocolVersion6Type(127);
	
	/// RFC 4443: Echo Request ('ping')
	pub const EchoRequest: Self = InternetControlMessageProtocolVersion6Type(128);
	
	/// Echo Reply ('pong') (RFC 4443).
	pub const EchoReply: Self = InternetControlMessageProtocolVersion6Type(129);
	
	/// Multicast Listener Query (RFC 2710).
	pub const MulticastListenerQuery: Self = InternetControlMessageProtocolVersion6Type(130);
	
	/// Multicast Listener Report (RFC 2710).
	pub const MulticastListenerReport: Self = InternetControlMessageProtocolVersion6Type(131);
	
	/// Multicast Listener Done (RFC 2710).
	pub const MulticastListenerDone: Self = InternetControlMessageProtocolVersion6Type(132);
	
	/// Router Solicitation (RFC 4861).
	pub const RouterSolicitation: Self = InternetControlMessageProtocolVersion6Type(133);
	
	/// Router Advertisement (RFC 4861).
	pub const RouterAdvertisement: Self = InternetControlMessageProtocolVersion6Type(134);
	
	/// Neighbor Solicitation (RFC 4861).
	pub const NeighborSolicitation: Self = InternetControlMessageProtocolVersion6Type(135);
	
	/// Neighbor Advertisement (RFC 4861).
	pub const NeighborAdvertisement: Self = InternetControlMessageProtocolVersion6Type(136);
	
	/// Redirect (RFC 4861).
	pub const Redirect: Self = InternetControlMessageProtocolVersion6Type(137);
	
	/// Router Renumbering (RFC 2894).
	pub const RouterRenumbering: Self = InternetControlMessageProtocolVersion6Type(138);
	
	/// Node Information Query (RFC 4620).
	pub const NodeInformationQuery: Self = InternetControlMessageProtocolVersion6Type(139);
	
	/// Node Information Response (RFC 4620).
	pub const NodeInformationResponse: Self = InternetControlMessageProtocolVersion6Type(140);
	
	/// Inverse Neighbor Discovery Solicitation (RFC 3122).
	pub const InverseNeighborDiscoverySolicitation: Self = InternetControlMessageProtocolVersion6Type(141);
	
	/// Inverse Neighbor Discovery Advertisement (RFC 3122).
	pub const InverseNeighborDiscoveryAdvertisement: Self = InternetControlMessageProtocolVersion6Type(142);
	
	/// Version 2 Multicast Listener Report (RFC 3810).
	pub const Version2MulticastListenerReport: Self = InternetControlMessageProtocolVersion6Type(143);
	
	/// Home Agent Address Discovery Request (RFC 6275).
	pub const HomeAgentAddressDiscoveryRequest: Self = InternetControlMessageProtocolVersion6Type(144);
	
	/// Home Agent Address Discovery Reply (RFC 6275).
	pub const HomeAgentAddressDiscoveryReply: Self = InternetControlMessageProtocolVersion6Type(145);
	
	/// Mobile Prefix Solicitation (RFC 6275).
	pub const MobilePrefixSolicitation: Self = InternetControlMessageProtocolVersion6Type(146);
	
	/// Mobile Prefix Advertisement (RFC 6275).
	pub const MobilePrefixAdvertisement: Self = InternetControlMessageProtocolVersion6Type(147);
	
	/// Certification Path Solicitation (RFC 3971).
	pub const CertificationPathSolicitation: Self = InternetControlMessageProtocolVersion6Type(148);
	
	/// Certification Path Advertisement (RFC 3971).
	pub const CertificationPathAdvertisement: Self = InternetControlMessageProtocolVersion6Type(149);
	
	/// Utilized by experimental mobility protocols such as Seamoby (RFC 4065).
	pub const ExperimentalMobilityProtocol: Self = InternetControlMessageProtocolVersion6Type(150);
	
	/// Multicast Router Advertisement (RFC 4286).
	pub const MulticastRouterAdvertisement: Self = InternetControlMessageProtocolVersion6Type(151);
	
	/// Multicast Router Solicitation (RFC 4286).
	pub const MulticastRouterSolicitation: Self = InternetControlMessageProtocolVersion6Type(152);
	
	/// Multicast Router Termination (RFC 4286).
	pub const MulticastRouterTermination: Self = InternetControlMessageProtocolVersion6Type(153);
	
	/// Fast handovers for Mobile Internet Protocol (IP) v6, aka FIMPv6 (RFC 5568).
	pub const FastHandoversForMobileInternetProtocolVersion6: Self = InternetControlMessageProtocolVersion6Type(154);
	
	/// Internet Protocol (IP) version 6 Routing Protocol for Low-Power and Lossy Networks Control (RPL) (RFC 6550).
	pub const RoutingProtocolForLowPowerAndLossyNetworksControl: Self = InternetControlMessageProtocolVersion6Type(155);
	
	/// ILNPv6 Locator Update (RFC 6743).
	pub const ILNPv6LocatorUpdate: Self = InternetControlMessageProtocolVersion6Type(156);
	
	/// Duplicate Address Request (RFC 6775).
	pub const DuplicateAddressRequest: Self = InternetControlMessageProtocolVersion6Type(157);
	
	/// Duplicate Address Confirmation (RFC 6775).
	pub const DuplicateAddressConfirmation: Self = InternetControlMessageProtocolVersion6Type(158);
	
	/// MPL Control (RFC 7731).
	pub const MPLControl: Self = InternetControlMessageProtocolVersion6Type(159);
	
	/// Extended Echo Request ('ping') (RFC 8335).
	pub const ExtendedEchoRequest: Self = InternetControlMessageProtocolVersion6Type(160);
	
	/// Extended Echo Reply ('pong') (RFC 8335).
	pub const ExtendedEchoReply: Self = InternetControlMessageProtocolVersion6Type(161);
	
	/// Private informational message experiment (RFC 4443).
	pub const PrivateInformationalMessageExperiment1: Self = InternetControlMessageProtocolVersion6Type(200);
	
	/// Private informational message experiment (RFC 4443).
	pub const PrivateInformationalMessageExperiment2: Self = InternetControlMessageProtocolVersion6Type(201);
	
	/// "reserved for future expansion of the \[informational message\] value range if there is a shortage in the future" (RFC 4443).
	pub const ReservedForExpansionOfInformationalMessageRange: Self = InternetControlMessageProtocolVersion6Type(255);
	
	/// Is this an error message or an informational message?
	#[inline(always)]
	pub fn message_kind(self) -> InternetControlMessageProtocolVersion6MessageKind
	{
		use self::InternetControlMessageProtocolVersion6MessageKind::*;
		
		if self.0 <= 127
		{
			Error
		}
		else
		{
			Informational
		}
	}
}
