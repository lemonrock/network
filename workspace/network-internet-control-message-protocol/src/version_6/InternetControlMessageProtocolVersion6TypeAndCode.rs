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
	
//	/// Internet Control Message Protocol (ICMP) version 6 node information query.
//	fn internet_control_message_protocol_node_information_query(&self, internet_control_message_protocol_node_information_query: InternetControlMessageProtocolVersion6CodeInternetControlMessageProtocolNodeInformationQuery) -> R;
//
//	/// Internet Control Message Protocol (ICMP) version 6 node information response.
//	fn internet_control_message_protocol_node_information_response(&self, internet_control_message_protocol_node_information_response: InternetControlMessageProtocolVersion6CodeInternetControlMessageProtocolNodeInformationResponse) -> R;
//
//	/// Inverse neighbor discovery solicitation.
//	fn inverse_neighbor_discovery_solicitation(&self, inverse_neighbor_discovery_solicitation: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoverySolicitation) -> R;
//
//	/// Inverse neighbor discovery advertisement.
//	fn inverse_neighbor_discovery_advertisement(&self, inverse_neighbor_discovery_advertisement: InternetControlMessageProtocolVersion6CodeInverseNeighborDiscoveryAdvertisement) -> R;
//
//	/// Version 2 multicast listener report.
//	fn version2_multicast_listener_report(&self, version2_multicast_listener_report: InternetControlMessageProtocolVersion6CodeVersion2MulticastListenerReport) -> R;
//
//	/// Home agent address discovery request.
//	fn home_agent_address_discovery_request(&self, home_agent_address_discovery_request: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryRequest) -> R;
//
//	/// Home agent address discovery reply.
//	fn home_agent_address_discovery_reply(&self, home_agent_address_discovery_reply: InternetControlMessageProtocolVersion6CodeHomeAgentAddressDiscoveryReply) -> R;
//
//	/// Mobile prefix solicitation.
//	fn mobile_prefix_solicitation(&self, mobile_prefix_solicitation: InternetControlMessageProtocolVersion6CodeMobilePrefixSolicitation) -> R;
//
//	/// Mobile prefix advertisement.
//	fn mobile_prefix_advertisement(&self, mobile_prefix_advertisement: InternetControlMessageProtocolVersion6CodeMobilePrefixAdvertisement) -> R;
//
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
//
//	/// Extended echo request.
//	fn extended_echo_request(&self, extended_echo_request: InternetControlMessageProtocolVersion6CodeExtendedEchoRequest) -> R;
//
//	/// Extended echo reply.
//	fn extended_echo_reply(&self, extended_echo_reply: InternetControlMessageProtocolVersion6CodeExtendedEchoReply) -> R;

	/// Private informational message experiment1.
	fn private_informational_message_experiment1(&self, private_informational_message_experiment1: u8) -> R;

	/// Private informational message experiment2.
	fn private_informational_message_experiment2(&self, private_informational_message_experiment2: u8) -> R;

	/// Reserved for expansion of informational message range.
	fn reserved_for_expansion_of_informational_message_range(&self, reserved_for_expansion_of_informational_message_range: u8) -> R;
	
	/// Unknown.
	fn unknown(&self, unknown: u8) -> R;
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
			InternetControlMessageProtocolVersion6Type::RedirectMessage => visitor.redirect_message(unsafe { self.code.redirect_message }),
			InternetControlMessageProtocolVersion6Type::RouterRenumbering => visitor.router_renumbering(unsafe { self.code.router_renumbering }),
