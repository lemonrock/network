// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


#[derive(Debug)]
struct ListNode<K: Eq + Hash>
{
	key: K,
	next: *mut ListNode<K>,
	previous: *mut ListNode<K>,
}

impl<K: Eq + Hash> ListNode<K>
{
	#[inline(always)]
	fn key(&self) -> NonNull<K>
	{
		unsafe { NonNull::new_unchecked((&self.key) as *const _ as *mut _) }
	}
	
	#[inline(always)]
	fn move_to_tail(&mut self, mut linked_list: &mut LinkedList<K>)
	{
		if likely!(linked_list.tail != self)
		{
			self.remove_from_linked_list(&mut linked_list);
			self.insert_at_tail(&mut linked_list);
		}
	}
	
	#[inline(always)]
	fn remove_from_linked_list(&mut self, linked_list: &mut LinkedList<K>)
	{
		let previous = self.previous;
		let next = self.next;
		
		if linked_list.head == self
		{
			linked_list.head = self.next
		}
		
		if linked_list.tail == self
		{
			linked_list.tail = self.previous
		}
		
		if !previous.is_null()
		{
			debug_assert_ne!(previous, self as *mut Self, "self.previous is self");
			
			let previous = unsafe { &mut *previous };
			previous.next = next;
		}
		
		if !next.is_null()
		{
			debug_assert_ne!(next, self as *mut Self, "self.next is self");
			
			(unsafe { &mut * next }).previous = previous;
		}
	}
	
	#[inline(always)]
	fn insert_at_tail(&mut self, linked_list: &mut LinkedList<K>)
	{
		let old_tail = linked_list.tail;
		if unlikely!(old_tail.is_null())
		{
			debug_assert!(linked_list.head.is_null(), "If linked_list.tail is null then linked_list.head must be null");
			
			self.previous = null_mut();
			self.next = null_mut();
			
			linked_list.head = self;
		}
		else
		{
			debug_assert!(!linked_list.head.is_null(), "If linked_list.tail is not null then linked_list.head must be not null");
			
			self.previous = old_tail;
			self.next = null_mut();
			
			(unsafe { &mut * old_tail }).next = self;
		}
		
		linked_list.tail = self;
	}
	
	#[inline(always)]
	fn insert_at_tail_newly_created(&mut self, linked_list: &mut LinkedList<K>, old_tail: *mut Self) -> NonNull<Self>
	{
		if unlikely!(old_tail.is_null())
		{
			debug_assert!(linked_list.head.is_null(), "If linked_list.tail is null then linked_list.head must be null");
			
			linked_list.head = self;
		}
		else
		{
			debug_assert!(!linked_list.head.is_null(), "If linked_list.tail is not null then linked_list.head must be not null");
			
			(unsafe { &mut * old_tail }).next = self;
		}
		
		linked_list.tail = self;
		
		unsafe { NonNull::new_unchecked(self) }
	}
}
