// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Abstract network endian data.
pub trait NetworkEndian: Sized + Default + Debug + Copy + Clone + PartialOrd + Ord + PartialEq + Eq + Hash + Display + Serialize + DeserializeOwned
{
	/// Length in bytes.
	const Length: usize;
	
	/// Implements `[u8; Self::Length]` but this can not yet be expressed in Rust.
	type Bytes;
	
	/// Convert into bytes.
	#[inline(always)]
	fn to_bytes(self) -> Self::Bytes;
	
	/// Underlying bytes.
	#[inline(always)]
	fn bytes(&self) -> &[u8];
	
	/// Writes to a hasher creating a hash.
	#[inline(always)]
	fn write_to_hash<H: Hasher>(&self, hasher: &mut H);
}
