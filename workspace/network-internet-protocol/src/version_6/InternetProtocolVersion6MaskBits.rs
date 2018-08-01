// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/*
Generated using the logic:-
fn main()
{
    for prefix_length in 0 .. 129
    {
        let mut prefix: u128 = 0;
    
        for bit in ( (128 - prefix_length) .. 128)
        {
            prefix |= 1 << bit;
        }
        
        println!("\t/// `/{}`", prefix_length);
        println!("\t#[cfg(target_endian = \"big\")] _{} = 0x{:032X},", prefix_length, prefix);
        println!("\t#[cfg(target_endian = \"little\")] _{} = 0x{:032X},", prefix_length, prefix.to_be());
        println!("\t");
    }
}
*/

/// Mask bits for `/0` to `/128`.
///
/// Stored internally in network byte (big endian) byte order.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u128)]
#[derive(Serialize, Deserialize)]
pub enum InternetProtocolVersion6MaskBits
{
	/// `/0`
	#[cfg(target_endian = "big")] _0 = 0x00000000000000000000000000000000,
	/// `/0`
	#[cfg(target_endian = "little")] _0 = 0x00000000000000000000000000000000,
	
	/// `/1`
	#[cfg(target_endian = "big")] _1 = 0x80000000000000000000000000000000,
	/// `/1`
	#[cfg(target_endian = "little")] _1 = 0x00000000000000000000000000000080,
	
	/// `/2`
	#[cfg(target_endian = "big")] _2 = 0xC0000000000000000000000000000000,
	/// `/2`
	#[cfg(target_endian = "little")] _2 = 0x000000000000000000000000000000C0,
	
	/// `/3`
	#[cfg(target_endian = "big")] _3 = 0xE0000000000000000000000000000000,
	/// `/3`
	#[cfg(target_endian = "little")] _3 = 0x000000000000000000000000000000E0,
	
	/// `/4`
	#[cfg(target_endian = "big")] _4 = 0xF0000000000000000000000000000000,
	/// `/4`
	#[cfg(target_endian = "little")] _4 = 0x000000000000000000000000000000F0,
	
	/// `/5`
	#[cfg(target_endian = "big")] _5 = 0xF8000000000000000000000000000000,
	/// `/5`
	#[cfg(target_endian = "little")] _5 = 0x000000000000000000000000000000F8,
	
	/// `/6`
	#[cfg(target_endian = "big")] _6 = 0xFC000000000000000000000000000000,
	/// `/6`
	#[cfg(target_endian = "little")] _6 = 0x000000000000000000000000000000FC,
	
	/// `/7`
	#[cfg(target_endian = "big")] _7 = 0xFE000000000000000000000000000000,
	/// `/7`
	#[cfg(target_endian = "little")] _7 = 0x000000000000000000000000000000FE,
	
	/// `/8`
	#[cfg(target_endian = "big")] _8 = 0xFF000000000000000000000000000000,
	/// `/8`
	#[cfg(target_endian = "little")] _8 = 0x000000000000000000000000000000FF,
	
	/// `/9`
	#[cfg(target_endian = "big")] _9 = 0xFF800000000000000000000000000000,
	/// `/9`
	#[cfg(target_endian = "little")] _9 = 0x000000000000000000000000000080FF,
	
	/// `/10`
	#[cfg(target_endian = "big")] _10 = 0xFFC00000000000000000000000000000,
	/// `/10`
	#[cfg(target_endian = "little")] _10 = 0x0000000000000000000000000000C0FF,
	
	/// `/11`
	#[cfg(target_endian = "big")] _11 = 0xFFE00000000000000000000000000000,
	/// `/11`
	#[cfg(target_endian = "little")] _11 = 0x0000000000000000000000000000E0FF,
	
	/// `/12`
	#[cfg(target_endian = "big")] _12 = 0xFFF00000000000000000000000000000,
	/// `/12`
	#[cfg(target_endian = "little")] _12 = 0x0000000000000000000000000000F0FF,
	
	/// `/13`
	#[cfg(target_endian = "big")] _13 = 0xFFF80000000000000000000000000000,
	/// `/13`
	#[cfg(target_endian = "little")] _13 = 0x0000000000000000000000000000F8FF,
	
	/// `/14`
	#[cfg(target_endian = "big")] _14 = 0xFFFC0000000000000000000000000000,
	/// `/14`
	#[cfg(target_endian = "little")] _14 = 0x0000000000000000000000000000FCFF,
	
