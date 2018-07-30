// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// A monotonically increasing timestamp in milliseconds.
///
/// This is not necessarily the system clock.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MonotonicMillisecondTimestamp(u64);

impl Display for MonotonicMillisecondTimestamp
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Default for MonotonicMillisecondTimestamp
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::now()
	}
}

impl Into<u32> for MonotonicMillisecondTimestamp
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self.0 as u32
	}
}

impl Add<MillisecondDuration> for MonotonicMillisecondTimestamp
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: MillisecondDuration) -> Self::Output
	{
		MonotonicMillisecondTimestamp(self.0 + rhs.0)
	}
}

impl Sub for MonotonicMillisecondTimestamp
{
	type Output = MillisecondDuration;
	
	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		MillisecondDuration(self.0 - rhs.0)
	}
}

impl MonotonicMillisecondTimestamp
{
	/// Zero.
	pub const Zero: Self = MonotonicMillisecondTimestamp(0);
	
	/// Time now.
	#[inline(always)]
	pub fn now() -> Self
	{
		MonotonicMillisecondTimestamp(Self::cycles_since_boot() / Self::cycles_per_millisecond())
	}
	
	/// Microseconds since boot.
	#[inline(always)]
	pub fn microseconds_since_boot() -> u64
	{
		Self::cycles_since_boot() / Self::cycles_per_microsecond()
	}
	
	#[inline(always)]
	fn cycles_per_millisecond() -> u64
	{
		const Uninitialized: u64 = 0;
		
		static mut CyclesPerMillisecond: u64 = Uninitialized;
		
		let cycles_per_millisecond = unsafe { CyclesPerMillisecond };
		if unlikely!(cycles_per_millisecond == Uninitialized)
		{
			let cycles_per_second_in_hertz = match Self::get_frequency_for_architecture()
			{
				Some(frequency) => frequency,
				None => Self::estimate_frequency(),
			};
			
			let cycles_per_millisecond = cycles_per_second_in_hertz / 1_000;
			
			unsafe { CyclesPerMillisecond = cycles_per_millisecond };
			cycles_per_millisecond
		}
		else
		{
			cycles_per_millisecond
		}
	}
	
	#[inline(always)]
	fn cycles_per_microsecond() -> u64
	{
		const Uninitialized: u64 = 0;
		
		static mut CyclesPerMicrosecond: u64 = Uninitialized;
		
		let cycles_per_microsecond = unsafe { CyclesPerMicrosecond };
		if unlikely!(cycles_per_microsecond == Uninitialized)
		{
			let cycles_per_second_in_hertz = match Self::get_frequency_for_architecture()
			{
				Some(frequency) => frequency,
				None => Self::estimate_frequency(),
			};
			
			let cycles_per_microsecond = cycles_per_second_in_hertz / 1_000_000;
			
			unsafe { CyclesPerMicrosecond = cycles_per_microsecond };
			cycles_per_microsecond
		}
		else
		{
			cycles_per_microsecond
		}
	}
	
	#[inline(always)]
	fn estimate_frequency() -> u64
	{
		let start = Self::cycles_since_boot();
		sleep(Duration::from_secs(1));
		let end = Self::cycles_since_boot();
		
		end - start
	}
	
