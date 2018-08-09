// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Internet Protocol (IP) version 4 check sum status or Layer 4 (TCP, UDP, SCTP) check sum status.
#[deny(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(Deserialize, Serialize)]
pub enum HardwareOffloadCheckSumStatus
{
	/// No information available about the check sum.
	NoInformationKnown,
	
	/// The check sum in the packet is wrong.
	Bad,
	
	/// The check sum in the packet is valid.
	Good,
	
	/// The check sum is not correct in the packet data, but the integrity of the layer 4 or Internet Protocol (IP) version 4 pseudo-header was verified.
	///
	/// This is not known to occur for any DPDK poll mode drivers for Internet Protocol (IP) version 4.
	/// It does occur for Layer 4, but only for the VirtIO virtual DPDK poll mode driver.
	IncorrectButInternetProtocolHeaderIntegrityVerified,
}
