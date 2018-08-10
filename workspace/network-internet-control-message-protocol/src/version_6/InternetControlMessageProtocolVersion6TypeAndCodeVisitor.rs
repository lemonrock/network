// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A visitor to deal with the type and code 'tagged enumeration'.
///
/// `R` is the result of a visit.
pub trait InternetControlMessageProtocolVersion6TypeAndCodeVisitor<R>
{
	/// Destination unreachable.
	fn destination_unreachable(&self, destination_unreachable: InternetControlMessageProtocolVersion6CodeDestinationUnreachable) -> R;
	
	/// Packet too big.
	fn packet_too_big(&self, packet_too_big: InternetControlMessageProtocolVersion6CodePacketTooBig) -> R;
	
	/// Time exceeded.
	fn time_exceeded(&self, time_exceeded: InternetControlMessageProtocolVersion6CodeTimeExceeded) -> R;
	
	/// Parameter problem.
	fn parameter_problem(&self, parameter_problem: InternetControlMessageProtocolVersion6CodeParameterProblem) -> R;
	
	/// Private error message experiment 1.
	fn private_error_message_experiment1(&self, private_error_message_experiment1: u8) -> R;
	
	/// Private error message experiment 2.
	fn private_error_message_experiment2(&self, private_error_message_experiment2: u8) -> R;
	
	/// Reserved for expansion of error message range.
	fn reserved_for_expansion_of_error_message_range(&self, reserved_for_expansion_of_error_message_range: u8) -> R;
	
	/// Echo request.
	fn echo_request(&self, echo_request: InternetControlMessageProtocolVersion6CodeEchoRequest) -> R;
	
	/// Echo reply.
	fn echo_reply(&self, echo_reply: InternetControlMessageProtocolVersion6CodeEchoReply) -> R;
	
	/// Multicast listener query.
	fn multicast_listener_query(&self, multicast_listener_query: InternetControlMessageProtocolVersion6CodeMulticastListenerQuery) -> R;
	
	/// Multicast listener report.
	fn multicast_listener_report(&self, multicast_listener_report: InternetControlMessageProtocolVersion6CodeMulticastListenerReport) -> R;
	
	/// Multicast listener done.
	fn multicast_listener_done(&self, multicast_listener_done: InternetControlMessageProtocolVersion6CodeMulticastListenerDone) -> R;
	
	/// Router solicitation.
	fn router_solicitation(&self, router_solicitation: InternetControlMessageProtocolVersion6CodeRouterSolicitation) -> R;
	
	/// Router advertisement.
	fn router_advertisement(&self, router_advertisement: InternetControlMessageProtocolVersion6CodeRouterAdvertisement) -> R;
	
	/// Neighbor solicitation.
	fn neighbor_solicitation(&self, neighbor_solicitation: InternetControlMessageProtocolVersion6CodeNeighborSolicitation) -> R;
	
	/// Neighbor advertisement.
	fn neighbor_advertisement(&self, neighbor_advertisement: InternetControlMessageProtocolVersion6CodeNeighborAdvertisement) -> R;
	
	/// Redirect message.
	fn redirect_message(&self, redirect_message: InternetControlMessageProtocolVersion6CodeRedirectMessage) -> R;
	
	/// Router renumbering.
	fn router_renumbering(&self, router_renumbering: InternetControlMessageProtocolVersion6CodeRouterRenumbering) -> R;
	
	/// Node information query.
	fn node_information_query(&self, node_information_query: InternetControlMessageProtocolVersion6CodeNodeInformationQuery) -> R;

	/// Node information response.
	fn node_information_response(&self, node_information_response: InternetControlMessageProtocolVersion6CodeNodeInformationResponse) -> R;

	/// Inverse neighbor discovery solicitation.
	fn inverse_neighbor_discovery_solicitation(&self, inverse_neighbor_discovery_solicitation: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation) -> R;

	/// Inverse neighbor discovery advertisement.
	fn inverse_neighbor_discovery_advertisement(&self, inverse_neighbor_discovery_advertisement: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoveryAdvertisement) -> R;

//	/// Version 2 multicast listener report.
//	fn version2_multicast_listener_report(&self, version2_multicast_listener_report: InternetControlMessageProtocolVersion6CodeVersion2MulticastListenerReport) -> R;

