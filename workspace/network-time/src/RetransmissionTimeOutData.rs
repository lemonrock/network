// This file is part of network. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT. No part of network, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of network. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/network/master/COPYRIGHT.


/// Maintains the state of the retransmission time out timer.
///
/// * Calculates according to RFC 6298
/// * Resets the interpretation of the measurement of round-trip times once a threshold of back off doubling is reached (MaximumNumberOfBackOffsBeforeResettingMeasurements).
///
/// This logic is based on RFC 6298, which obsoleted RFC 2988 (which itself clarified RFC 1122 and RFC 793 and complemented RFC 2581).
///
/// In practice, RFC 6298 is extremely similar to RFC 2988.
///
/// This timer is used with congestion control; see RFC 5681 (which itself obsoletes RFC 2581).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RetransmissionTimeOutData
{
	/// `SRTT`.
	smoothed_round_trip_time: MillisecondDuration,
	
	/// `RTTVAR`.
	round_trip_time_variance: MillisecondDuration,
	
	/// `RTO`.
	retransmission_time_out: MillisecondDuration,
	
	number_of_retransmissions: u8,
	
	back_off_scalars: &'static [u64; RetransmissionTimeOutData::MaximumNumberOfRetransmissions],
}

static NonSynchronizedStateBackOffScalars: [u64; RetransmissionTimeOutData::MaximumNumberOfRetransmissions] =
[
	0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0
];

static AllOtherStatesBackOffScalars: [u64; RetransmissionTimeOutData::MaximumNumberOfRetransmissions] =
[
	1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0
];

impl RetransmissionTimeOutData
{
	const MaximumNumberOfRetransmissions: usize = 12;
	
	const ExclusiveMaximumNumberOfTransmissionsBeforeResetingMeasurementOfRoundTripTime: usize = Self::MaximumNumberOfRetransmissions / 4;
	
	const ClockGranularity: MillisecondDuration = MillisecondDuration::from_milliseconds(Tick::MillisecondsPerTick);
	
	/// RFC 6298 Section 2.1: "Until a round-trip time (RTT) measurement has been made for a segment sent between the sender and receiver, the sender SHOULD set RTO <- 1 second, though the "backing off" on repeated retransmission discussed in (5.5) still applies.
	///
	/// RFC 6298 Section 2.4: "Whenever RTO is computed, if it is less than 1 second then the RTO SHOULD be rounded up to 1 second".
	///
	/// NOTE: We VIOLATE the RFC here by choosing a minimum of 128 milliseconds.
	///
	/// This is more inline with modern TCP stacks.
	/// For example, Linux defaults to 200ms and FreeBSD to approximately 30 ms (1/33 second).
	const MinimumRetransmissionTimeOut: MillisecondDuration = MillisecondDuration::from_milliseconds(256);
	
	/// RFC 6298 Section 2.4 here ("A maximum value MAY be placed on RTO provided it is at least 60 seconds").
	const MaximumRetransmissionTimeOut: MillisecondDuration = MillisecondDuration::OneMinute;
	
	/// This is the reverse of the logic in `compute_retransmission_time_out()`.
	pub const InitialSmoothedRoundTripTime: MillisecondDuration = MillisecondDuration(Self::MinimumRetransmissionTimeOut.0 / 3);
	
	/// This matches the logic in `first_measurement_of_round_trip_time_made()`.
	pub const InitialRoundTripTimeVariance: MillisecondDuration = MillisecondDuration(Self::InitialSmoothedRoundTripTime.0 / 2);
	
	// RFC 6298 Section 5.7: "If the timer expires awaiting the ACK of a SYN segment and the TCP implementation is using an RTO less than 3 seconds, the RTO MUST be re-initialized to 3 seconds when data transmission begins (i.e., after the three-way handshake completes)."
	//
	// We VIOLATE the RFC here and use 3 × 128 ms.
	const ReInitializeRetransmissionTimeOut: MillisecondDuration = MillisecondDuration((Self::MinimumRetransmissionTimeOut.0) * 3);
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(smoothed_round_trip_time: MillisecondDuration, round_trip_time_variance: MillisecondDuration, is_for_non_synchronized_state: bool) -> Self
	{
		Self
		{
			smoothed_round_trip_time,
			round_trip_time_variance,
			retransmission_time_out: Self::compute_retransmission_time_out(smoothed_round_trip_time, round_trip_time_variance),
			number_of_retransmissions: 0,
			back_off_scalars: if is_for_non_synchronized_state
			{
				&NonSynchronizedStateBackOffScalars
			}
			else
			{
				&AllOtherStatesBackOffScalars
			}
		}
	}
	