	/// `/15`
	#[cfg(target_endian = "big")] _15 = 0xFFFE0000000000000000000000000000,
	/// `/15`
	#[cfg(target_endian = "little")] _15 = 0x0000000000000000000000000000FEFF,
	
	/// `/16`
	#[cfg(target_endian = "big")] _16 = 0xFFFF0000000000000000000000000000,
	/// `/16`
	#[cfg(target_endian = "little")] _16 = 0x0000000000000000000000000000FFFF,
	
	/// `/17`
	#[cfg(target_endian = "big")] _17 = 0xFFFF8000000000000000000000000000,
	/// `/17`
	#[cfg(target_endian = "little")] _17 = 0x0000000000000000000000000080FFFF,
	
	/// `/18`
	#[cfg(target_endian = "big")] _18 = 0xFFFFC000000000000000000000000000,
	/// `/18`
	#[cfg(target_endian = "little")] _18 = 0x00000000000000000000000000C0FFFF,
	
	/// `/19`
	#[cfg(target_endian = "big")] _19 = 0xFFFFE000000000000000000000000000,
	/// `/19`
	#[cfg(target_endian = "little")] _19 = 0x00000000000000000000000000E0FFFF,
	
	/// `/20`
	#[cfg(target_endian = "big")] _20 = 0xFFFFF000000000000000000000000000,
	/// `/20`
	#[cfg(target_endian = "little")] _20 = 0x00000000000000000000000000F0FFFF,
	
	/// `/21`
	#[cfg(target_endian = "big")] _21 = 0xFFFFF800000000000000000000000000,
	/// `/21`
	#[cfg(target_endian = "little")] _21 = 0x00000000000000000000000000F8FFFF,
	
	/// `/22`
	#[cfg(target_endian = "big")] _22 = 0xFFFFFC00000000000000000000000000,
	/// `/22`
	#[cfg(target_endian = "little")] _22 = 0x00000000000000000000000000FCFFFF,
	
	/// `/23`
	#[cfg(target_endian = "big")] _23 = 0xFFFFFE00000000000000000000000000,
	/// `/23`
	#[cfg(target_endian = "little")] _23 = 0x00000000000000000000000000FEFFFF,
	
	/// `/24`
	#[cfg(target_endian = "big")] _24 = 0xFFFFFF00000000000000000000000000,
	/// `/24`
	#[cfg(target_endian = "little")] _24 = 0x00000000000000000000000000FFFFFF,
	
	/// `/25`
	#[cfg(target_endian = "big")] _25 = 0xFFFFFF80000000000000000000000000,
	/// `/25`
	#[cfg(target_endian = "little")] _25 = 0x00000000000000000000000080FFFFFF,
	
	/// `/26`
	#[cfg(target_endian = "big")] _26 = 0xFFFFFFC0000000000000000000000000,
	/// `/26`
	#[cfg(target_endian = "little")] _26 = 0x000000000000000000000000C0FFFFFF,
	
	/// `/27`
	#[cfg(target_endian = "big")] _27 = 0xFFFFFFE0000000000000000000000000,
	/// `/27`
	#[cfg(target_endian = "little")] _27 = 0x000000000000000000000000E0FFFFFF,
	
	/// `/28`
	#[cfg(target_endian = "big")] _28 = 0xFFFFFFF0000000000000000000000000,
	/// `/28`
	#[cfg(target_endian = "little")] _28 = 0x000000000000000000000000F0FFFFFF,
	
	/// `/29`
	#[cfg(target_endian = "big")] _29 = 0xFFFFFFF8000000000000000000000000,
	/// `/29`
	#[cfg(target_endian = "little")] _29 = 0x000000000000000000000000F8FFFFFF,
	
	/// `/30`
	#[cfg(target_endian = "big")] _30 = 0xFFFFFFFC000000000000000000000000,
	/// `/30`
	#[cfg(target_endian = "little")] _30 = 0x000000000000000000000000FCFFFFFF,
	
	/// `/31`
	#[cfg(target_endian = "big")] _31 = 0xFFFFFFFE000000000000000000000000,
	/// `/31`
	#[cfg(target_endian = "little")] _31 = 0x000000000000000000000000FEFFFFFF,
	
	/// `/32`
	#[cfg(target_endian = "big")] _32 = 0xFFFFFFFF000000000000000000000000,
	/// `/32`
	#[cfg(target_endian = "little")] _32 = 0x000000000000000000000000FFFFFFFF,
	
