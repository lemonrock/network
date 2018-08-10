// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A visitor to deal with the type and code 'tagged enumeration'.
///
/// `R` is the result of a visit.
pub trait InternetControlMessageProtocolVersion6TypeAndCodeVisitor<R>
{
	/// Destination unreachable (RFC 4443).
	fn destination_unreachable(&self, destination_unreachable: InternetControlMessageProtocolVersion6CodeDestinationUnreachable) -> R;
	
	/// Packet too big (RFC 4443).
	fn packet_too_big(&self, packet_too_big: InternetControlMessageProtocolVersion6CodePacketTooBig) -> R;
	
	/// Time exceeded (RFC 4443).
	fn time_exceeded(&self, time_exceeded: InternetControlMessageProtocolVersion6CodeTimeExceeded) -> R;
	
	/// Parameter problem (RFC 4443).
	fn parameter_problem(&self, parameter_problem: InternetControlMessageProtocolVersion6CodeParameterProblem) -> R;
	
	/// Private error message experiment 1 (RFC 4443).
	fn private_error_message_experiment1(&self, private_error_message_experiment1: u8) -> R;
	
	/// Private error message experiment 2 (RFC 4443).
	fn private_error_message_experiment2(&self, private_error_message_experiment2: u8) -> R;
	
	/// Reserved for expansion of error message range (RFC 4443).
	fn reserved_for_expansion_of_error_message_range(&self, reserved_for_expansion_of_error_message_range: u8) -> R;
	
	/// Echo request (RFC 4443).
	fn echo_request(&self, echo_request: InternetControlMessageProtocolVersion6CodeEchoRequest) -> R;
	
	/// Echo Reply (RFC 4443).
	fn echo_reply(&self, echo_reply: InternetControlMessageProtocolVersion6CodeEchoReply) -> R;
	
	/// Multicast Listener Query (RFC 3810).
	fn multicast_listener_query(&self, multicast_listener_query: InternetControlMessageProtocolVersion6CodeMulticastListenerQuery) -> R;
	
	/// Multicast Listener Report (RFC 3810).
	fn multicast_listener_report(&self, multicast_listener_report: InternetControlMessageProtocolVersion6CodeMulticastListenerReport) -> R;
	
	/// Multicast Listener Done (RFC 3810).
	fn multicast_listener_done(&self, multicast_listener_done: InternetControlMessageProtocolVersion6CodeMulticastListenerDone) -> R;
	
	/// Router Solicitation (RFC 4861).
	fn router_solicitation(&self, router_solicitation: InternetControlMessageProtocolVersion6CodeRouterSolicitation) -> R;
	
	/// Router Advertisement (RFC 4861).
	fn router_advertisement(&self, router_advertisement: InternetControlMessageProtocolVersion6CodeRouterAdvertisement) -> R;
	
	/// Neighbor Solicitation (RFC 4861).
	fn neighbor_solicitation(&self, neighbor_solicitation: InternetControlMessageProtocolVersion6CodeNeighborSolicitation) -> R;
	
	/// Neighbor Advertisement (RFC 4861).
	fn neighbor_advertisement(&self, neighbor_advertisement: InternetControlMessageProtocolVersion6CodeNeighborAdvertisement) -> R;
	
	/// Redirect (RFC 4861).
	fn redirect(&self, redirect: InternetControlMessageProtocolVersion6CodeRedirect) -> R;
	
	/// Router Renumbering (RFC 2894).
	fn router_renumbering(&self, router_renumbering: InternetControlMessageProtocolVersion6CodeRouterRenumbering) -> R;
	
	/// Node Information Query (RFC 4620).
	fn node_information_query(&self, node_information_query: InternetControlMessageProtocolVersion6CodeNodeInformationQuery) -> R;

	/// Node Information Response (RFC 4620).
	fn node_information_response(&self, node_information_response: InternetControlMessageProtocolVersion6CodeNodeInformationResponse) -> R;

	/// Inverse Neighbor Discovery Solicitation (RFC 3122).
	fn inverse_neighbor_discovery_solicitation(&self, inverse_neighbor_discovery_solicitation: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation) -> R;

	/// Inverse Neighbor Discovery Advertisement (RFC 3122).
	fn inverse_neighbor_discovery_advertisement(&self, inverse_neighbor_discovery_advertisement: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoveryAdvertisement) -> R;

	/// Version 2 Multicast Listener Report (RFC 3810).
	fn version_2_multicast_listener_report(&self, version2_multicast_listener_report: InternetControlMessageProtocolVersion6CodeVersion2MulticastListenerReport) -> R;

	/// Home Agent Address Discovery request (RFC 3775).
	fn home_agent_address_discovery_request(&self, home_agent_address_discovery_request: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryRequest) -> R;

