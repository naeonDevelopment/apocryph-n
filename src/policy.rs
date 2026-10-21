use crate::{Error, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recommendation { Continue, StopNewAdmission, Drain, Export, Retain }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Thresholds { pub stop: u128, pub drain: u128, pub export: u128 }
/// Recommendations only. Retain means no automatic deletion or key release.
pub fn recommend(remaining_minor: u128, thresholds: Thresholds) -> Result<Recommendation> {
    if !(thresholds.stop > thresholds.drain && thresholds.drain > thresholds.export && thresholds.export > 0) { return Err(Error::Invalid); }
    Ok(if remaining_minor == 0 { Recommendation::Retain }
    else if remaining_minor <= thresholds.export { Recommendation::Export }
    else if remaining_minor <= thresholds.drain { Recommendation::Drain }
    else if remaining_minor <= thresholds.stop { Recommendation::StopNewAdmission }
    else { Recommendation::Continue })
}