	/// `/33`
	#[cfg(target_endian = "big")] _33 = 0xFFFFFFFF800000000000000000000000,
	/// `/33`
	#[cfg(target_endian = "little")] _33 = 0x000000000000000000000080FFFFFFFF,
	
	/// `/34`
	#[cfg(target_endian = "big")] _34 = 0xFFFFFFFFC00000000000000000000000,
	/// `/34`
	#[cfg(target_endian = "little")] _34 = 0x0000000000000000000000C0FFFFFFFF,
	
	/// `/35`
	#[cfg(target_endian = "big")] _35 = 0xFFFFFFFFE00000000000000000000000,
	/// `/35`
	#[cfg(target_endian = "little")] _35 = 0x0000000000000000000000E0FFFFFFFF,
	
	/// `/36`
	#[cfg(target_endian = "big")] _36 = 0xFFFFFFFFF00000000000000000000000,
	/// `/36`
	#[cfg(target_endian = "little")] _36 = 0x0000000000000000000000F0FFFFFFFF,
	
	/// `/37`
	#[cfg(target_endian = "big")] _37 = 0xFFFFFFFFF80000000000000000000000,
	/// `/37`
	#[cfg(target_endian = "little")] _37 = 0x0000000000000000000000F8FFFFFFFF,
	
	/// `/38`
	#[cfg(target_endian = "big")] _38 = 0xFFFFFFFFFC0000000000000000000000,
	/// `/38`
	#[cfg(target_endian = "little")] _38 = 0x0000000000000000000000FCFFFFFFFF,
	
	/// `/39`
	#[cfg(target_endian = "big")] _39 = 0xFFFFFFFFFE0000000000000000000000,
	/// `/39`
	#[cfg(target_endian = "little")] _39 = 0x0000000000000000000000FEFFFFFFFF,
	
	/// `/40`
	#[cfg(target_endian = "big")] _40 = 0xFFFFFFFFFF0000000000000000000000,
	/// `/40`
	#[cfg(target_endian = "little")] _40 = 0x0000000000000000000000FFFFFFFFFF,
	
	/// `/41`
	#[cfg(target_endian = "big")] _41 = 0xFFFFFFFFFF8000000000000000000000,
	/// `/41`
	#[cfg(target_endian = "little")] _41 = 0x0000000000000000000080FFFFFFFFFF,
	
	/// `/42`
	#[cfg(target_endian = "big")] _42 = 0xFFFFFFFFFFC000000000000000000000,
	/// `/42`
	#[cfg(target_endian = "little")] _42 = 0x00000000000000000000C0FFFFFFFFFF,
	
	/// `/43`
	#[cfg(target_endian = "big")] _43 = 0xFFFFFFFFFFE000000000000000000000,
	/// `/43`
	#[cfg(target_endian = "little")] _43 = 0x00000000000000000000E0FFFFFFFFFF,
	
	/// `/44`
	#[cfg(target_endian = "big")] _44 = 0xFFFFFFFFFFF000000000000000000000,
	/// `/44`
	#[cfg(target_endian = "little")] _44 = 0x00000000000000000000F0FFFFFFFFFF,
	
	/// `/45`
	#[cfg(target_endian = "big")] _45 = 0xFFFFFFFFFFF800000000000000000000,
	/// `/45`
	#[cfg(target_endian = "little")] _45 = 0x00000000000000000000F8FFFFFFFFFF,
	
	/// `/46`
	#[cfg(target_endian = "big")] _46 = 0xFFFFFFFFFFFC00000000000000000000,
	/// `/46`
	#[cfg(target_endian = "little")] _46 = 0x00000000000000000000FCFFFFFFFFFF,
	
	/// `/47`
	#[cfg(target_endian = "big")] _47 = 0xFFFFFFFFFFFE00000000000000000000,
	/// `/47`
	#[cfg(target_endian = "little")] _47 = 0x00000000000000000000FEFFFFFFFFFF,
	
	/// `/48`
	#[cfg(target_endian = "big")] _48 = 0xFFFFFFFFFFFF00000000000000000000,
	/// `/48`
	#[cfg(target_endian = "little")] _48 = 0x00000000000000000000FFFFFFFFFFFF,
	
	/// `/49`
	#[cfg(target_endian = "big")] _49 = 0xFFFFFFFFFFFF80000000000000000000,
	/// `/49`
	#[cfg(target_endian = "little")] _49 = 0x00000000000000000080FFFFFFFFFFFF,
	
