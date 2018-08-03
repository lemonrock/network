// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents an Internet Protocol (IP) version 4 option number, 0 to 31.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct InternetProtocolVersion4OptionNumber(u8);

impl TryFrom<u8> for InternetProtocolVersion4OptionNumber
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if value > InternetProtocolVersion4OptionKind::FiveBitNumberField
		{
			Err(())
		}
		else
		{
			Ok(InternetProtocolVersion4OptionKind(value))
		}
	}
}

impl Into<u8> for InternetProtocolVersion4OptionKind
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl InternetProtocolVersion4OptionNumber
{
	/// End of options list.
	pub const EndOfOptionsList: Self = InternetProtocolVersion4OptionNumber(0);
	
	/// No operation.
	pub const NoOperation: Self = InternetProtocolVersion4OptionNumber(1);
	
	/// DoD Basic Security (SEC).
	///
	/// Still required in rare circumstance as of RFC 7126.
	pub const BasicSecurity: Self = InternetProtocolVersion4OptionKind(2);
	
	/// Loose Source and Record Route (LSRR or LSR).
	///
	/// Threat as of RFC 7126.
	pub const LooseSourceRouteAndRecordRoute: Self = InternetProtocolVersion4OptionKind(3);
	
	/// Internet Timestamp (TS).
	///
	/// Threat as of RFC 7126.
	pub const InternetTimestamp: Self = InternetProtocolVersion4OptionKind(4);
	
	/// DoD Extended Security (E-SEC).
	///
	/// Still required in rare circumstance as of RFC 7126.
	pub const ExtendedSecurity: Self = InternetProtocolVersion4OptionKind(5);
	
	/// Commercial IP Security (CIPSO).
	///
	/// Still required in rare circumstance as of RFC 7126.
	pub const CommercialSecurity: Self = InternetProtocolVersion4OptionKind(6);
	
	/// Record Route (RR).
	///
	/// Threat as of RFC 7126.
	pub const RecordRoute: Self = InternetProtocolVersion4OptionKind(7);
	
	/// Stream Identifier (SID).
	///
	/// Obsolete as of RFC 7126.
	pub const StreamIdentifier: Self = InternetProtocolVersion4OptionKind(8);
	
	/// Strict Source and Record Route (SSRR or SSR).
	///
	/// Threat as of RFC 7126.
	pub const StrictSourceRouteAndRecordRoute: Self = InternetProtocolVersion4OptionKind(9);
	
	/// Experimental Measurement (ZSU).
	///
	/// Extremely rare, if used at all.
	pub const ExperimentalMeasurement: Self = InternetProtocolVersion4OptionKind(10);
	
	/// Probe MTU (MTUP).
	///
	/// Obsolete as of RFC 7126.
	pub const ProbeMaximumTransmissionUnit: Self = InternetProtocolVersion4OptionKind(11);
	
	/// Reply MTU (MTUR).
	///
	/// Obsolete as of RFC 7126.
	pub const ReplyMaximumTransmissionUnit: Self = InternetProtocolVersion4OptionKind(12);
	
	/// Experimental Flow Control (FINN).
	///
	/// Extremely rare, if used at all.
	pub const ExperimentalFlowControl: Self = InternetProtocolVersion4OptionKind(13);
	
	/// Experimental Access Control (VISA).
	///
	/// Obsolete as of RFC 6814.
	pub const ExperimentalAccessControl: Self = InternetProtocolVersion4OptionKind(14);
	
	/// ENCODE (ENCODE).
	///
	/// Obsolete as of RFC 6814.
	pub const ENCODE: Self = InternetProtocolVersion4OptionKind(15);
	
	/// IMI Traffic Descriptor (IMITD).
	///
	/// Extremely rare, if used at all.
	pub const ImiTrafficDescriptor: Self = InternetProtocolVersion4OptionKind(16);
	
	/// Extended Internet Protocol (EIP).
	///
	/// Obsolete as of RFC 6814.
	pub const ExtendedInternetProtocol: Self = InternetProtocolVersion4OptionKind(17);
	
	/// Traceroute (TR).
	///
	/// Obsolete as of RFC 6814.
	pub const Traceroute: Self = InternetProtocolVersion4OptionKind(18);
	
	/// Address Extension (ADDEXT).
	///
	/// Obsolete as of RFC 6814.
	pub const AddressExtension: Self = InternetProtocolVersion4OptionKind(19);
	
	/// Router Alert (RTRALT).
	///
	/// Threat as of RFC 7126.
	pub const RouterAlert: Self = InternetProtocolVersion4OptionKind(20);
	
	/// Sender Directed Multi-Destination Delivery (SDB).
	///
	/// Also known as Selective Directed Broadcast.
	///
	/// Obsolete as of RFC 6814.
	pub const SenderDirectedMultiDestinationDelivery: Self = InternetProtocolVersion4OptionKind(21);
	
	/// Type = 150 (unassigned but previously in use until 2005).
	///
	/// Extremely rare, if used at all.
	pub const _150: Self = InternetProtocolVersion4OptionKind(22);
	
	/// Dynamic Packet State (DPS).
	///
	/// Obsolete as of RFC 6814.
	pub const DynamicPacketState: Self = InternetProtocolVersion4OptionKind(23);
	
	/// Upstream Multicast Packet (UMP).
	///
	/// Obsolete as of RFC 6814.
	pub const UpstreamMulticastPacket: Self = InternetProtocolVersion4OptionKind(24);
	
	/// Quick-Start (QS).
	///
	/// Threat as of RFC 7126.
	pub const QuickStart: Self = InternetProtocolVersion4OptionKind(25);
	
	/// RFC 3692 style Experiment (EXP) defined in RFC 4727.
	pub const Rfc3692StyleExperiment: Self = InternetProtocolVersion4OptionKind(30);
	
	/// Is this number assigned at [IANA](https://www.iana.org/assignments/ip-parameters/ip-parameters.xhtml#ip-parameters-1) or was it previously assigned at IANA?
	#[inline(always)]
	pub fn is_assigned_or_previously_assigned(self) -> bool
	{
		self.0 <= 25 || self.0 == 30
	}
}
