// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[derive(Debug)]
struct ValueWrapper<K: Eq + Hash, V>
{
	list_node: NonNull<ListNode<K>>,
	value: V,
}

impl<K: Eq + Hash, V> ValueWrapper<K, V>
{
	#[inline(always)]
	fn list_node(&mut self) -> &mut ListNode<K>
	{
		unsafe { &mut * self.list_node.as_ptr() }
	}
	
	#[inline(always)]
	fn key(&self) -> NonNull<K>
	{
		unsafe { self.list_node.as_ref().key() }
	}
	
	#[inline(always)]
	fn recently_used(&mut self, linked_list: &mut LinkedList<K>)
	{
		self.list_node().move_to_tail(linked_list)
	}
}