	/// `/50`
	#[cfg(target_endian = "big")] _50 = 0xFFFFFFFFFFFFC0000000000000000000,
	/// `/50`
	#[cfg(target_endian = "little")] _50 = 0x000000000000000000C0FFFFFFFFFFFF,
	
	/// `/51`
	#[cfg(target_endian = "big")] _51 = 0xFFFFFFFFFFFFE0000000000000000000,
	/// `/51`
	#[cfg(target_endian = "little")] _51 = 0x000000000000000000E0FFFFFFFFFFFF,
	
	/// `/52`
	#[cfg(target_endian = "big")] _52 = 0xFFFFFFFFFFFFF0000000000000000000,
	/// `/52`
	#[cfg(target_endian = "little")] _52 = 0x000000000000000000F0FFFFFFFFFFFF,
	
	/// `/53`
	#[cfg(target_endian = "big")] _53 = 0xFFFFFFFFFFFFF8000000000000000000,
	/// `/53`
	#[cfg(target_endian = "little")] _53 = 0x000000000000000000F8FFFFFFFFFFFF,
	
	/// `/54`
	#[cfg(target_endian = "big")] _54 = 0xFFFFFFFFFFFFFC000000000000000000,
	/// `/54`
	#[cfg(target_endian = "little")] _54 = 0x000000000000000000FCFFFFFFFFFFFF,
	
	/// `/55`
	#[cfg(target_endian = "big")] _55 = 0xFFFFFFFFFFFFFE000000000000000000,
	/// `/55`
	#[cfg(target_endian = "little")] _55 = 0x000000000000000000FEFFFFFFFFFFFF,
	
	/// `/56`
	#[cfg(target_endian = "big")] _56 = 0xFFFFFFFFFFFFFF000000000000000000,
	/// `/56`
	#[cfg(target_endian = "little")] _56 = 0x000000000000000000FFFFFFFFFFFFFF,
	
	/// `/57`
	#[cfg(target_endian = "big")] _57 = 0xFFFFFFFFFFFFFF800000000000000000,
	/// `/57`
	#[cfg(target_endian = "little")] _57 = 0x000000000000000080FFFFFFFFFFFFFF,
	
	/// `/58`
	#[cfg(target_endian = "big")] _58 = 0xFFFFFFFFFFFFFFC00000000000000000,
	/// `/58`
	#[cfg(target_endian = "little")] _58 = 0x0000000000000000C0FFFFFFFFFFFFFF,
	
	/// `/59`
	#[cfg(target_endian = "big")] _59 = 0xFFFFFFFFFFFFFFE00000000000000000,
	/// `/59`
	#[cfg(target_endian = "little")] _59 = 0x0000000000000000E0FFFFFFFFFFFFFF,
	
	/// `/60`
	#[cfg(target_endian = "big")] _60 = 0xFFFFFFFFFFFFFFF00000000000000000,
	/// `/60`
	#[cfg(target_endian = "little")] _60 = 0x0000000000000000F0FFFFFFFFFFFFFF,
	
	/// `/61`
	#[cfg(target_endian = "big")] _61 = 0xFFFFFFFFFFFFFFF80000000000000000,
	/// `/61`
	#[cfg(target_endian = "little")] _61 = 0x0000000000000000F8FFFFFFFFFFFFFF,
	
	/// `/62`
	#[cfg(target_endian = "big")] _62 = 0xFFFFFFFFFFFFFFFC0000000000000000,
	/// `/62`
	#[cfg(target_endian = "little")] _62 = 0x0000000000000000FCFFFFFFFFFFFFFF,
	
	/// `/63`
	#[cfg(target_endian = "big")] _63 = 0xFFFFFFFFFFFFFFFE0000000000000000,
	/// `/63`
	#[cfg(target_endian = "little")] _63 = 0x0000000000000000FEFFFFFFFFFFFFFF,
	
	/// `/64`
	#[cfg(target_endian = "big")] _64 = 0xFFFFFFFFFFFFFFFF0000000000000000,
	/// `/64`
	#[cfg(target_endian = "little")] _64 = 0x0000000000000000FFFFFFFFFFFFFFFF,
	
	/// `/65`
	#[cfg(target_endian = "big")] _65 = 0xFFFFFFFFFFFFFFFF8000000000000000,
	/// `/65`
	#[cfg(target_endian = "little")] _65 = 0x0000000000000080FFFFFFFFFFFFFFFF,
	
