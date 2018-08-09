// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Allows for notification of dropped incoming network packets.
pub trait IncomingNetworkPacketDropObserver
{
	/// The type of the drop reason.
	///
	/// Usually implemented as an enum.
	type DropReason: IncomingNetworkPacketProcessingDropReason;
	
	/// Notifies the observer that a packet has been dropped.
	///
	/// Things that can be done with this information:-
	///
	/// * Log to syslog.
	/// * Log to standard error.
	/// * Log to an in-memory ring buffer which can be 'inspected' (this has the advantage of fixing memory usage).
	/// * Increment a statistic, such as a counter.
	/// * Use injected Lua
	/// * Pass to a security monitor which can reactively and automatically adjust configuration (eg ban an IP address).
	/// * Ignore.
	///
	/// One additional possible idea, not yet supported, would to allow the observer to mutate the packet and 're-inject' it or 'reject' the drop. This is fraught with potential pitfalls.
	#[inline(always)]
	fn dropped_packet(&self, reason: Self::DropReason);
}
