// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Mask bits.
pub trait InternetProtocolMaskBits: Sized + Debug + Display + PartialOrd + Ord + PartialEq + Eq + Hash + Serialize + Clone + Copy
{
	/// As depth, 0 to 32 for Internet Protocol Version 4 and 0 to 128 for Internet Protocol Version 6.
	#[inline(always)]
	fn as_depth(self) -> u8;
}
