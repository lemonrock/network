// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A least recently used cache with expiry of items.
///
/// Expiry is only checked on access.
#[derive(Debug)]
pub struct LeastRecentlyUsedCacheWithExpiry<K: Eq + Hash, V>
{
	least_recently_used_cache: LeastRecentlyUsedCache<K, ExpiringValueWrapper<V>>,
	expiry_period: MillisecondDuration,
}

impl<K: Eq + Hash, V> LeastRecentlyUsedCacheWithExpiry<K, V>
{
	/// The `expiry_period` can be zero; entries are considered to be expired when this is exceeded.
	pub fn new(maximum_capacity: usize, expiry_period: MillisecondDuration) -> Self
	{
		Self
		{
			least_recently_used_cache: LeastRecentlyUsedCache::new(maximum_capacity),
			expiry_period
		}
	}
	
	/// Gets a value.
	#[inline(always)]
	pub fn get(&mut self, now: MonotonicMillisecondTimestamp, key: &K) -> Option<&V>
	{
		self.get_internal(now, key).map(|expiring_value_wrapper| &expiring_value_wrapper.value)
	}
	
	/// Gets a value that can be mutated.
	#[inline(always)]
	pub fn get_mut(&mut self, now: MonotonicMillisecondTimestamp, key: &K) -> Option<&mut V>
	{
		self.get_internal(now, key).map(|expiring_value_wrapper| &mut expiring_value_wrapper.value)
	}
	
	/// Insert an item.
	#[inline(always)]
	pub fn insert(&mut self, now: MonotonicMillisecondTimestamp, key: K, value: V)
	{
		self.least_recently_used_cache.insert(key, ExpiringValueWrapper
		{
			expires_at: now + self.expiry_period,
			value,
		})
	}
	
	#[inline(always)]
	fn get_internal(&mut self, now: MonotonicMillisecondTimestamp, key: &K) -> Option<&mut ExpiringValueWrapper<V>>
	{
		self.get_internal_borrow_check_defeating(now, key).map(|value| unsafe { &mut * value })
	}
	
	// TODO: Revisit this with a later Rust version, changing the result from `Option<*mut ExpiringValueWrapper<V>>` to `Option<&mut ExpiringValueWrapper<V>>`.
	// Currently, the rust borrow checker assumes too wide a scope for the first borrow.
	#[inline(always)]
	fn get_internal_borrow_check_defeating(&mut self, now: MonotonicMillisecondTimestamp, key: &K) -> Option<*mut ExpiringValueWrapper<V>>
	{
		{
			let x = &mut self.least_recently_used_cache;
			if let Some(expiring_value_wrapper) = x.get_mut(key)
			{
				if expiring_value_wrapper.has_expired(now)
				{
				}
				else
				{
					expiring_value_wrapper.recently_used(now, self.expiry_period);
					return Some(expiring_value_wrapper as *mut _)
				}
			}
			else
			{
				return None
			}
		}
		
		// The Rust borrow checker (as of rustc nightly 2018-07-12) thinks that this is a second mutable borrow whilst the first (`let x =`) is in scope. This is probably because it sees the result of the function as an &mut related to `x`. However, we know that no borrowed data is returned in this case and that the 'borrow' is effectively released.
		self.least_recently_used_cache.remove(key);
		None
	}
}