//			InternetControlMessageProtocolVersion6Type::InternetControlMessageProtocolNodeInformationQuery => visitor.internet_control_message_protocol_node_information_query(unsafe { self.code.internet_control_message_protocol_node_information_query }),
//			InternetControlMessageProtocolVersion6Type::InternetControlMessageProtocolNodeInformationResponse => visitor.internet_control_message_protocol_node_information_response(unsafe { self.code.internet_control_message_protocol_node_information_response }),
//			InternetControlMessageProtocolVersion6Type::InverseNeighborDiscoverySolicitation => visitor.inverse_neighbor_discovery_solicitation(unsafe { self.code.inverse_neighbor_discovery_solicitation }),
//			InternetControlMessageProtocolVersion6Type::InverseNeighborDiscoveryAdvertisement => visitor.inverse_neighbor_discovery_advertisement(unsafe { self.code.inverse_neighbor_discovery_advertisement }),
//			InternetControlMessageProtocolVersion6Type::Version2MulticastListenerReport => visitor.version2_multicast_listener_report(unsafe { self.code.version2_multicast_listener_report }),
//			InternetControlMessageProtocolVersion6Type::HomeAgentAddressDiscoveryRequest => visitor.home_agent_address_discovery_request(unsafe { self.code.home_agent_address_discovery_request }),
//			InternetControlMessageProtocolVersion6Type::HomeAgentAddressDiscoveryReply => visitor.home_agent_address_discovery_reply(unsafe { self.code.home_agent_address_discovery_reply }),
//			InternetControlMessageProtocolVersion6Type::MobilePrefixSolicitation => visitor.mobile_prefix_solicitation(unsafe { self.code.mobile_prefix_solicitation }),
//			InternetControlMessageProtocolVersion6Type::MobilePrefixAdvertisement => visitor.mobile_prefix_advertisement(unsafe { self.code.mobile_prefix_advertisement }),
//			InternetControlMessageProtocolVersion6Type::CertificationPathSolicitation => visitor.certification_path_solicitation(unsafe { self.code.certification_path_solicitation }),
//			InternetControlMessageProtocolVersion6Type::CertificationPathAdvertisement => visitor.certification_path_advertisement(unsafe { self.code.certification_path_advertisement }),
//			InternetControlMessageProtocolVersion6Type::ExperimentalMobilityProtocol => visitor.experimental_mobility_protocol(unsafe { self.code.experimental_mobility_protocol }),
//			InternetControlMessageProtocolVersion6Type::MulticastRouterAdvertisement => visitor.multicast_router_advertisement(unsafe { self.code.multicast_router_advertisement }),
//			InternetControlMessageProtocolVersion6Type::MulticastRouterSolicitation => visitor.multicast_router_solicitation(unsafe { self.code.multicast_router_solicitation }),
//			InternetControlMessageProtocolVersion6Type::MulticastRouterTermination => visitor.multicast_router_termination(unsafe { self.code.multicast_router_termination }),
//			InternetControlMessageProtocolVersion6Type::FMIPv6Messages => visitor.f_m_i_pv6_messages(unsafe { self.code.f_m_i_pv6_messages }),
//			InternetControlMessageProtocolVersion6Type::RPLControl => visitor.r_p_l_control(unsafe { self.code.r_p_l_control }),
//			InternetControlMessageProtocolVersion6Type::ILNPv6LocatorUpdate => visitor.i_l_n_pv6_locator_update(unsafe { self.code.i_l_n_pv6_locator_update }),
//			InternetControlMessageProtocolVersion6Type::DuplicateAddressRequest => visitor.duplicate_address_request(unsafe { self.code.duplicate_address_request }),
//			InternetControlMessageProtocolVersion6Type::DuplicateAddressConfirmation => visitor.duplicate_address_confirmation(unsafe { self.code.duplicate_address_confirmation }),
//			InternetControlMessageProtocolVersion6Type::MPLControl => visitor.m_p_l_control(unsafe { self.code.m_p_l_control }),
//			InternetControlMessageProtocolVersion6Type::ExtendedEchoRequest => visitor.extended_echo_request(unsafe { self.code.extended_echo_request }),
//			InternetControlMessageProtocolVersion6Type::ExtendedEchoReply => visitor.extended_echo_reply(unsafe { self.code.extended_echo_reply }),
			InternetControlMessageProtocolVersion6Type::PrivateInformationalMessageExperiment1 => visitor.private_informational_message_experiment1(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::PrivateInformationalMessageExperiment2 => visitor.private_informational_message_experiment2(unsafe { self.code.undifferentiated }),
			InternetControlMessageProtocolVersion6Type::ReservedForExpansionOfInformationalMessageRange => visitor.reserved_for_expansion_of_informational_message_range(unsafe { self.code.undifferentiated }),
			_ => visitor.unknown(unsafe { self.code.undifferentiated })
		}
	}
}

/*
DestinationUnreachable	destination_unreachable
PacketTooBig	packet_too_big
TimeExceeded	time_exceeded
ParameterProblem	parameter_problem
PrivateErrorMessageExperiment1	private_error_message_experiment1
PrivateErrorMessageExperiment2	private_error_message_experiment2
ReservedForExpansionOfErrorMessageRange	reserved_for_expansion_of_error_message_range
EchoRequest	echo_request
EchoReply	echo_reply
MulticastListenerQuery	multicast_listener_query
MulticastListenerReport	multicast_listener_report
MulticastListenerDone	multicast_listener_done
RouterSolicitation	router_solicitation
RouterAdvertisement	router_advertisement
NeighborSolicitation	neighbor_solicitation
NeighborAdvertisement	neighbor_advertisement
RedirectMessage	redirect_message
RouterRenumbering	router_renumbering
InternetControlMessageProtocolNodeInformationQuery	internet_control_message_protocol_node_information_query
InternetControlMessageProtocolNodeInformationResponse	internet_control_message_protocol_node_information_response
InverseNeighborDiscoverySolicitation	inverse_neighbor_discovery_solicitation
InverseNeighborDiscoveryAdvertisement	inverse_neighbor_discovery_advertisement
Version2MulticastListenerReport	version2_multicast_listener_report
HomeAgentAddressDiscoveryRequest	home_agent_address_discovery_request
HomeAgentAddressDiscoveryReply	home_agent_address_discovery_reply
MobilePrefixSolicitation	mobile_prefix_solicitation
MobilePrefixAdvertisement	mobile_prefix_advertisement
CertificationPathSolicitation	certification_path_solicitation
CertificationPathAdvertisement	certification_path_advertisement
ExperimentalMobilityProtocol	experimental_mobility_protocol
MulticastRouterAdvertisement	multicast_router_advertisement
MulticastRouterSolicitation	multicast_router_solicitation
MulticastRouterTermination	multicast_router_termination
FMIPv6Messages	f_m_i_pv6_messages
RPLControl	r_p_l_control
ILNPv6LocatorUpdate	i_l_n_pv6_locator_update
DuplicateAddressRequest	duplicate_address_request
DuplicateAddressConfirmation	duplicate_address_confirmation
MPLControl	m_p_l_control
ExtendedEchoRequest	extended_echo_request
ExtendedEchoReply	extended_echo_reply
PrivateInformationalMessageExperiment1	private_informational_message_experiment1
PrivateInformationalMessageExperiment2	private_informational_message_experiment2
ReservedForExpansionOfInformationalMessageRange	reserved_for_expansion_of_informational_message_range
*/
