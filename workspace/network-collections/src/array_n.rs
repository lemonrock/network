// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


macro_rules! array_n
{
	($type: tt, $size: expr) =>
	{
		/// An array new type wrapper with $type elements.
		///
		/// Exists because Rust does not provide arrays with more than 32 elements which implement common traits.
		pub struct $type<T>(pub [T; $size]);
		
		default impl<'deserialize, T: Deserialize<'deserialize>> Deserialize<'deserialize> for $type<T>
		{
			#[inline(always)]
			fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
			{
				const Size: usize = $size;
				
				struct DeserializeVisitor<T>(PhantomData<T>);
				
				impl<'deserialize, T: Deserialize<'deserialize>> Visitor<'deserialize> for DeserializeVisitor<T>
				{
					type Value = $type<T>;
					
					#[inline(always)]
					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
					{
						write!(formatter, "a {} element array", Size)
					}
					
					#[inline(always)]
					fn visit_seq<A: SeqAccess<'deserialize>>(self, mut access: A) -> Result<Self::Value, A::Error>
					{
						let mut result = $type(unsafe { uninitialized() });
						
						let mut index = 0;
						while let Some(byte) = access.next_element()?
						{
							if index == Size
							{
								return Err(A::Error::invalid_length(index, &self))
							}
							* (unsafe { result.0.get_unchecked_mut(index) }) = byte;
							index += 1;
						}
						if index != Size
						{
							Err(A::Error::invalid_length(index, &self))
						}
						else
						{
							Ok(result)
						}
					}
				}
				
				deserializer.deserialize_tuple(Size, DeserializeVisitor(PhantomData))
			}
		}
		
		impl<'deserialize> Deserialize<'deserialize> for $type<u8>
		{
			#[inline(always)]
			fn deserialize<D: Deserializer<'deserialize>>(deserializer: D) -> Result<Self, D::Error>
			{
				const Size: usize = $size;
				
				struct DeserializeVisitor;
				
				impl<'deserialize> Visitor<'deserialize> for DeserializeVisitor
				{
					type Value = $type<u8>;
					
					#[inline(always)]
					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
					{
						write!(formatter, "a {} element byte array", Size)
					}
					
					#[inline(always)]
					fn visit_bytes<E: DeserializeError>(self, v: &[u8]) -> Result<Self::Value, E>
					{
						if v.len() != Size
						{
							return Err(E::invalid_length(v.len(), &self))
						}
		
						let mut result = $type(unsafe { uninitialized() });
						result.0.as_mut().clone_from_slice(v);
						Ok(result)
					}
					
					#[inline(always)]
					fn visit_seq<A: SeqAccess<'deserialize>>(self, mut access: A) -> Result<Self::Value, A::Error>
					{
						let mut result = $type(unsafe { uninitialized() });
						
						let mut index = 0;
						while let Some(byte) = access.next_element()?
						{
							if index == Size
							{
								return Err(A::Error::invalid_length(index, &self))
							}
							* (unsafe { result.0.get_unchecked_mut(index) }) = byte;
							index += 1;
						}
						if index != Size
						{
							Err(A::Error::invalid_length(index, &self))
						}
						else
						{
							Ok(result)
						}
					}
				}
				
				deserializer.deserialize_tuple(Size, DeserializeVisitor)
			}
		}

		impl<T: Serialize> Serialize for $type<T>
		{
			#[inline(always)]
			fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
			{
				const Size: usize = $size;
				
				let mut tuple = serializer.serialize_tuple(Size)?;
				for index in 0 .. Size
				{
					tuple.serialize_element(unsafe { self.0.get_unchecked(index) })?;
				}
				tuple.end()
			}
		}
		
		impl<T: Copy> Copy for $type<T>
		{
		}
		
		impl<T: Copy> Clone for $type<T>
		{
			#[inline(always)]
			fn clone(&self) -> Self
			{
				$type(self.0)
			}
		}
		
		impl<T: Debug> Debug for $type<T>
		{
			#[inline(always)]
			fn fmt(&self, formatter: &mut Formatter) -> fmt::Result
			{
				self.0[..].fmt(formatter)
			}
		}
		
		impl<T: PartialEq> PartialEq for $type<T>
		{
			#[inline(always)]
			fn eq(&self, other: &Self) -> bool
			{
				self.0[..].eq(&other.0[..])
			}
		}
		
		impl<T: Eq> Eq for $type<T>
		{
		}
		
		impl<T: PartialOrd> PartialOrd for $type<T>
		{
			#[inline(always)]
			fn partial_cmp(&self, other: &Self) -> Option<Ordering>
			{
				PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
			}
			
			#[inline(always)]
			fn lt(&self, other: &Self) -> bool
			{
				PartialOrd::lt(&&self.0[..], &&other.0[..])
			}
			
			#[inline(always)]
			fn le(&self, other: &Self) -> bool
			{
				PartialOrd::le(&&self.0[..], &&other.0[..])
			}
			
			#[inline(always)]
			fn ge(&self, other: &Self) -> bool
			{
				PartialOrd::ge(&&self.0[..], &&other.0[..])
			}
			
			#[inline(always)]
			fn gt(&self, other: &Self) -> bool
			{
				PartialOrd::gt(&&self.0[..], &&other.0[..])
			}
		}
		
		impl<T: Ord> Ord for $type<T>
		{
			#[inline(always)]
			fn cmp(&self, other: &Self) -> Ordering
			{
				Ord::cmp(&&self.0[..], &&other.0[..])
			}
		}
		
		impl<T: Hash> Hash for $type<T>
		{
			#[inline(always)]
			fn hash<H: Hasher>(&self, state: &mut H)
			{
				Hash::hash(&self.0[..], state)
			}
		}
		
		impl<'a, T> IntoIterator for &'a $type<T>
		{
			type Item = &'a T;
			type IntoIter = Iter<'a, T>;
			
			#[inline(always)]
			fn into_iter(self) -> Iter<'a, T>
			{
				self.0.iter()
			}
		}
		
		impl<'a, T> IntoIterator for &'a mut $type<T>
		{
			type Item = &'a mut T;
			type IntoIter = IterMut<'a, T>;
			
			#[inline(always)]
			fn into_iter(self) -> IterMut<'a, T>
			{
				self.0.iter_mut()
			}
		}
		
		impl<T> AsRef<[T]> for $type<T>
		{
			#[inline(always)]
			fn as_ref(&self) -> &[T]
			{
				&self.0[..]
			}
		}
		
		impl<T> AsMut<[T]> for $type<T>
		{
			#[inline(always)]
			fn as_mut(&mut self) -> &mut [T]
			{
				&mut self.0[..]
			}
		}
		
		impl<T> Deref for $type<T>
		{
			type Target = [T; $size];
		
			#[inline(always)]
			fn deref(&self) -> &Self::Target
			{
				&self.0
			}
		}
		
		impl<T> DerefMut for $type<T>
		{
			#[inline(always)]
			fn deref_mut(&mut self) -> &mut Self::Target
			{
				&mut self.0
			}
		}
		
		impl<T: Default + Copy> Default for $type<T>
		{
			#[inline(always)]
			fn default() -> Self
			{
				$type([T::default(); $size])
			}
		}
		
		impl<T> From<[T; $size]> for $type<T>
		{
			#[inline(always)]
			fn from(value: [T; $size]) -> Self
			{
				$type(value)
			}
		}
		
		impl<T> Into<[T; $size]> for $type<T>
		{
			#[inline(always)]
			fn into(self) -> [T; $size]
			{
				self.0
			}
		}
		
		impl<T> Borrow<[T]> for $type<T>
		{
			#[inline(always)]
			fn borrow(&self) -> &[T]
			{
				&self.0[..]
			}
		}

		impl<T> BorrowMut<[T]> for $type<T>
		{
			#[inline(always)]
			fn borrow_mut(&mut self) -> &mut [T]
			{
				&mut self.0
			}
		}
	}
}