	/// `/66`
	#[cfg(target_endian = "big")] _66 = 0xFFFFFFFFFFFFFFFFC000000000000000,
	/// `/66`
	#[cfg(target_endian = "little")] _66 = 0x00000000000000C0FFFFFFFFFFFFFFFF,
	
	/// `/67`
	#[cfg(target_endian = "big")] _67 = 0xFFFFFFFFFFFFFFFFE000000000000000,
	/// `/67`
	#[cfg(target_endian = "little")] _67 = 0x00000000000000E0FFFFFFFFFFFFFFFF,
	
	/// `/68`
	#[cfg(target_endian = "big")] _68 = 0xFFFFFFFFFFFFFFFFF000000000000000,
	/// `/68`
	#[cfg(target_endian = "little")] _68 = 0x00000000000000F0FFFFFFFFFFFFFFFF,
	
	/// `/69`
	#[cfg(target_endian = "big")] _69 = 0xFFFFFFFFFFFFFFFFF800000000000000,
	/// `/69`
	#[cfg(target_endian = "little")] _69 = 0x00000000000000F8FFFFFFFFFFFFFFFF,
	
	/// `/70`
	#[cfg(target_endian = "big")] _70 = 0xFFFFFFFFFFFFFFFFFC00000000000000,
	/// `/70`
	#[cfg(target_endian = "little")] _70 = 0x00000000000000FCFFFFFFFFFFFFFFFF,
	
	/// `/71`
	#[cfg(target_endian = "big")] _71 = 0xFFFFFFFFFFFFFFFFFE00000000000000,
	/// `/71`
	#[cfg(target_endian = "little")] _71 = 0x00000000000000FEFFFFFFFFFFFFFFFF,
	
	/// `/72`
	#[cfg(target_endian = "big")] _72 = 0xFFFFFFFFFFFFFFFFFF00000000000000,
	/// `/72`
	#[cfg(target_endian = "little")] _72 = 0x00000000000000FFFFFFFFFFFFFFFFFF,
	
	/// `/73`
	#[cfg(target_endian = "big")] _73 = 0xFFFFFFFFFFFFFFFFFF80000000000000,
	/// `/73`
	#[cfg(target_endian = "little")] _73 = 0x00000000000080FFFFFFFFFFFFFFFFFF,
	
	/// `/74`
	#[cfg(target_endian = "big")] _74 = 0xFFFFFFFFFFFFFFFFFFC0000000000000,
	/// `/74`
	#[cfg(target_endian = "little")] _74 = 0x000000000000C0FFFFFFFFFFFFFFFFFF,
	
	/// `/75`
	#[cfg(target_endian = "big")] _75 = 0xFFFFFFFFFFFFFFFFFFE0000000000000,
	/// `/75`
	#[cfg(target_endian = "little")] _75 = 0x000000000000E0FFFFFFFFFFFFFFFFFF,
	
	/// `/76`
	#[cfg(target_endian = "big")] _76 = 0xFFFFFFFFFFFFFFFFFFF0000000000000,
	/// `/76`
	#[cfg(target_endian = "little")] _76 = 0x000000000000F0FFFFFFFFFFFFFFFFFF,
	
	/// `/77`
	#[cfg(target_endian = "big")] _77 = 0xFFFFFFFFFFFFFFFFFFF8000000000000,
	/// `/77`
	#[cfg(target_endian = "little")] _77 = 0x000000000000F8FFFFFFFFFFFFFFFFFF,
	
	/// `/78`
	#[cfg(target_endian = "big")] _78 = 0xFFFFFFFFFFFFFFFFFFFC000000000000,
	/// `/78`
	#[cfg(target_endian = "little")] _78 = 0x000000000000FCFFFFFFFFFFFFFFFFFF,
	
	/// `/79`
	#[cfg(target_endian = "big")] _79 = 0xFFFFFFFFFFFFFFFFFFFE000000000000,
	/// `/79`
	#[cfg(target_endian = "little")] _79 = 0x000000000000FEFFFFFFFFFFFFFFFFFF,
	
	/// `/80`
	#[cfg(target_endian = "big")] _80 = 0xFFFFFFFFFFFFFFFFFFFF000000000000,
	/// `/80`
	#[cfg(target_endian = "little")] _80 = 0x000000000000FFFFFFFFFFFFFFFFFFFF,
	
