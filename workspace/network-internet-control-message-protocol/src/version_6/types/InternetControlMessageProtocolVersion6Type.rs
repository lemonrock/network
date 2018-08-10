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
	/// RFC 4443: Destination Unreachable.
	pub const DestinationUnreachable: Self = InternetControlMessageProtocolVersion6Type(1);
	
	/// RFC 4443: Packet Too Big.
	pub const PacketTooBig: Self = InternetControlMessageProtocolVersion6Type(2);
	
	/// RFC 4443: Time Exceeded.
	pub const TimeExceeded: Self = InternetControlMessageProtocolVersion6Type(3);
	
	/// RFC 4443: Parameter Problem.
	pub const ParameterProblem: Self = InternetControlMessageProtocolVersion6Type(4);
	
	/// RFC 4443: Private error message experiment.
	pub const PrivateErrorMessageExperiment1: Self = InternetControlMessageProtocolVersion6Type(100);
	
	/// RFC 4443: Private error message experiment.
	pub const PrivateErrorMessageExperiment2: Self = InternetControlMessageProtocolVersion6Type(101);
	
	/// RFC 4443: "reserved for future expansion of the \[error message\] value range if there is a shortage in the future".
	pub const ReservedForExpansionOfErrorMessageRange: Self = InternetControlMessageProtocolVersion6Type(127);
	
	/// RFC 4443: Echo Request ('ping')
	pub const EchoRequest: Self = InternetControlMessageProtocolVersion6Type(128);
	
	/// RFC 4443: Echo Reply ('pong').
	pub const EchoReply: Self = InternetControlMessageProtocolVersion6Type(129);
	
	/// RFC 2710: Multicast Listener Query.
	pub const MulticastListenerQuery: Self = InternetControlMessageProtocolVersion6Type(130);
	
	/// RFC 2710: Multicast Listener Report.
	pub const MulticastListenerReport: Self = InternetControlMessageProtocolVersion6Type(131);
	
	/// RFC 2710: Multicast Listener Done.
	pub const MulticastListenerDone: Self = InternetControlMessageProtocolVersion6Type(132);
	
	/// RFC 4861: Router Solicitation.
	pub const RouterSolicitation: Self = InternetControlMessageProtocolVersion6Type(133);
	
	/// RFC 4861: Router Advertisement.
	pub const RouterAdvertisement: Self = InternetControlMessageProtocolVersion6Type(134);
	
	/// RFC 4861: Neighbor Solicitation.
	pub const NeighborSolicitation: Self = InternetControlMessageProtocolVersion6Type(135);
	
	/// RFC 4861: Neighbor Advertisement.
	pub const NeighborAdvertisement: Self = InternetControlMessageProtocolVersion6Type(136);
	
	/// RFC 4861: Redirect Message.
	pub const RedirectMessage: Self = InternetControlMessageProtocolVersion6Type(137);
	
	/// RFC 2894: Router Renumbering.
	pub const RouterRenumbering: Self = InternetControlMessageProtocolVersion6Type(138);
	
	/// RFC 4620: Node Information Query.
	pub const NodeInformationQuery: Self = InternetControlMessageProtocolVersion6Type(139);
	
	/// RFC 4620: Node Information Response.
	pub const NodeInformationResponse: Self = InternetControlMessageProtocolVersion6Type(140);
	
	/// RFC 3122: Inverse Neighbor Discovery Solicitation.
	pub const InverseNeighborDiscoverySolicitation: Self = InternetControlMessageProtocolVersion6Type(141);
	
	/// RFC 3122: Inverse Neighbor Discovery Advertisement.
	pub const InverseNeighborDiscoveryAdvertisement: Self = InternetControlMessageProtocolVersion6Type(142);
	
	/// RFC 3810: Version 2 Multicast Listener Report.
	pub const Version2MulticastListenerReport: Self = InternetControlMessageProtocolVersion6Type(143);
	
	/// RFC 6275: Home Agent Address Discovery Request.
	pub const HomeAgentAddressDiscoveryRequest: Self = InternetControlMessageProtocolVersion6Type(144);
	
	/// RFC 6275: Home Agent Address Discovery Reply.
	pub const HomeAgentAddressDiscoveryReply: Self = InternetControlMessageProtocolVersion6Type(145);
	
	/// RFC 6275: Mobile Prefix Solicitation.
	pub const MobilePrefixSolicitation: Self = InternetControlMessageProtocolVersion6Type(146);
	
	/// RFC 6275: Mobile Prefix Advertisement.
	pub const MobilePrefixAdvertisement: Self = InternetControlMessageProtocolVersion6Type(147);
	
	/// RFC 3971: Certification Path Solicitation.
	pub const CertificationPathSolicitation: Self = InternetControlMessageProtocolVersion6Type(148);
	
	/// RFC 3971: Certification Path Advertisement.
	pub const CertificationPathAdvertisement: Self = InternetControlMessageProtocolVersion6Type(149);
	
	/// RFC 4065: ICMP messages utilized by experimental mobility protocols such as Seamoby.
	pub const ExperimentalMobilityProtocol: Self = InternetControlMessageProtocolVersion6Type(150);
	
	/// RFC 4286: Multicast Router Advertisement.
	pub const MulticastRouterAdvertisement: Self = InternetControlMessageProtocolVersion6Type(151);
	
	/// RFC 4286: Multicast Router Solicitation.
	pub const MulticastRouterSolicitation: Self = InternetControlMessageProtocolVersion6Type(152);
	
	/// RFC 4286: Multicast Router Termination.
	pub const MulticastRouterTermination: Self = InternetControlMessageProtocolVersion6Type(153);
	
	/// RFC 5568: Fast handovers for Mobile Internet Protocol (IP) v6, aka FIMPv6.
	pub const FastHandoversForMobileInternetProtocolVersion6: Self = InternetControlMessageProtocolVersion6Type(154);
	
	/// RFC 6550: RPL Control.
	pub const RPLControl: Self = InternetControlMessageProtocolVersion6Type(155);
	
	/// RFC 6743: ILNPv6 Locator Update.
	pub const ILNPv6LocatorUpdate: Self = InternetControlMessageProtocolVersion6Type(156);
	
	/// RFC 6775: Duplicate Address Request.
	pub const DuplicateAddressRequest: Self = InternetControlMessageProtocolVersion6Type(157);
	
	/// RFC 6775: Duplicate Address Confirmation.
	pub const DuplicateAddressConfirmation: Self = InternetControlMessageProtocolVersion6Type(158);
	
	/// RFC 7731: RPL Control.
	pub const MPLControl: Self = InternetControlMessageProtocolVersion6Type(159);
	
	/// RFC 8335: Echo Request ('ping')
	pub const ExtendedEchoRequest: Self = InternetControlMessageProtocolVersion6Type(160);
	
	/// RFC 8335: Echo Reply ('pong').
	pub const ExtendedEchoReply: Self = InternetControlMessageProtocolVersion6Type(161);
	
	/// RFC 4443: Private informational message experiment.
	pub const PrivateInformationalMessageExperiment1: Self = InternetControlMessageProtocolVersion6Type(200);
	
	/// RFC 4443: Private informational message experiment.
	pub const PrivateInformationalMessageExperiment2: Self = InternetControlMessageProtocolVersion6Type(201);
	
	/// RFC 4443: "reserved for future expansion of the \[informational message\] value range if there is a shortage in the future".
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
