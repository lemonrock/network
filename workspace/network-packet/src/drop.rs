// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[macro_export]
macro_rules! drop
{
	($reason: expr, $dropped_packet_oberserver: ident, $packet: ident) =>
	{
		{
			let reason = $reason;
			$dropped_packet_oberserver.dropped_packet(reason);
			$packet.free_direct_contiguous_packet();
			return
		}
	}
}
