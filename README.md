# network

[network] is a set of Rust crates to make it easier to work with networking.


## Features

### network-ethernet

* Use the feature `libc` to enable support for converting to and from:-
    * libc's `ether_addr` type for the struct `MediaAccessControlAddress`;
    * libc's `ether_header` type for the struct `EthernetPacketHeader`.
* Use the feature `dpdk-sys` to enable support for converting to and from:-
    * DPDK's `ether_addr` type for the struct `MediaAccessControlAddress`;
    * DPDK's `ether_hdr` type for the struct `EthernetPacketHeader`;
    * DPDK's `vlan_hdr` type for the struct `VirtualLanPacketHeader`.


## Licensing

The license for this project is AGPL3.

[network]: https://github.com/lemonrock/network "network GitHub page"
