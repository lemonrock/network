// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents known Internet Control Message Protocol (ICMP) Neighbor Discovery Option (RFC 4861) types.
///
/// See [IANA](https://www.iana.org/assignments/icmpv6-parameters/icmpv6-parameters.xhtml#icmpv6-parameters-5) for a complete list.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(C, packed)]
pub struct KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(u8);

impl Display for KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<u8> for KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl From<u8> for KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(value)
	}
}

impl KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType
{
	/// Source Link-layer Address (RFC 4861).
	pub const SourceLinkLayerAddress: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(1);
	
	/// Target Link-layer Address (RFC 4861).
	pub const TargetLinkLayerAddress: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(2);
	
	/// Prefix Information (RFC 4861).
	pub const PrefixInformation: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(3);
	
	/// Redirected Header (RFC 4861).
	pub const RedirectedHeader: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(4);
	
	/// Maximu mTransmission Unit (MTU) (RFC 4861).
	pub const MaximumTransmissionUnit: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(5);
	
	/// Non-Broadcast Multiple Access (NBMA) Shortcut Limit (RFC 2491).
	pub const NonBroadcastMultipleAccessShortcutLimit: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(6);
	
	/// AdvertisementInterval (RFC 6275).
	pub const AdvertisementInterval: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(7);
	
	/// Home Agent Information (RFC 6275).
	pub const HomeAgentInformation: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(8);
	
	/// Source Address List (RFC 3122).
	pub const SourceAddressList: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(9);
	
	/// Target Address List (RFC 3122).
	pub const TargetAddressList: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(10);
	
	/// Cryptrographically Generated Address (CGA) (RFC 3971).
	pub const CryptrographicallyGeneratedAddress: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(11);
	
	/// Rivest–Shamir–Adleman (RSA) Signature (RFC 3971).
	pub const RivestShamirAdlemanSignature: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(12);
	
	/// Timestamp (RFC 3971).
	pub const Timestamp: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(13);
	
	/// Nonce (RFC 3971).
	pub const Nonce: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(14);
	
	/// Trust Anchor (RFC 3971).
	pub const TrustAnchor: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(15);
	
	/// Certificate (RFC 3971).
	pub const Certificate: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(16);
	
	/// Internet Protocol (IP) version 6 Address or Prefix (RFC 5568).
	pub const InternetProtocolVersion6AddressPrefix: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(17);
	
	/// New Router Prefix Information (RFC 4068).
	pub const NewRouterPrefixInformation: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(18);
	
	/// Link-layer Address (RFC 5568).
	pub const LinkLayerAddress: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(19);
	
	/// Neighbor Advertisement Acknowledgment (RFC 5568).
	pub const NeighborAdvertisementAcknowledgment: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(20);
	
	/// Formerly, PvD ID Router Advertisement (reclaimable in future) (draft-ietf-intarea-provisioning-domains).
	pub const FormerlyPvDIDRouterAdvertisement: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(21);
	
	/// Mobility Anchor Point (MAP) (RFC 4140).
	pub const MobilityAnchorPoint: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(23);
	
	/// Route Information (RFC 4191).
	pub const RouteInformation: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(24);
	
	/// Recursive Domain Name System (DNS) Server (RFC 5006 and RFC 8106).
	pub const RecursiveDomainNameSystemServer: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(25);
	
	/// Router Advertisement (RA) Flags Extension (RFC 5175).
	pub const RouterAdvertisementFlagsExtension: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(26);
	
	/// Handover Key Request (RFC 5269).
	pub const HandoverKeyRequest: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(27);
	
	/// Handover Key Reply (RFC 5269).
	pub const HandoverKeyReply: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(28);
	
	/// Handover Assist Information (RFC 5271).
	pub const HandoverAssistInformation: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(29);
	
	/// Mobile Node Identifier (RFC 5271).
	pub const MobileNodeIdentifier: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(30);
	
	/// Domain Name System (DNS) Search List (RFC 8106).
	pub const DomainNameSystemSearchList: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(31);
	
	/// Proxy Signature (PS) (RFC 6496).
	pub const ProxySignature: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(32);
	
	/// Address Registration (RFC 6775).
	pub const AddressRegistration: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(33);
	
	/// 6LoWPAN Context (RFC 6775).
	pub const _6LoWPANContext: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(34);
	
	/// Authoritative Border Router (RFC 6775).
	pub const AuthoritativeBorderRouter: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(35);
	
	/// 6LoWPAN Capability Indication (6CIO). (RFC 7400).
	pub const _6LoWPANCapabilityIndication: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(36);
	
	/// Dynamic Host Control Protocol (DHCP) Captive-Portal (RFC 7710)
	pub const DynamicHostControlProtocolCaptivePortal: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(37);
	
	/// Candidate Access Router Discovery Protocol (CARD) Request (RFC 4065).
	pub const CandidateAccessRouterDiscoveryProtocolRequest: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(138);
	
	/// Candidate Access Router Discovery Protocol (CARD) Reply (RFC 4065).
	pub const CandidateAccessRouterDiscoveryProtocolReply: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(139);
	
	/// RFC 3692-style Experiment 1 (RFC 4727).
	pub const Rfc3262StyleExperiment1: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(253);
	
	/// RFC 3692-style Experiment 2 (RFC 4727).
	pub const Rfc3262StyleExperiment2: Self = KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType(254);
}