	/// Current retransmission time out, `RTO`.
	#[inline(always)]
	pub fn retransmission_time_out(&self) -> MillisecondDuration
	{
		self.retransmission_time_out
	}
	
	/// Increment retransmissions.
	#[inline(always)]
	pub fn increment_retransmissions(&mut self) -> Option<u8>
	{
		let number_of_retransmissions = self.number_of_retransmissions;
		
		if number_of_retransmissions >= Self::MaximumNumberOfRetransmissions as u8
		{
			return None
		}
		
		// RFC 6298 Section 5: "(5.5) The host MUST set RTO <- RTO * 2 ("back off the timer").
		// The maximum value discussed in (2.5) above may be used to provide an upper bound to this doubling operation".
		//
		// We VIOLATE the RFC by using an array of scalars.
		let back_off_scalar = unsafe { * self.back_off_scalars.get_unchecked(number_of_retransmissions as usize) };
		
		let unclamped_retransmission_time_out = self.retransmission_time_out << back_off_scalar;
		
		self.retransmission_time_out = if unclamped_retransmission_time_out < Self::MinimumRetransmissionTimeOut
		{
			Self::MinimumRetransmissionTimeOut
		}
		else if unclamped_retransmission_time_out > Self::MaximumRetransmissionTimeOut
		{
			Self::MaximumRetransmissionTimeOut
		}
		else
		{
			unclamped_retransmission_time_out
		};
		
		self.number_of_retransmissions += 1;
		Some(number_of_retransmissions)
	}
	
	/// Reset retransmissions.
	#[inline(always)]
	pub fn reset_retransmissions(&mut self)
	{
		self.number_of_retransmissions = 0
	}
	
	/// Reset retransmissions after establishing state if we sent the first Synchronize segment and the retransmission timer fired at least once.
	#[inline(always)]
	pub fn reset_after_establishment_of_state_if_we_sent_the_first_synchronize_segment_and_the_timer_expired(&mut self)
	{
		if self.number_of_retransmissions != 0
		{
			self.retransmission_time_out = Self::ReInitializeRetransmissionTimeOut;
			self.number_of_retransmissions = 0
		}
	}
	
	/// Entering established state; a different set of back off scalars will now apply.
	#[inline(always)]
	pub fn entering_established_state(&mut self)
	{
		if self.retransmission_time_out() < Self::ReInitializeRetransmissionTimeOut
		{
			self.retransmission_time_out = Self::ReInitializeRetransmissionTimeOut;
		}
		
		self.back_off_scalars = &AllOtherStatesBackOffScalars;
		
		self.number_of_retransmissions = 0;
	}
	
	/// Process a round trip time measurement.
	#[inline(always)]
	pub fn process_measurement_of_round_trip_time(&mut self, measurement_of_round_trip_time: MillisecondDuration)
	{
		self.number_of_retransmissions = 0;
		
		if unlikely!(measurement_of_round_trip_time.is_zero())
		{
			return
		}
		
		if self.round_trip_time_measurements_are_bogus()
		{
			self.first_measurement_of_round_trip_time_made(measurement_of_round_trip_time);
		}
		else
		{
			self.subsequent_measurement_of_round_trip_time(measurement_of_round_trip_time);
		}
	}
	
	/// Obtain current smoothed round trip time, `SRTT`, and round trip time variances, `RTTVAR`.
	#[inline(always)]
	pub fn smoothed_round_trip_time_and_round_trip_time_variance(&self) -> (MillisecondDuration, MillisecondDuration)
	{
		if self.round_trip_time_measurements_are_bogus()
		{
			(Self::InitialSmoothedRoundTripTime, Self::InitialRoundTripTimeVariance)
		}
		else
		{
			(self.smoothed_round_trip_time, self.round_trip_time_variance)
		}
	}
	
	// RFC 6298 Section 5, final paragraph: "Note that a TCP implementation MAY clear SRTT and RTTVAR after backing off the timer multiple times as it is likely that the current SRTT and RTTVAR are bogus in this situation.
	// Once SRTT and RTTVAR are cleared, they should be initialized with the next RTT sample taken per (2.2) rather than using (2.3)".
	#[inline(always)]
	fn round_trip_time_measurements_are_bogus(&self) -> bool
	{
		self.number_of_retransmissions > (Self::ExclusiveMaximumNumberOfTransmissionsBeforeResetingMeasurementOfRoundTripTime as u8) || (self.smoothed_round_trip_time == Self::InitialSmoothedRoundTripTime && self.round_trip_time_variance == Self::InitialRoundTripTimeVariance)
	}
	
