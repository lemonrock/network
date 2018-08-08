// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Represents an Internet Protocol (IP) version 4 option kind.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(transparent)]
pub struct InternetProtocolVersion4OptionKind(u8);

impl From<u8> for InternetProtocolVersion4OptionKind
{
	#[inline(always)]
	fn from(value: u8) -> Self
	{
		InternetProtocolVersion4OptionKind(value)
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

impl InternetProtocolVersion4OptionKind
{
	const OneBitCopyField: u8 = 0b1000_0000;
	
	const TwoBitClassField: u8 = 0b0110_0000;
	
	const FiveBitNumberField: u8 = 0b0001_1111;
	
	/// End of options list.
	pub const EndOfOptionsList: Self = InternetProtocolVersion4OptionKind(0);
	
	/// No operation.
	pub const NoOperation: Self = InternetProtocolVersion4OptionKind(1);
	
	/// DoD Basic Security (SEC) (Type = 130).
	///
	/// Still required in rare circumstance as of RFC 7126.
	pub const BasicSecurity: Self = InternetProtocolVersion4OptionKind(130);
	
	/// Loose Source and Record Route (LSRR or LSR) (Type = 131).
	///
	/// Threat as of RFC 7126.
	pub const LooseSourceRouteAndRecordRoute: Self = InternetProtocolVersion4OptionKind(131);
	
	/// Internet Timestamp (TS) (Type = 68).
	///
	/// Threat as of RFC 7126.
	pub const InternetTimestamp: Self = InternetProtocolVersion4OptionKind(68);
	
	/// DoD Extended Security (E-SEC) (Type = 133).
	///
	/// Still required in rare circumstance as of RFC 7126.
	pub const ExtendedSecurity: Self = InternetProtocolVersion4OptionKind(133);
	
	/// Commercial IP Security (CIPSO) (Type = 134).
	///
	/// Still required in rare circumstance as of RFC 7126.
	pub const CommercialSecurity: Self = InternetProtocolVersion4OptionKind(134);
	
	/// Record Route (RR) (Type = 7).
	///
	/// Threat as of RFC 7126.
	pub const RecordRoute: Self = InternetProtocolVersion4OptionKind(7);
	
	/// Stream Identifier (SID) (Type = 136).
	///
	/// Obsolete as of RFC 7126.
	pub const StreamIdentifier: Self = InternetProtocolVersion4OptionKind(136);
	
	/// Strict Source and Record Route (SSRR or SSR) (Type = 137).
	///
	/// Threat as of RFC 7126.
	pub const StrictSourceRouteAndRecordRoute: Self = InternetProtocolVersion4OptionKind(137);
	
	/// Experimental Measurement (ZSU) (Type = 10).
	///
	/// Extremely rare, if used at all.
	pub const ExperimentalMeasurement: Self = InternetProtocolVersion4OptionKind(10);
	
	/// Probe MTU (MTUP) (Type = 11).
	///
	/// Obsolete as of RFC 7126.
	pub const ProbeMaximumTransmissionUnit: Self = InternetProtocolVersion4OptionKind(11);
	
	/// Reply MTU (MTUR) (Type = 12).
	///
	/// Obsolete as of RFC 7126.
	pub const ReplyMaximumTransmissionUnit: Self = InternetProtocolVersion4OptionKind(12);
	
	/// Experimental Flow Control (FINN) (Type = 205).
	///
	/// Extremely rare, if used at all.
	pub const ExperimentalFlowControl: Self = InternetProtocolVersion4OptionKind(205);
	
	/// Experimental Access Control (VISA) (Type = 142).
	///
	/// Obsolete as of RFC 6814.
	pub const ExperimentalAccessControl: Self = InternetProtocolVersion4OptionKind(142);
	
	/// ENCODE (ENCODE) (Type = 15).
	///
	/// Obsolete as of RFC 6814.
	pub const ENCODE: Self = InternetProtocolVersion4OptionKind(15);
	
	/// IMI Traffic Descriptor (IMITD) (Type = 144).
	///
	/// Extremely rare, if used at all.
	pub const ImiTrafficDescriptor: Self = InternetProtocolVersion4OptionKind(144);
	
	/// Extended Internet Protocol (EIP) (Type = 145).
	///
	/// Obsolete as of RFC 6814.
	pub const ExtendedInternetProtocol: Self = InternetProtocolVersion4OptionKind(145);
	
	/// Traceroute (TR) (Type = 82).
	///
	/// Obsolete as of RFC 6814.
	pub const Traceroute: Self = InternetProtocolVersion4OptionKind(82);
	
	/// Address Extension (ADDEXT) (Type = 147).
	///
	/// Obsolete as of RFC 6814.
	pub const AddressExtension: Self = InternetProtocolVersion4OptionKind(147);
	
	/// Router Alert (RTRALT) (Type = 148).
	///
	/// Threat as of RFC 7126.
	pub const RouterAlert: Self = InternetProtocolVersion4OptionKind(148);
	
	/// Sender Directed Multi-Destination Delivery (SDB) (Type = 149).
	///
	/// Also known as Selective Directed Broadcast.
	///
	/// Obsolete as of RFC 6814.
	pub const SenderDirectedMultiDestinationDelivery: Self = InternetProtocolVersion4OptionKind(149);
	
	/// Type = 150 (unassigned but previously in use until 2005).
	///
	/// Extremely rare, if used at all.
	pub const _150: Self = InternetProtocolVersion4OptionKind(150);
	
	/// Dynamic Packet State (DPS) (Type = 151).
	///
	/// Obsolete as of RFC 6814.
	pub const DynamicPacketState: Self = InternetProtocolVersion4OptionKind(151);
	
	/// Upstream Multicast Packet (UMP) (Type = 152).
	///
	/// Obsolete as of RFC 6814.
	pub const UpstreamMulticastPacket: Self = InternetProtocolVersion4OptionKind(152);
	
	/// Quick-Start (QS) (Type = 25).
	///
	/// Threat as of RFC 7126.
	pub const QuickStart: Self = InternetProtocolVersion4OptionKind(25);
	
	/// RFC 3692 style Experiment (EXP) defined in RFC 4727.
	pub const Rfc3692StyleExperiment1: Self = InternetProtocolVersion4OptionKind(30);
	
	/// RFC 3692 style Experiment (EXP) defined in RFC 4727.
	pub const Rfc3692StyleExperiment2: Self = InternetProtocolVersion4OptionKind(94);
	
	/// RFC 3692 style Experiment (EXP) defined in RFC 4727.
	pub const Rfc3692StyleExperiment3: Self = InternetProtocolVersion4OptionKind(158);
	
	/// RFC 3692 style Experiment (EXP) defined in RFC 4727.
	pub const Rfc3692StyleExperiment4: Self = InternetProtocolVersion4OptionKind(222);
	
	/// Copied onto all fragments.
	///
	/// This result should be ignored if the `number()` is `EndOfOptionsList` or `NoOperation`.
	#[inline(always)]
	pub fn copied_onto_all_fragments(self) -> bool
	{
		self.0 & Self::OneBitCopyField != 0
	}
	
	/// Should not be copied onto fragments.
	///
	/// This result should be ignored if the `number()` is `EndOfOptionsList` or `NoOperation`.
	#[inline(always)]
	pub fn should_not_be_copied_onto_fragments(self) -> bool
	{
		self.0 & Self::OneBitCopyField == 0
	}
	
	/// Class.
	#[inline(always)]
	pub fn class(self) -> InternetProtocolVersion4OptionClass
	{
		unsafe { transmute(self.0 & Self::TwoBitClassField >> 5) }
	}
	
	/// Number.
	#[inline(always)]
	pub fn number(self) -> InternetProtocolVersion4OptionNumber
	{
		InternetProtocolVersion4OptionNumber(self.0 & Self::FiveBitNumberField)
	}
}
