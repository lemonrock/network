// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A bounded hash map, very similar to a regular Rust HashMap but can not become larger than its maximum capacity (plus one); ideal for indexes of network connections, etc in resource constraint aware environments.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct BoundedHashMap<K: Hash + Eq, V>
{
	map: HashMap<K, V>,
	maximum_capacity: usize,
}

impl<K: Hash + Eq, V> BoundedHashMap<K, V>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(maximum_capacity: usize) -> Self
	{
		let allocate_hash_map_with_slightly_more_capacity_as_insert_always_reserves_1 = maximum_capacity + 1;
		
		Self
		{
			map: HashMap::with_capacity(allocate_hash_map_with_slightly_more_capacity_as_insert_always_reserves_1),
			maximum_capacity,
		}
	}
	
	/// Identical to HashMap.
	#[inline(always)]
	pub fn contains_key(&self, key: &K) -> bool
	{
		self.map.contains_key(key)
	}
	
	/// Identical to HashMap.
	#[inline(always)]
	pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
	{
		self.map.get_mut(key)
	}
	
	/// Identical to HashMap.
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.map.len()
	}
	
	/// Is this map full?
	#[inline(always)]
	pub fn is_full(&self) -> bool
	{
		self.len() == self.maximum_capacity
	}
	
	/// Is this map over-full? False if just full.
	#[inline(always)]
	pub fn is_over_filled(&self) -> bool
	{
		self.len() > self.maximum_capacity
	}
	
	/// Inserts a value once, returning a mutable reference to it.
	#[inline(always)]
	pub fn insert_uniquely_and_return_mutable_reference(&mut self, key: K, value: V) -> &mut V
	{
		debug_assert!(!self.map.contains_key(&key), "Already present");
		
		self.map.entry(key).or_insert(value)
	}
	
	/// Identical to HashMap.
	#[inline(always)]
	pub fn insert(&mut self, key: K, value: V) -> Option<V>
	{
		self.map.insert(key, value)
	}
	
	/// Identical to HashMap.
	#[inline(always)]
	pub fn remove(&mut self, key: &K) -> Option<V>
	{
		self.map.remove(key)
	}
	
	/// Identical to HashMap.
	#[inline(always)]
	pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)>
	{
		self.map.remove_entry(key)
	}
}