	/// Home Agent Address Discovery Reply (RFC 3775).
	fn home_agent_address_discovery_reply(&self, home_agent_address_discovery_reply: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryReply) -> R;

	/// Mobile Prefix Solicitation (RFC 3775).
	fn mobile_prefix_solicitation(&self, mobile_prefix_solicitation: InternetControlMessageProtocolVersion6CodeMobilePrefixSolicitation) -> R;

	/// Mobile Prefix Advertisement (RFC 3775).
	fn mobile_prefix_advertisement(&self, mobile_prefix_advertisement: InternetControlMessageProtocolVersion6CodeMobilePrefixAdvertisement) -> R;

	/// Certification Path Solicitation (RFC 3971).
	fn certification_path_solicitation(&self, certification_path_solicitation: InternetControlMessageProtocolVersion6CodeCertificationPathSolicitation) -> R;

	/// Certification Path Advertisement (RFC 3971).
	fn certification_path_advertisement(&self, certification_path_advertisement: InternetControlMessageProtocolVersion6CodeCertificationPathAdvertisement) -> R;

	/// Utilized by experimental mobility protocols such as Seamoby (RFC 4065).
	fn experimental_mobility_protocol(&self, experimental_mobility_protocol: InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol) -> R;

	/// Multicast Router Advertisement (RFC 4286).
	fn multicast_router_advertisement(&self, multicast_router_advertisement: InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement) -> R;

	/// Multicast Router Solicitation (RFC 4286).
	fn multicast_router_solicitation(&self, multicast_router_solicitation: InternetControlMessageProtocolVersion6CodeMulticastRouterSolicitation) -> R;

	/// Multicast Router Termination (RFC 4286).
	fn multicast_router_termination(&self, multicast_router_termination: InternetControlMessageProtocolVersion6CodeMulticastRouterTermination) -> R;

	/// Fast handovers for Mobile Internet Protocol (IP) version 6, aka FMIPv6 (RFC 5568).
	fn fast_handovers_for_mobile_internet_protocol_version_6(&self, fast_handovers_for_mobile_internet_protocol_version_6: InternetControlMessageProtocolVersion6CodeFastHandoversForMobileInternetProtocolVersion6) -> R;

	/// Internet Protocol (IP) version 6 Routing Protocol for Low-Power and Lossy Networks Control (RPL) (RFC 6550).
	fn routing_protocol_for_low_power_and_lossy_networks_control(&self, routing_protocol_for_low_power_and_lossy_networks_control: InternetControlMessageProtocolVersion6CodeRoutingProtocolForLowPowerAndLossyNetworksControl) -> R;

	/// Identifier-Locator Network Protocol for Internet Protocol (IP) version 6 (ILNPv6) Locator Update (RFC 6743).
	fn identifier_locator_network_protocol_for_internet_protocol_version_6_locator_update(&self, identifier_locator_network_protocol_for_internet_protocol_version_6_locator_update: InternetControlMessageProtocolVersion6CodeIdentifierLocatorNetworkProtocolForInternetProtocolVersion6LocatorUpdate) -> R;

	/// Duplicate Address Request (RFC 6775).
	fn duplicate_address_request(&self, duplicate_address_request: InternetControlMessageProtocolVersion6CodeDuplicateAddressRequest) -> R;

	/// Duplicate Address Confirmation (RFC 6775).
	fn duplicate_address_confirmation(&self, duplicate_address_confirmation: InternetControlMessageProtocolVersion6CodeDuplicateAddressConfirmation) -> R;

//	/// MPL Control (RFC 7731).
//	fn m_p_l_control(&self, m_p_l_control: InternetControlMessageProtocolVersion6CodeMPLControl) -> R;

	/// Extended Echo Request (RFC 8335).
	fn extended_echo_request(&self, extended_echo_request: InternetControlMessageProtocolVersion6CodeExtendedEchoRequest) -> R;

	/// Extended Echo Reply (RFC 8335).
	fn extended_echo_reply(&self, extended_echo_reply: InternetControlMessageProtocolVersion6CodeExtendedEchoReply) -> R;

	/// Private informational message experiment 1 (RFC 4443).
	fn private_informational_message_experiment1(&self, private_informational_message_experiment1: u8) -> R;

	/// Private informational message experiment 2 (RFC 4443).
	fn private_informational_message_experiment2(&self, private_informational_message_experiment2: u8) -> R;

	/// Reserved for expansion of informational message range (RFC 4443).
	fn reserved_for_expansion_of_informational_message_range(&self, reserved_for_expansion_of_informational_message_range: u8) -> R;
	
	/// Unknown.
	fn unknown(&self, unknown: u8) -> R;
}