	/// Home agent address discovery request.
	fn home_agent_address_discovery_request(&self, home_agent_address_discovery_request: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryRequest) -> R;

	/// Home agent address discovery reply.
	fn home_agent_address_discovery_reply(&self, home_agent_address_discovery_reply: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryReply) -> R;

	/// Mobile prefix solicitation.
	fn mobile_prefix_solicitation(&self, mobile_prefix_solicitation: InternetControlMessageProtocolVersion6CodeMobilePrefixSolicitation) -> R;

	/// Mobile prefix advertisement.
	fn mobile_prefix_advertisement(&self, mobile_prefix_advertisement: InternetControlMessageProtocolVersion6CodeMobilePrefixAdvertisement) -> R;

//	/// Certification path solicitation.
//	fn certification_path_solicitation(&self, certification_path_solicitation: InternetControlMessageProtocolVersion6CodeCertificationPathSolicitation) -> R;
//
//	/// Certification path advertisement.
//	fn certification_path_advertisement(&self, certification_path_advertisement: InternetControlMessageProtocolVersion6CodeCertificationPathAdvertisement) -> R;
//
//	/// Experimental mobility protocol.
//	fn experimental_mobility_protocol(&self, experimental_mobility_protocol: InternetControlMessageProtocolVersion6CodeExperimentalMobilityProtocol) -> R;
//
//	/// Multicast router advertisement.
//	fn multicast_router_advertisement(&self, multicast_router_advertisement: InternetControlMessageProtocolVersion6CodeMulticastRouterAdvertisement) -> R;
//
//	/// Multicast router solicitation.
//	fn multicast_router_solicitation(&self, multicast_router_solicitation: InternetControlMessageProtocolVersion6CodeMulticastRouterSolicitation) -> R;
//
//	/// Multicast router termination.
//	fn multicast_router_termination(&self, multicast_router_termination: InternetControlMessageProtocolVersion6CodeMulticastRouterTermination) -> R;
//
//	/// FMIPv6 messages.
//	fn f_m_i_pv6_messages(&self, f_m_i_pv6_messages: InternetControlMessageProtocolVersion6CodeFMIPv6Messages) -> R;
//
//	/// RPL Control.
//	fn r_p_l_control(&self, r_p_l_control: InternetControlMessageProtocolVersion6CodeRPLControl) -> R;
//
//	/// ILNPv6 locator update.
//	fn i_l_n_pv6_locator_update(&self, i_l_n_pv6_locator_update: InternetControlMessageProtocolVersion6CodeILNPv6LocatorUpdate) -> R;
//
//	/// Duplicate address request.
//	fn duplicate_address_request(&self, duplicate_address_request: InternetControlMessageProtocolVersion6CodeDuplicateAddressRequest) -> R;
//
//	/// Duplicate address confirmation.
//	fn duplicate_address_confirmation(&self, duplicate_address_confirmation: InternetControlMessageProtocolVersion6CodeDuplicateAddressConfirmation) -> R;
//
//	/// MPL Control.
//	fn m_p_l_control(&self, m_p_l_control: InternetControlMessageProtocolVersion6CodeMPLControl) -> R;

	/// Extended echo request.
	fn extended_echo_request(&self, extended_echo_request: InternetControlMessageProtocolVersion6CodeExtendedEchoRequest) -> R;

	/// Extended echo reply.
	fn extended_echo_reply(&self, extended_echo_reply: InternetControlMessageProtocolVersion6CodeExtendedEchoReply) -> R;

	/// Private informational message experiment1.
	fn private_informational_message_experiment1(&self, private_informational_message_experiment1: u8) -> R;

	/// Private informational message experiment2.
	fn private_informational_message_experiment2(&self, private_informational_message_experiment2: u8) -> R;

	/// Reserved for expansion of informational message range.
	fn reserved_for_expansion_of_informational_message_range(&self, reserved_for_expansion_of_informational_message_range: u8) -> R;
	
	/// Unknown.
	fn unknown(&self, unknown: u8) -> R;
}