	/// `/81`
	#[cfg(target_endian = "big")] _81 = 0xFFFFFFFFFFFFFFFFFFFF800000000000,
	/// `/81`
	#[cfg(target_endian = "little")] _81 = 0x000000000080FFFFFFFFFFFFFFFFFFFF,
	
	/// `/82`
	#[cfg(target_endian = "big")] _82 = 0xFFFFFFFFFFFFFFFFFFFFC00000000000,
	/// `/82`
	#[cfg(target_endian = "little")] _82 = 0x0000000000C0FFFFFFFFFFFFFFFFFFFF,
	
	/// `/83`
	#[cfg(target_endian = "big")] _83 = 0xFFFFFFFFFFFFFFFFFFFFE00000000000,
	/// `/83`
	#[cfg(target_endian = "little")] _83 = 0x0000000000E0FFFFFFFFFFFFFFFFFFFF,
	
	/// `/84`
	#[cfg(target_endian = "big")] _84 = 0xFFFFFFFFFFFFFFFFFFFFF00000000000,
	/// `/84`
	#[cfg(target_endian = "little")] _84 = 0x0000000000F0FFFFFFFFFFFFFFFFFFFF,
	
	/// `/85`
	#[cfg(target_endian = "big")] _85 = 0xFFFFFFFFFFFFFFFFFFFFF80000000000,
	/// `/85`
	#[cfg(target_endian = "little")] _85 = 0x0000000000F8FFFFFFFFFFFFFFFFFFFF,
	
	/// `/86`
	#[cfg(target_endian = "big")] _86 = 0xFFFFFFFFFFFFFFFFFFFFFC0000000000,
	/// `/86`
	#[cfg(target_endian = "little")] _86 = 0x0000000000FCFFFFFFFFFFFFFFFFFFFF,
	
	/// `/87`
	#[cfg(target_endian = "big")] _87 = 0xFFFFFFFFFFFFFFFFFFFFFE0000000000,
	/// `/87`
	#[cfg(target_endian = "little")] _87 = 0x0000000000FEFFFFFFFFFFFFFFFFFFFF,
	
