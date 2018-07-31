// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Extension trait for the overlap between Media Access Control (MAC) control addresses and internet protocol addresses.
pub trait MediaAccessControlAddressAndInternetProtocolAddressOverlap: Sized
{
	/// Currently, this is an identifier formerly used by Xerox.
	const PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_10_0_0_0: [u8; OrganizationallyUniqueIdentifier::Size] = [0x00, 0x00, 0x00];
	
	/// Currently, this is an identifier formerly used by Xerox.
	const PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_172_16_0_0: [u8; OrganizationallyUniqueIdentifier::Size] = [0x00, 0x00, 0x01];
	
	/// Currently, this is an identifier formerly used by Xerox.
	const PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_192_168_0_0: [u8; OrganizationallyUniqueIdentifier::Size] = [0x00, 0x00, 0x02];

	/// From an Internet Protocol (version 4) private host address.
	///
	/// This is a non-standard technique that uses universally administered addresses to encode an internet protocol host address as a Media Access Control Address.
	///
	/// Using this approach means one can avoid the need for insecure ARP requests and replies in controlled server environments.
	///
	/// Three different Universal Organizational Identifiers (OUIs) are used; currently these are official OUIs from IEEE for organizations whose usage of them is now defunct.
	///
	/// The idea for this technique comes from a similar one implemented by DECnet.
	///
	/// Specifically:-
	///
	/// * The last 3 bytes of the internet protocol address are encoded in the last 3 bytes of the media access control address.
	/// * The first byte of the internet protocol address is encoded as a Media Access Control address.
	/// * Checks are not made for invalid addresses (eg ending in .0 or .255).
	#[inline(always)]
	fn from_private_internet_protocol_version_4_host_address(internet_protocol_version_4_host_address: &InternetProtocolVersion4HostAddress) -> Result<Self, ()>;
	
	/// Tries to convert to an internet protocol (IP) version 4 host address.
	#[inline(always)]
	fn to_private_internet_protocol_version_4_host_address(&self) -> Result<InternetProtocolVersion4HostAddress, ()>;
	
	/// From an Internet Protocol (version 6) host address.
	///
	/// Returns an error if octets 11 and 12 (zero based indices) are not 0xFF and 0xFE respectively.
	#[inline(always)]
	fn from_internet_protocol_version_6_host_address(internet_protocol_version_6_host_address: InternetProtocolVersion6HostAddress) -> Result<Self, ()>;
	
	/// Convert to a link-local internet protocol (IP) version 6 host address.
	#[inline(always)]
	fn to_link_local_internet_protocol_version_6_host_address(&self) -> InternetProtocolVersion6HostAddress
	{
		self.to_internet_protocol_version_6_host_address(&[0xFE, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
	}
	
	/// Convert to an internet protocol (IP) version 6 host address.
	///
	/// See RFC 4291 Appendix A (Creating Modified EUI-64 Format Interface Identifiers) updated by RFC 7136 section 5, specifically in Appendix A, "Links or Nodes with IEEE 802 48-bit MACs".
	#[inline(always)]
	fn to_internet_protocol_version_6_host_address(&self, internet_protocol_version_6_host_address_prefix: &[u8; 8]) -> InternetProtocolVersion6HostAddress;
}

impl MediaAccessControlAddressAndInternetProtocolAddressOverlap for MediaAccessControlAddress
{
	#[inline(always)]
	fn from_private_internet_protocol_version_4_host_address(internet_protocol_version_4_host_address: &InternetProtocolVersion4HostAddress) -> Result<Self, ()>
	{
		let internet_protocol_version_4_host_address = *internet_protocol_version_4_host_address;
		
		let organizationally_unique_identifier = if InternetProtocolVersion4NetworkAddress::Private1.contains(internet_protocol_version_4_host_address)
		{
			Self::PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_10_0_0_0
		}
		else if InternetProtocolVersion4NetworkAddress::Private2.contains(internet_protocol_version_4_host_address)
		{
			Self::PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_172_16_0_0
		}
		else if InternetProtocolVersion4NetworkAddress::Private3.contains(internet_protocol_version_4_host_address)
		{
			Self::PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_192_168_0_0
		}
		else
		{
			return Err(())
		};
		
		let mut this: MediaAccessControlAddress = unsafe { uninitialized() };
		
		{
			let bytes = this.to_octets_mutable_reference();
			
			unsafe { copy_nonoverlapping((&organizationally_unique_identifier[..]).as_ptr(), bytes.as_mut_ptr(), Self::OrganizationallyUniqueIdentifierSize) };
			
			unsafe { copy_nonoverlapping((&internet_protocol_version_4_host_address.0[1..InternetProtocolVersion4HostAddress::Size]).as_ptr(), bytes.as_mut_ptr(), 3) };
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	fn to_private_internet_protocol_version_4_host_address(&self) -> Result<InternetProtocolVersion4HostAddress, ()>
	{
		let first_octet = match array_ref!(self.to_octets_reference(), 0, OrganizationallyUniqueIdentifier::Size)
		{
			&Self::PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_10_0_0_0 => 10,
			&Self::PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_172_16_0_0 => 172,
			&Self::PrivateInternetProtocolVersion4AddressOrganizationallyUniqueIdentifier_192_168_0_0 => 192,
			_ => return Err(()),
		};
		
		let mut internet_protocol_version_4_host_address: InternetProtocolVersion4HostAddress = unsafe { uninitialized() };
		{
			let octets = &mut internet_protocol_version_4_host_address.0;
			*unsafe { octets.get_unchecked_mut(0) } = first_octet;
			unsafe { copy_nonoverlapping((&self.to_octets_reference()[Self::OrganizationallyUniqueIdentifierSize..6]).as_ptr(), (&mut octets[1..]).as_mut_ptr(), Self::OrganizationallyUniqueIdentifierSize) };
		}
		Ok(internet_protocol_version_4_host_address)
	}
	
	#[inline(always)]
	fn from_internet_protocol_version_6_host_address(internet_protocol_version_6_host_address: InternetProtocolVersion6HostAddress) -> Result<Self, ()>
	{
		let mut this: MediaAccessControlAddress = unsafe { uninitialized() };
		
		{
			let bytes = this.to_octets_mutable_reference();
			let octets = &internet_protocol_version_6_host_address.0;
			
			if octets[11] != 0xFF || octets[12] != 0xFE
			{
				return Err(())
			}
			
			bytes[0] = octets[8] ^ Self::LocallyAdministeredAddressBitFlag;
			bytes[1] = octets[9];
			bytes[2] = octets[10];
			bytes[3] = octets[13];
			bytes[4] = octets[14];
			bytes[5] = octets[15];
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	fn to_internet_protocol_version_6_host_address(&self, internet_protocol_version_6_host_address_prefix: &[u8; 8]) -> InternetProtocolVersion6HostAddress
	{
		let mut internet_protocol_version_6_host_address = InternetProtocolVersion6HostAddress::from_octets(unsafe { uninitialized() });
		
		{
			let octets = &mut internet_protocol_version_6_host_address.0;
			
			unsafe { *(octets.as_mut_ptr() as *mut u64) = *(internet_protocol_version_6_host_address_prefix.as_ptr() as *mut u64) };
			let bytes = self.to_octets_reference();
			octets[8] = bytes[0] ^ Self::LocallyAdministeredAddressBitFlag;
			octets[9] = bytes[1];
			octets[10] = bytes[2];
			octets[11] = 0xFF;
			octets[12] = 0xFE;
			octets[13] = bytes[3];
			octets[14] = bytes[4];
			octets[15] = bytes[5];
		}
		
		internet_protocol_version_6_host_address
	}
}
