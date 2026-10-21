use std::collections::BTreeSet;
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile { PublicCpu, TrustedCpu, ConfidentialCpu, PublicGpu, TrustedGpu, ConfidentialGpu }
impl Profile {
    fn sensitive(self) -> bool { !matches!(self, Self::PublicCpu | Self::PublicGpu) }
}

/// Unverified caller assertions, never a signature, attestation or authority claim.
/// Timestamps use the same caller-selected seconds clock as `select`'s `now`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub provider: String, pub profile: Profile, pub epoch: u64,
    pub issued: u64, pub expires: u64, pub challenge: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offer {
    pub provider: String, pub region: String, pub capacity: u64, pub price_minor: u128,
    /// Exact identifier for currency/token minor-unit scale AND service quantum.
    /// Example: "synthetic-USDC:10^-6/slot-second". No conversion is performed.
    pub billing_unit: String,
    pub profile: Profile, pub epoch: u64, pub expires: u64, pub evidence: Option<Evidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workload {
    pub region: String, pub capacity: u64, pub budget_minor: u128,
    pub billing_unit: String, pub profile: Profile,
    /// Positive inclusive maximum age in seconds, required for sensitive profiles.
    pub max_evidence_age_seconds: u64,
    /// Required for sensitive profiles: nonempty and not all zero bytes.
    /// The caller must supply a fresh session challenge; uniqueness is not tracked.
    pub challenge: Vec<u8>,
}

/// Advisory selection only: no capacity reservation or workload admission effect.
/// Profiles match exactly; CPU evidence cannot satisfy GPU requirements.
/// A trusted-profile reference record does not establish a TEE claim.
pub fn select<'a>(offers: &'a [Offer], work: &Workload, epoch: u64, now: u64) -> Result<&'a Offer> {
    if work.region.is_empty() || work.capacity == 0 || work.billing_unit.trim().is_empty() {
        return Err(Error::Invalid);
    }
    if work.profile.sensitive()
        && (work.max_evidence_age_seconds == 0 || work.challenge.iter().all(|byte| *byte == 0))
    {
        return Err(Error::Invalid);
    }
    let mut ids = BTreeSet::new();
    for offer in offers {
        if offer.provider.is_empty() || offer.region.is_empty()
            || offer.billing_unit.trim().is_empty() || !ids.insert(&offer.provider)
        {
            return Err(Error::Invalid);
        }
    }
    offers.iter().filter(|o| {
        o.region == work.region && o.capacity >= work.capacity
        && o.billing_unit == work.billing_unit && o.price_minor <= work.budget_minor
        && o.profile == work.profile && o.epoch == epoch && now < o.expires
        && (!work.profile.sensitive() || o.evidence.as_ref().is_some_and(|e| {
            e.provider == o.provider && e.profile == work.profile && e.epoch == epoch
            && e.challenge == work.challenge && now < e.expires
            && now.checked_sub(e.issued).is_some_and(|age| age <= work.max_evidence_age_seconds)
        }))
    }).min_by(|a, b| a.price_minor.cmp(&b.price_minor).then(a.provider.cmp(&b.provider))).ok_or(Error::NoEligible)
}
