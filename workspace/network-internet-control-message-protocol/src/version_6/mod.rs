// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


use super::*;
use self::codes::*;
use self::types::*;


/// Code 'enumerations'.
pub mod codes;


/// Type 'enumerations'.
pub mod types;


include!("InternetControlMessageProtocolVersion6NeighborDiscoveryAdvertisementFlags.rs");
include!("InternetControlMessageProtocolVersion6NeighborDiscoveryOption.rs");
include!("InternetControlMessageProtocolVersion6NeighborDiscoveryOptionHeader.rs");
include!("InternetControlMessageProtocolVersion6NeighborDiscoveryOptionType.rs");
include!("InternetControlMessageProtocolVersion6Packet.rs");
include!("InternetControlMessageProtocolVersion6PacketHeader.rs");
include!("InternetControlMessageProtocolVersion6PacketPayload.rs");
include!("InternetControlMessageProtocolVersion6TypeAndCode.rs");
include!("KnownInternetControlMessageProtocolVersion6NeighborDiscoveryOptionType.rs");