	/// `/88`
	#[cfg(target_endian = "big")] _88 = 0xFFFFFFFFFFFFFFFFFFFFFF0000000000,
	/// `/88`
	#[cfg(target_endian = "little")] _88 = 0x0000000000FFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/89`
	#[cfg(target_endian = "big")] _89 = 0xFFFFFFFFFFFFFFFFFFFFFF8000000000,
	/// `/89`
	#[cfg(target_endian = "little")] _89 = 0x0000000080FFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/90`
	#[cfg(target_endian = "big")] _90 = 0xFFFFFFFFFFFFFFFFFFFFFFC000000000,
	/// `/90`
	#[cfg(target_endian = "little")] _90 = 0x00000000C0FFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/91`
	#[cfg(target_endian = "big")] _91 = 0xFFFFFFFFFFFFFFFFFFFFFFE000000000,
	/// `/91`
	#[cfg(target_endian = "little")] _91 = 0x00000000E0FFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/92`
	#[cfg(target_endian = "big")] _92 = 0xFFFFFFFFFFFFFFFFFFFFFFF000000000,
	/// `/92`
	#[cfg(target_endian = "little")] _92 = 0x00000000F0FFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/93`
	#[cfg(target_endian = "big")] _93 = 0xFFFFFFFFFFFFFFFFFFFFFFF800000000,
	/// `/93`
	#[cfg(target_endian = "little")] _93 = 0x00000000F8FFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/94`
	#[cfg(target_endian = "big")] _94 = 0xFFFFFFFFFFFFFFFFFFFFFFFC00000000,
	/// `/94`
	#[cfg(target_endian = "little")] _94 = 0x00000000FCFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/95`
	#[cfg(target_endian = "big")] _95 = 0xFFFFFFFFFFFFFFFFFFFFFFFE00000000,
	/// `/95`
	#[cfg(target_endian = "little")] _95 = 0x00000000FEFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/96`
	#[cfg(target_endian = "big")] _96 = 0xFFFFFFFFFFFFFFFFFFFFFFFF00000000,
	/// `/96`
	#[cfg(target_endian = "little")] _96 = 0x00000000FFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/97`
	#[cfg(target_endian = "big")] _97 = 0xFFFFFFFFFFFFFFFFFFFFFFFF80000000,
	/// `/97`
	#[cfg(target_endian = "little")] _97 = 0x00000080FFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/98`
	#[cfg(target_endian = "big")] _98 = 0xFFFFFFFFFFFFFFFFFFFFFFFFC0000000,
	/// `/98`
	#[cfg(target_endian = "little")] _98 = 0x000000C0FFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/99`
	#[cfg(target_endian = "big")] _99 = 0xFFFFFFFFFFFFFFFFFFFFFFFFE0000000,
	/// `/99`
	#[cfg(target_endian = "little")] _99 = 0x000000E0FFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/100`
	#[cfg(target_endian = "big")] _100 = 0xFFFFFFFFFFFFFFFFFFFFFFFFF0000000,
	/// `/100`
	#[cfg(target_endian = "little")] _100 = 0x000000F0FFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/101`
	#[cfg(target_endian = "big")] _101 = 0xFFFFFFFFFFFFFFFFFFFFFFFFF8000000,
	/// `/101`
	#[cfg(target_endian = "little")] _101 = 0x000000F8FFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/102`
	#[cfg(target_endian = "big")] _102 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFC000000,
	/// `/102`
	#[cfg(target_endian = "little")] _102 = 0x000000FCFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/103`
	#[cfg(target_endian = "big")] _103 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFE000000,
	/// `/103`
	#[cfg(target_endian = "little")] _103 = 0x000000FEFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/104`
	#[cfg(target_endian = "big")] _104 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFF000000,
	/// `/104`
	#[cfg(target_endian = "little")] _104 = 0x000000FFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/105`
	#[cfg(target_endian = "big")] _105 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFF800000,
	/// `/105`
	#[cfg(target_endian = "little")] _105 = 0x000080FFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/106`
	#[cfg(target_endian = "big")] _106 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFC00000,
	/// `/106`
	#[cfg(target_endian = "little")] _106 = 0x0000C0FFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/107`
	#[cfg(target_endian = "big")] _107 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFE00000,
	/// `/107`
	#[cfg(target_endian = "little")] _107 = 0x0000E0FFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/108`
	#[cfg(target_endian = "big")] _108 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFF00000,
	/// `/108`
	#[cfg(target_endian = "little")] _108 = 0x0000F0FFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/109`
	#[cfg(target_endian = "big")] _109 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFF80000,
	/// `/109`
	#[cfg(target_endian = "little")] _109 = 0x0000F8FFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/110`
	#[cfg(target_endian = "big")] _110 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFC0000,
	/// `/110`
	#[cfg(target_endian = "little")] _110 = 0x0000FCFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/111`
	#[cfg(target_endian = "big")] _111 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFE0000,
	/// `/111`
	#[cfg(target_endian = "little")] _111 = 0x0000FEFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/112`
	#[cfg(target_endian = "big")] _112 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000,
	/// `/112`
	#[cfg(target_endian = "little")] _112 = 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/113`
	#[cfg(target_endian = "big")] _113 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFF8000,
	/// `/113`
	#[cfg(target_endian = "little")] _113 = 0x0080FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/114`
	#[cfg(target_endian = "big")] _114 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFC000,
	/// `/114`
	#[cfg(target_endian = "little")] _114 = 0x00C0FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/115`
	#[cfg(target_endian = "big")] _115 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFE000,
	/// `/115`
	#[cfg(target_endian = "little")] _115 = 0x00E0FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/116`
	#[cfg(target_endian = "big")] _116 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000,
	/// `/116`
	#[cfg(target_endian = "little")] _116 = 0x00F0FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/117`
	#[cfg(target_endian = "big")] _117 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFF800,
	/// `/117`
	#[cfg(target_endian = "little")] _117 = 0x00F8FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/118`
	#[cfg(target_endian = "big")] _118 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC00,
	/// `/118`
	#[cfg(target_endian = "little")] _118 = 0x00FCFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/119`
	#[cfg(target_endian = "big")] _119 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE00,
	/// `/119`
	#[cfg(target_endian = "little")] _119 = 0x00FEFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/120`
	#[cfg(target_endian = "big")] _120 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00,
	/// `/120`
	#[cfg(target_endian = "little")] _120 = 0x00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/121`
	#[cfg(target_endian = "big")] _121 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF80,
	/// `/121`
	#[cfg(target_endian = "little")] _121 = 0x80FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/122`
	#[cfg(target_endian = "big")] _122 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC0,
	/// `/122`
	#[cfg(target_endian = "little")] _122 = 0xC0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/123`
	#[cfg(target_endian = "big")] _123 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE0,
	/// `/123`
	#[cfg(target_endian = "little")] _123 = 0xE0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/124`
	#[cfg(target_endian = "big")] _124 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0,
	/// `/124`
	#[cfg(target_endian = "little")] _124 = 0xF0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/125`
	#[cfg(target_endian = "big")] _125 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8,
	/// `/125`
	#[cfg(target_endian = "little")] _125 = 0xF8FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/126`
	#[cfg(target_endian = "big")] _126 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC,
	/// `/126`
	#[cfg(target_endian = "little")] _126 = 0xFCFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/127`
	#[cfg(target_endian = "big")] _127 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE,
	/// `/127`
	#[cfg(target_endian = "little")] _127 = 0xFEFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	
	/// `/128`
	#[cfg(target_endian = "big")] _128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
	/// `/128`
	#[cfg(target_endian = "little")] _128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
}

