use std::collections::BTreeMap;
use crate::{add, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteUnit { DecimalGb, BinaryGib }
impl ByteUnit {
    pub fn bytes(self) -> u128 { match self { Self::DecimalGb => 1_000_000_000, Self::BinaryGib => 1_073_741_824 } }
}
/// Prices are token minor units per byte unit per explicit number of seconds.
/// A "month" is never inferred: a 30-day month is 2_592_000 seconds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rate {
    pub token: String,
    pub minor_units_per_token: u128,
    pub price_minor: u128,
    pub byte_unit: ByteUnit,
    pub period_seconds: u128,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval { pub id: String, pub start: i64, pub end: i64, pub bytes: u128 }
#[derive(Debug, Clone)]
pub struct Meter { rate: Rate, intervals: BTreeMap<String, Interval> }
impl Meter {
    pub fn new(rate: Rate) -> Result<Self> {
        if rate.token.is_empty() || rate.minor_units_per_token == 0 || rate.period_seconds == 0 { return Err(Error::Invalid); }
        rate.byte_unit.bytes().checked_mul(rate.period_seconds).ok_or(Error::Overflow)?;
        Ok(Self { rate, intervals: BTreeMap::new() })
    }
    pub fn record(&mut self, interval: Interval) -> Result<()> {
        if interval.id.is_empty() || interval.start < 0 || interval.end <= interval.start { return Err(Error::Invalid); }
        if let Some(old) = self.intervals.get(&interval.id) {
            return if old == &interval { Ok(()) } else { Err(Error::DuplicateMismatch) };
        }
        if self.intervals.values().any(|x| interval.start < x.end && x.start < interval.end) { return Err(Error::Overlap); }
        self.intervals.insert(interval.id.clone(), interval);
        Ok(())
    }
    /// Requires complete [start,end) coverage. Intersecting intervals are clipped.
    /// Sum exact byte-second price numerators, then floor once to token minor units.
    pub fn charge(&self, start: i64, end: i64) -> Result<u128> {
        if start < 0 || end <= start { return Err(Error::Invalid); }
        let mut rows: Vec<_> = self.intervals.values().filter(|x| x.start < end && start < x.end).collect();
        rows.sort_by_key(|x| x.start);
        if rows.is_empty() { return Err(Error::Missing); }
        let mut cursor = start;
        let mut numerator = 0;
        for x in rows {
            let left = x.start.max(start);
            let right = x.end.min(end);
            if left != cursor { return Err(Error::Gap); }
            let seconds = right.checked_sub(left).ok_or(Error::Overflow)? as u128;
            let term = x.bytes.checked_mul(seconds).and_then(|v| v.checked_mul(self.rate.price_minor)).ok_or(Error::Overflow)?;
            numerator = add(numerator, term)?;
            cursor = right;
        }
        if cursor != end { return Err(Error::Gap); }
        let denominator = self.rate.byte_unit.bytes().checked_mul(self.rate.period_seconds).ok_or(Error::Overflow)?;
        Ok(numerator / denominator)
    }
}