	/// RFC 6298 Section 2.2 & RFC 2988 Section 2.2: "When the first RTT measurement R is made, the host MUST set
	/// SRTT <- R
	/// RTTVAR <- R/2
	/// RTO <- SRTT + max (G, K*RTTVAR)
	/// where K = 4.
	/// ".
	#[inline(always)]
	#[allow(non_snake_case)]
	fn first_measurement_of_round_trip_time_made(&mut self, measurement_of_round_trip_time: MillisecondDuration)
	{
		let R = measurement_of_round_trip_time;
		
		self.smoothed_round_trip_time = R;
		self.round_trip_time_variance = R / 2;
		self.recompute_retransmission_time_out();
	}
	
	/// RFC 6298 Section 2.3 & RFC 2988 Section 2.3: "When a subsequent RTT measurement R' is made, a host MUST set
	/// RTTVAR <- (1 - beta) * RTTVAR + beta * |SRTT - R'|
	/// SRTT <- (1 - alpha) * SRTT + alpha * R'
	///
	/// The value of SRTT used in the update to RTTVAR is its value before updating SRTT itself using the second assignment.
	/// That is, updating RTTVAR and SRTT MUST be computed in the above order.
	///
	/// The above SHOULD be computed using alpha=1/8 and beta=1/4 (as suggested in JK88 (Jacobson, V. and M. Karels, Congestion Avoidance and Control)).
	///
	/// After the computation, a host MUST update RTO <- SRTT + max (G, K*RTTVAR)".
	#[inline(always)]
	#[allow(non_snake_case)]
	fn subsequent_measurement_of_round_trip_time(&mut self, measurement_of_round_trip_time: MillisecondDuration)
	{
		// R'.
		let Rdash = measurement_of_round_trip_time;
		
		// Nominally, `beta` is ¼ and `1 - beta` is ¾.
		// Thus `RTTVAR = (1 - beta) * RTTVAR + beta * |SRTT - R'|` is actually `SRTT = ¾ * self.round_trip_time_variance + ¼ * |SRTT - R'|`; we can then multiply by 4 to get:-
		// `4 * RTTVAR = 3 * self.round_trip_time_variance + |SRTT - R'|`.
		// `RTTVAR = (3 * self.round_trip_time_variance + |SRTT - R'|) / 4`.
		// So, instead of `self.round_trip_time_variance = (1 - beta) * self.round_trip_time_variance + beta + self.smoothed_round_trip_time.absolute_difference(Rdash)`, we have:-
		self.round_trip_time_variance = (self.round_trip_time_variance * 3 + self.smoothed_round_trip_time.absolute_difference(Rdash)) / 4;
		
		// Nominally, `alpha` is ¹⁄₈ and `1 - alpha` is ⁷⁄₈
		// Thus, `SRTT = (1 - alpha) * SRTT + alpha * R'` is actually `SRTT = ⁷⁄₈ * SRTT + ¹⁄₈ * R'`; we can then multiply by 8 to get:-
		// `8 * SRTT = 7 * SRTT + R'` .
		// `SRTT = (7 * SRTT + R') / 8.
		// So, instead of `self.smoothed_round_trip_time = (1 - alpha) * self.smoothed_round_trip_time + alpha * Rdash`, we have:-
		self.smoothed_round_trip_time = (self.smoothed_round_trip_time * 7 + Rdash) / 8;
		
		self.recompute_retransmission_time_out();
	}
	
	#[inline(always)]
	fn recompute_retransmission_time_out(&mut self)
	{
		self.retransmission_time_out = Self::compute_retransmission_time_out(self.smoothed_round_trip_time, self.round_trip_time_variance);
	}
	
	#[inline(always)]
	fn compute_retransmission_time_out(smoothed_round_trip_time: MillisecondDuration, round_trip_time_variance: MillisecondDuration) -> MillisecondDuration
	{
		// Clock granularity of `G` milliseconds.
		const G: MillisecondDuration = RetransmissionTimeOutData::ClockGranularity;
		
		const K: u64 = 4;
		
		smoothed_round_trip_time + max(G, round_trip_time_variance * K)
	}
}