impl Display for InternetProtocolVersion6MaskBits
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::InternetProtocolVersion6MaskBits::*;
		
		let string = match *self
		{
			_0 => "0",
			_1 => "1",
			_2 => "2",
			_3 => "3",
			_4 => "4",
			_5 => "5",
			_6 => "6",
			_7 => "7",
			_8 => "8",
			_9 => "9",
			_10 => "10",
			_11 => "11",
			_12 => "12",
			_13 => "13",
			_14 => "14",
			_15 => "15",
			_16 => "16",
			_17 => "17",
			_18 => "18",
			_19 => "19",
			_20 => "20",
			_21 => "21",
			_22 => "22",
			_23 => "23",
			_24 => "24",
			_25 => "25",
			_26 => "26",
			_27 => "27",
			_28 => "28",
			_29 => "29",
			_30 => "30",
			_31 => "31",
			_32 => "32",
			_33 => "33",
			_34 => "34",
			_35 => "35",
			_36 => "36",
			_37 => "37",
			_38 => "38",
			_39 => "39",
			_40 => "40",
			_41 => "41",
			_42 => "42",
			_43 => "43",
			_44 => "44",
			_45 => "45",
			_46 => "46",
			_47 => "47",
			_48 => "48",
			_49 => "49",
			_50 => "50",
			_51 => "51",
			_52 => "52",
			_53 => "53",
			_54 => "54",
			_55 => "55",
			_56 => "56",
			_57 => "57",
			_58 => "58",
			_59 => "59",
			_60 => "60",
			_61 => "61",
			_62 => "62",
			_63 => "63",
			_64 => "64",
			_65 => "65",
			_66 => "66",
			_67 => "67",
			_68 => "68",
			_69 => "69",
			_70 => "70",
			_71 => "71",
			_72 => "72",
			_73 => "73",
			_74 => "74",
			_75 => "75",
			_76 => "76",
			_77 => "77",
			_78 => "78",
			_79 => "79",
			_80 => "80",
			_81 => "81",
			_82 => "82",
			_83 => "83",
			_84 => "84",
			_85 => "85",
			_86 => "86",
			_87 => "87",
			_88 => "88",
			_89 => "89",
			_90 => "90",
			_91 => "91",
			_92 => "92",
			_93 => "93",
			_94 => "94",
			_95 => "95",
			_96 => "96",
			_97 => "97",
			_98 => "98",
			_99 => "99",
			_100 => "100",
			_101 => "101",
			_102 => "102",
			_103 => "103",
			_104 => "104",
			_105 => "105",
			_106 => "106",
			_107 => "107",
			_108 => "108",
			_109 => "109",
			_110 => "110",
			_111 => "111",
			_112 => "112",
			_113 => "113",
			_114 => "114",
			_115 => "115",
			_116 => "116",
			_117 => "117",
			_118 => "118",
			_119 => "119",
			_120 => "120",
			_121 => "121",
			_122 => "122",
			_123 => "123",
			_124 => "124",
			_125 => "125",
			_126 => "126",
			_127 => "127",
			_128 => "128",
		};
		write!(f, "{}", string)
	}
}

impl InternetProtocolMaskBits for InternetProtocolVersion6MaskBits
{
	#[inline(always)]
	fn as_depth(self) -> u8
	{
		let mask_bits = self as u128;
		if cfg!(target_endian = "little")
		{
			mask_bits.count_ones() as u8
		}
		else
		{
			(!mask_bits).trailing_zeros() as u8
		}
	}
}
