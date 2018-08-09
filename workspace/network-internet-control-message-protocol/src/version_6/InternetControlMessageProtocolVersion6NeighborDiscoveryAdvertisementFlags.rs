// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


bitflags!
{
	/// RFC 4861 flags for Neighbour Discovery Advertisement messages.
	pub struct InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags: u8
	{
		/// Also known as 'R' and the 'R-bit'.
		///
		/// RFC 4861 Section 4.4: "When set, the R-bit indicates that the sender is a router.
		/// The R-bit is used by Neighbor Unreachability Detection to detect a router that changes to a host".
		const Router = 0b1000_0000;
		
		/// Also known as 'S' and the 'S-bit'.
		///
		/// RFC 4861 Section 4.4: "When set, the S-bit indicates that the advertisement was sent in response to a Neighbor Solicitation from the Destination address.
 		/// The S-bit is used as a reachability confirmation for Neighbor Unreachability Detection.
 		/// It MUST NOT be set in multicast advertisements or in  unsolicited unicast advertisements".
		const Solicited = 0b0100_0000;
		
		/// Also known as 'O' and the 'O-bit'.
		///
		/// RFC 4861 Section 4.4: "When set, the O-bit indicates that the advertisement should override an existing cache entry and update the cached link-layer address.
		/// When it is not set the advertisement will not update a cached link-layer address though it will update an existing Neighbor Cache entry for which no link-layer address is known.
		/// It SHOULD NOT be set in solicited advertisements for anycast addresses and in solicited proxy advertisements. It SHOULD be set in other solicited advertisements and in unsolicited advertisements".
		const Override = 0b0010_0000;
	}
}

impl Default for InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
