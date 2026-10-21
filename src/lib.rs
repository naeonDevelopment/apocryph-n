#![forbid(unsafe_code)]
//! Reference semantics with an explicit local commerce journal. No network or payment effects.
pub mod admission;
pub mod metering;
pub mod policy;
pub mod settlement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Invalid, Overflow, DuplicateMismatch, Overlap, Gap, Missing,
    Insufficient, Transition, NoEligible,
}
pub type Result<T> = std::result::Result<T, Error>;
pub(crate) fn add(a: u128, b: u128) -> Result<u128> {
    a.checked_add(b).ok_or(Error::Overflow)
}

pub mod commerce;
pub mod commerce_wire;
pub mod commerce_store;
