// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Configuration used to build a static routing table.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct StaticRoutingTableConfiguration<NetworkAddress: InternetProtocolNetworkAddress>
where <NetworkAddress as InternetProtocolNetworkAddress>::HostAddress: DeserializeOwned
{
	/// These are the static routes.
	pub static_routes: HashMap<NetworkAddress, Route<NetworkAddress::HostAddress>>,
	
	/// This is used if no other routes match, or if essential information can't be found for a route.
	pub default_route_to_next_hop: EthernetDestination,
}

impl<NetworkAddress: InternetProtocolNetworkAddress> StaticRoutingTableConfiguration<NetworkAddress>
where <NetworkAddress as InternetProtocolNetworkAddress>::HostAddress: DeserializeOwned
{
	/// Configure.
	#[inline(always)]
	pub fn configure(mut self) -> StaticRoutingTable<NetworkAddress>
	{
		StaticRoutingTable
		{
			longest_prefix_match:
			{
				let mut tree_bitmap: TreeBitmap<Route<NetworkAddress::HostAddress>> = TreeBitmap::with_capacity(self.static_routes.len());
				
				for (network_address, static_route) in self.static_routes.drain()
				{
					let mask_length = network_address.mask_bits_as_depth_u32();
					let nibbles = network_address.network().nibbles();
					tree_bitmap.insert(nibbles.as_ref(), mask_length, static_route);
				}
				
				tree_bitmap
			},
			
			default_route_to_next_hop: self.default_route_to_next_hop,
		}
	}
}