	#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
	#[inline(always)]
	fn get_frequency_for_architecture() -> Option<u64>
	{
		None
	}
	
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	#[inline(always)]
	fn get_frequency_for_architecture() -> Option<u64>
	{
		#[cfg(target_arch = "x86")] use ::std::arch::x86::__cpuid;
		#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__cpuid;
		#[cfg(target_arch = "x86")] use ::std::arch::x86::__get_cpuid_max;
		#[cfg(target_arch = "x86_64")] use ::std::arch::x86_64::__get_cpuid_max;
	
		#[inline(always)]
		fn get_cpu_model(family_model_stepping: u32) -> u8
		{
			let family = (family_model_stepping >> 8) & 0xF;
			let base_model = ((family_model_stepping >> 4) & 0xF) as u8;
	
			match family
			{
				6 | 15 =>
				{
					let extended_model = ((family_model_stepping >> 16) & 0xF) as u8;
					(extended_model << 4) + base_model
				}
	
				_ => base_model,
			}
		}
	
		#[inline(always)]
		fn is_cpu_model_westmere_or_nehalem(model: u8) -> bool
		{
			const IsWestmere: bool = true;
			const IsNehalem: bool = true;
	
			match model
			{
				0x25 | 0x2C | 0x2F => IsWestmere,
				0x1E | 0x1F | 0x1A | 0x2E => IsNehalem,
				_ => false,
			}
		}
	
		#[inline(always)]
		fn is_cpu_model_goldmont_or_denverton(model: u8) -> bool
		{
			const IsGoldmont: bool = true;
			const IsDenverton: bool = true;
	
			match model
			{
				0x5C => IsGoldmont,
				0x5F => IsDenverton,
				_ => false,
			}
		}
	
		const Leaf0x15: u32 = 0x15;
	
		let maximum_leaf = (unsafe { __get_cpuid_max(0) }).0;
	
		if maximum_leaf >= Leaf0x15
		{
			let result = unsafe { __cpuid(Leaf0x15) };
			// `ebx` is the TimestampCounter: Crystal ratio and `ecx` is the Crystal's hertz.
			if result.ebx & result.ecx != 0
			{
				return Some((result.ecx * (result.ebx / result.eax)) as u64)
			}
		}
	
		if cfg!(target_os = "linux")
		{
			let result = unsafe { __cpuid(0x1) };
			let cpu_model = get_cpu_model(result.eax);
	
			// Defined in <cpuid.h>, part of clang or gcc's standard library.
			const bit_AVX: u32 = 0x10000000;
	
			let multiplier = if is_cpu_model_westmere_or_nehalem(cpu_model)
			{
				133 * 1_000_000
			}
			else if (result.ecx & bit_AVX != 0) || is_cpu_model_goldmont_or_denverton(cpu_model)
			{
				100 * 1_000_000
			}
			else
			{
				return None
			};
	
			#[inline(always)]
			fn read_model_specific_register() -> Result<u64, Box<error::Error + 'static>>
			{
				let model_specific_register: u64 = String::from_utf8_lossy(&read("/dev/cpu/0/msr")?).parse()?;
				Ok(model_specific_register)
			}
	
			match read_model_specific_register()
			{
				Err(_) => None,
				Ok(model_specific_register) => Some(((model_specific_register >> 8) & 0xFF) * multiplier),
			}
		}
		else
		{
			None
		}
	}
	
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	#[inline(always)]
	fn cycles_since_boot() -> u64
	{
		#[derive(Copy, Clone)]
		#[repr(C, packed)]
		struct CyclesU32Addressable
		{
			low: u32,
			high: u32,
		}
	
		#[repr(C, packed)]
		union Cycles
		{
			cycles: u64,
			cycles_u32_addressable: CyclesU32Addressable,
	
		}
	
		unsafe
		{
			let mut cycles: Cycles = uninitialized();
			
			asm!
			(
				"rdtsc"
				:
					"=a" (cycles.cycles_u32_addressable.low),
					"=d" (cycles.cycles_u32_addressable.high)
				:
				:
				:
					"volatile"
			);
			
			cycles.cycles
		}
	}
	
	#[cfg(target_arch = "powerpc64")]
	#[inline(always)]
	fn cycles_since_boot() -> u64
	{
		#[cfg(target_endian = "big")]
		#[derive(Copy, Clone)]
		#[repr(C, packed)]
		struct CyclesU32Addressable
		{
			high: u32,
			low: u32,
		}
	
		#[cfg(target_endian = "little")]
		#[derive(Copy, Clone)]
		#[repr(C, packed)]
		struct CyclesU32Addressable
		{
			low: u32,
			high: u32,
		}
	
		#[repr(C, packed)]
		union Cycles
		{
			cycles: u64,
			cycles_u32_addressable: CyclesU32Addressable,
	
		}
		
		unsafe
		{
			let mut cycles: Cycles = uninitialized();
			let mut temporary: u32 = uninitialized();
			
			asm!
			(
				"0:
mftbu %[high]
mftb  %[low]
mftbu %[temporary]
cmpw  %[temporary],%[high]
bne   0b
"
				:
					[high] "=r"(cycles.cycles_u32_addressable.high),
					[low] "=r"(cycles.cycles_u32_addressable.low),
					[temporary] "=r"(temporary)
				:
				:
				:
					"volatile"
			);
			
			cycles.cycles
		}
	}
	
	#[cfg(target_arch = "aarch64")]
	#[inline(always)]
	fn cycles_since_boot() -> u64
	{
		unsafe
		{
			let mut cycles: u64 = uninitialized();
			
			asm!
			(
				"mrs %0, cntvct_el0"
				:
					"=r" (cycles)
				:
				:
				:
					"volatile"
			);
			
			cycles
		}
	}
}
