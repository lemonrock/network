// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[macro_export]
macro_rules! drop
{
	($now: ident, $reason: expr, $ethernet_addresses: ident, $packet_processing: expr, $packet: ident) =>
	{
		{
			$packet_processing.drop($now, $reason, $ethernet_addresses, $packet);
			return
		}
	}
}
