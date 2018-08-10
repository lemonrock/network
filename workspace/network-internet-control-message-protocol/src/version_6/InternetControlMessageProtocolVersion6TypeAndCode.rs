// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents a 'tagged enumeration' of type and code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C, packed)]
pub struct InternetControlMessageProtocolVersion6TypeAndCode
{
	/// Type.
	pub type_: InternetControlMessageProtocolVersion6Type,
	
	/// The meaning of code depends on `type_`.
	pub code: InternetControlMessageProtocolVersion6Code,
}

impl InternetControlMessageProtocolVersion6TypeAndCode
{
	/// Is this an error message or an informational message?
	#[inline(always)]
	pub fn message_kind(self) -> InternetControlMessageProtocolVersion6MessageKind
	{
		self.type_.message_kind()
	}
	
	/// Visits the particular kind of message.
	#[inline(always)]
	pub fn visit<R>(self, visitor: &impl InternetControlMessageProtocolVersion6TypeAndCodeVisitor<R>) -> R
	{
		match self.type_
		{
			InternetControlMessageProtocolVersion6Type::DestinationUnreachable => visitor.destination_unreachable(unsafe { self.code.destination_unreachable }),
			InternetControlMessageProtocolVersion6Type::PacketTooBig => visitor.packet_too_big(unsafe { self.code.packet_too_big }),
			InternetControlMessageProtocolVersion6Type::TimeExceeded => visitor.time_exceeded(unsafe { self.code.time_exceeded }),
			InternetControlMessageProtocolVersion6Type::ParameterProblem => visitor.parameter_problem(unsafe { self.code.parameter_problem }),
			InternetControlMessageProtocolVersion6Type::PrivateErrorMessageExperiment1 => visitor.private_error_message_experiment1(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::PrivateErrorMessageExperiment2 => visitor.private_error_message_experiment2(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::ReservedForExpansionOfErrorMessageRange => visitor.reserved_for_expansion_of_error_message_range(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::EchoRequest => visitor.echo_request(unsafe { self.code.echo_request }),
			InternetControlMessageProtocolVersion6Type::EchoReply => visitor.echo_reply(unsafe { self.code.echo_reply }),
			InternetControlMessageProtocolVersion6Type::MulticastListenerQuery => visitor.multicast_listener_query(unsafe { self.code.multicast_listener_query }),
			InternetControlMessageProtocolVersion6Type::MulticastListenerReport => visitor.multicast_listener_report(unsafe { self.code.multicast_listener_report }),
			InternetControlMessageProtocolVersion6Type::MulticastListenerDone => visitor.multicast_listener_done(unsafe { self.code.multicast_listener_done }),
			InternetControlMessageProtocolVersion6Type::RouterSolicitation => visitor.router_solicitation(unsafe { self.code.router_solicitation }),
			InternetControlMessageProtocolVersion6Type::RouterAdvertisement => visitor.router_advertisement(unsafe { self.code.router_advertisement }),
			InternetControlMessageProtocolVersion6Type::NeighborSolicitation => visitor.neighbor_solicitation(unsafe { self.code.neighbor_solicitation }),
			InternetControlMessageProtocolVersion6Type::NeighborAdvertisement => visitor.neighbor_advertisement(unsafe { self.code.neighbor_advertisement }),
			InternetControlMessageProtocolVersion6Type::Redirect => visitor.redirect(unsafe { self.code.redirect }),
			InternetControlMessageProtocolVersion6Type::RouterRenumbering => visitor.router_renumbering(unsafe { self.code.router_renumbering }),
			InternetControlMessageProtocolVersion6Type::NodeInformationQuery => visitor.node_information_query(unsafe { self.code.node_information_query }),
			InternetControlMessageProtocolVersion6Type::NodeInformationResponse => visitor.node_information_response(unsafe { self.code.node_information_response }),
			InternetControlMessageProtocolVersion6Type::InverseNeighborDiscoverySolicitation => visitor.inverse_neighbor_discovery_solicitation(unsafe { self.code.inverse_neighbor_discovery_solicitation }),
			InternetControlMessageProtocolVersion6Type::InverseNeighborDiscoveryAdvertisement => visitor.inverse_neighbor_discovery_advertisement(unsafe { self.code.inverse_neighbor_discovery_advertisement }),
			InternetControlMessageProtocolVersion6Type::Version2MulticastListenerReport => visitor.version_2_multicast_listener_report(unsafe { self.code.version_2_multicast_listener_report }),
			InternetControlMessageProtocolVersion6Type::HomeAgentAddressDiscoveryRequest => visitor.home_agent_address_discovery_request(unsafe { self.code.home_agent_address_discovery_request }),
			InternetControlMessageProtocolVersion6Type::HomeAgentAddressDiscoveryReply => visitor.home_agent_address_discovery_reply(unsafe { self.code.home_agent_address_discovery_reply }),
			InternetControlMessageProtocolVersion6Type::MobilePrefixSolicitation => visitor.mobile_prefix_solicitation(unsafe { self.code.mobile_prefix_solicitation }),
			InternetControlMessageProtocolVersion6Type::MobilePrefixAdvertisement => visitor.mobile_prefix_advertisement(unsafe { self.code.mobile_prefix_advertisement }),
			InternetControlMessageProtocolVersion6Type::CertificationPathSolicitation => visitor.certification_path_solicitation(unsafe { self.code.certification_path_solicitation }),
			InternetControlMessageProtocolVersion6Type::CertificationPathAdvertisement => visitor.certification_path_advertisement(unsafe { self.code.certification_path_advertisement }),
			InternetControlMessageProtocolVersion6Type::ExperimentalMobilityProtocol => visitor.experimental_mobility_protocol(unsafe { self.code.experimental_mobility_protocol }),
			InternetControlMessageProtocolVersion6Type::MulticastRouterAdvertisement => visitor.multicast_router_advertisement(unsafe { self.code.multicast_router_advertisement }),
			InternetControlMessageProtocolVersion6Type::MulticastRouterSolicitation => visitor.multicast_router_solicitation(unsafe { self.code.multicast_router_solicitation }),
			InternetControlMessageProtocolVersion6Type::MulticastRouterTermination => visitor.multicast_router_termination(unsafe { self.code.multicast_router_termination }),
			InternetControlMessageProtocolVersion6Type::FastHandoversForMobileInternetProtocolVersion6 => visitor.fast_handovers_for_mobile_internet_protocol_version_6(unsafe { self.code.fast_handovers_for_mobile_internet_protocol_version_6 }),
			InternetControlMessageProtocolVersion6Type::RoutingProtocolForLowPowerAndLossyNetworksControl => visitor.routing_protocol_for_low_power_and_lossy_networks_control(unsafe { self.code.routing_protocol_for_low_power_and_lossy_networks_control }),
			InternetControlMessageProtocolVersion6Type::IdentifierLocatorNetworkProtocolForInternetProtocolVersion6LocatorUpdate => visitor.identifier_locator_network_protocol_for_internet_protocol_version_6_locator_update(unsafe { self.code.identifier_locator_network_protocol_for_internet_protocol_version_6_locator_update }),
			InternetControlMessageProtocolVersion6Type::DuplicateAddressRequest => visitor.duplicate_address_request(unsafe { self.code.duplicate_address_request }),
			InternetControlMessageProtocolVersion6Type::DuplicateAddressConfirmation => visitor.duplicate_address_confirmation(unsafe { self.code.duplicate_address_confirmation }),
			InternetControlMessageProtocolVersion6Type::MulticastProtocolForLowPowerAndLossyNetworksControl => visitor.multicast_protocol_for_low_power_and_lossy_networks_control(unsafe { self.code.multicast_protocol_for_low_power_and_lossy_networks_control }),
			InternetControlMessageProtocolVersion6Type::ExtendedEchoRequest => visitor.extended_echo_request(unsafe { self.code.extended_echo_request }),
			InternetControlMessageProtocolVersion6Type::ExtendedEchoReply => visitor.extended_echo_reply(unsafe { self.code.extended_echo_reply }),
			InternetControlMessageProtocolVersion6Type::PrivateInformationalMessageExperiment1 => visitor.private_informational_message_experiment1(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::PrivateInformationalMessageExperiment2 => visitor.private_informational_message_experiment2(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::ReservedForExpansionOfInformationalMessageRange => visitor.reserved_for_expansion_of_informational_message_range(unsafe { self.code.undifferentiated }),
			_ => visitor.unknown(unsafe { self.code.undifferentiated })
		}
	}
}
