use apocryph_n::{admission::*, metering::*, policy::*, settlement::*, Error};
fn rate(unit: ByteUnit, price: u128, seconds: u128) -> Rate {
    Rate { token: "micro-USDC (synthetic)".into(), minor_units_per_token: 1_000_000, price_minor: price, byte_unit: unit, period_seconds: seconds }
}
fn interval(id: &str, start: i64, end: i64, bytes: u128) -> Interval { Interval { id: id.into(), start, end, bytes } }
fn ledger() -> Settlement {
    let mut s = Settlement::new("synthetic-app".into(), "v7".into()).unwrap();
    s.accrue_to(100).unwrap(); s.invoice_to(100).unwrap(); s
}
fn offer(id: &str, price: u128) -> Offer {
    Offer { provider: id.into(), region: "test-region".into(), capacity: 4, price_minor: price, billing_unit: "synthetic-USDC:10^-6/slot-second".into(), profile: Profile::PublicCpu, epoch: 3, expires: 20, evidence: None }
}
fn workload(profile: Profile) -> Workload { Workload { region: "test-region".into(), capacity: 2, budget_minor: 100, billing_unit: "synthetic-USDC:10^-6/slot-second".into(), profile, max_evidence_age_seconds: 10, challenge: vec![1, 2, 3, 4] } }
#[test] fn decimal_month_regression() {
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 2500, 2_592_000)).unwrap();
    m.record(interval("a", 0, 2_592_000, 1_000_000_000)).unwrap();
    assert_eq!(m.charge(0, 2_592_000), Ok(2500));
}
#[test] fn gib_is_not_gb() {
    let mut m = Meter::new(rate(ByteUnit::BinaryGib, 2500, 1)).unwrap();
    m.record(interval("a", 0, 1, 1_073_741_824)).unwrap(); assert_eq!(m.charge(0, 1), Ok(2500));
    let mut n = Meter::new(rate(ByteUnit::DecimalGb, 2500, 1)).unwrap();
    n.record(interval("a", 0, 1, 1_073_741_824)).unwrap(); assert_eq!(n.charge(0, 1), Ok(2684));
}
#[test] fn cumulative_fraction_survives_partition() {
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 1, 2)).unwrap();
    m.record(interval("a", 0, 1, 1_000_000_000)).unwrap(); m.record(interval("b", 1, 2, 1_000_000_000)).unwrap();
    assert_eq!(m.charge(0, 1), Ok(0)); assert_eq!(m.charge(0, 2), Ok(1));
}
#[test] fn duplicate_and_overlap_are_distinct() {
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 1, 1)).unwrap(); let x = interval("a", 0, 2, 1);
    m.record(x.clone()).unwrap(); m.record(x).unwrap();
    assert_eq!(m.record(interval("a", 0, 2, 2)), Err(Error::DuplicateMismatch));
    assert_eq!(m.record(interval("b", 1, 3, 1)), Err(Error::Overlap));
}
#[test] fn unknown_coverage_never_becomes_zero() {
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 0, 1)).unwrap();
    assert_eq!(m.charge(0, 3), Err(Error::Missing));
    m.record(interval("a", 0, 1, 0)).unwrap(); m.record(interval("b", 2, 3, 0)).unwrap();
    assert_eq!(m.charge(0, 3), Err(Error::Gap)); assert_eq!(m.charge(0, 1), Ok(0));
}
#[test] fn bad_durations_and_rate_rejected() {
    assert!(Meter::new(rate(ByteUnit::DecimalGb, 1, 0)).is_err());
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 1, 1)).unwrap();
    for x in [interval("a", 1, 1, 0), interval("b", 2, 1, 0), interval("c", -1, 1, 0)] { assert_eq!(m.record(x), Err(Error::Invalid)); }
}
#[test] fn overflow_is_explicit() {
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 2, 1)).unwrap();
    m.record(interval("a", 0, 1, u128::MAX)).unwrap(); assert_eq!(m.charge(0, 1), Err(Error::Overflow));
    assert!(Meter::new(rate(ByteUnit::BinaryGib, 1, u128::MAX)).is_err());
}
#[test] fn constructor_keeps_version() { assert_eq!(ledger().identity(), ("synthetic-app", "v7")); }
#[test] fn reserve_excludes_second_collector() {
    let mut s = ledger(); s.reserve("a".into(), 100).unwrap(); s.reserve("a".into(), 100).unwrap();
    assert_eq!(s.reserve("b".into(), 1), Err(Error::Insufficient));
    assert_eq!(s.reserve("a".into(), 99), Err(Error::DuplicateMismatch)); assert_eq!(s.balances().reserved, 100);
}
#[test] fn timeout_blocks_until_reconciled() {
    let mut s = ledger(); s.reserve("a".into(), 100).unwrap(); s.transition("a", State::Unknown, false).unwrap();
    assert_eq!(s.reserve("b".into(), 100), Err(Error::Insufficient));
    assert_eq!(s.transition("a", State::Failed, false), Err(Error::Transition));
    s.transition("a", State::Failed, true).unwrap(); s.reserve("b".into(), 100).unwrap();
}
#[test] fn success_and_refund_are_once_only() {
    let mut s = ledger(); s.reserve("a".into(), 100).unwrap();
    s.transition("a", State::Paid, true).unwrap(); s.transition("a", State::Paid, true).unwrap();
    assert_eq!(s.balances().confirmed_paid, 100);
    s.transition("a", State::Refunded, true).unwrap(); s.transition("a", State::Refunded, true).unwrap();
    assert_eq!(s.balances().confirmed_paid, 0); assert_eq!(s.balances().disputed, 100); assert_eq!(s.available(), Ok(0));
    assert_eq!(s.transition("a", State::Paid, true), Err(Error::Transition));
}
#[test] fn balances_cannot_exceed_invoice() {
    let mut s = ledger(); assert_eq!(s.invoice_to(101), Err(Error::Invalid));
    s.dispute(60).unwrap(); assert_eq!(s.reserve("a".into(), 41), Err(Error::Insufficient));
    assert_eq!(s.release_dispute(61), Err(Error::Insufficient)); assert_eq!(s.balances().disputed, 60);
    assert_eq!(s.accrue_to(99), Err(Error::Invalid));
}
#[test] fn deterministic_minimum_and_tie() {
    let offers = [offer("z", 10), offer("b", 9), offer("a", 9)];
    assert_eq!(select(&offers, &workload(Profile::PublicCpu), 3, 10).unwrap().provider, "a");
    let reversed: Vec<_> = offers.into_iter().rev().collect();
    assert_eq!(select(&reversed, &workload(Profile::PublicCpu), 3, 10).unwrap().provider, "a");
}
#[test] fn constraints_reject_expired_wrong_epoch_budget_region_capacity() {
    let w = workload(Profile::PublicCpu);
    for bad in [Offer { expires: 10, ..offer("a", 1) }, Offer { epoch: 2, ..offer("a", 1) }, offer("a", 101), Offer { region: "elsewhere".into(), ..offer("a", 1) }, Offer { capacity: 1, ..offer("a", 1) }] {
        assert_eq!(select(&[bad], &w, 3, 10), Err(Error::NoEligible));
    }
}
#[test] fn confidential_gpu_requires_fresh_gpu_evidence() {
    let w = workload(Profile::ConfidentialGpu);
    let mut o = Offer { profile: Profile::ConfidentialGpu, ..offer("a", 1) };
    assert_eq!(select(&[o.clone()], &w, 3, 10), Err(Error::NoEligible));
    for (profile, issued, expires, epoch, provider) in [(Profile::ConfidentialCpu, 0, 20, 3, "a"), (Profile::ConfidentialGpu, 0, 10, 3, "a"), (Profile::ConfidentialGpu, 11, 20, 3, "a"), (Profile::ConfidentialGpu, 0, 20, 2, "a"), (Profile::ConfidentialGpu, 0, 20, 3, "b")] {
        o.evidence = Some(Evidence { provider: provider.into(), profile, epoch, issued, expires, challenge: w.challenge.clone() });
        assert_eq!(select(&[o.clone()], &w, 3, 10), Err(Error::NoEligible));
    }
    o.evidence = Some(Evidence { provider: "a".into(), profile: Profile::ConfidentialGpu, epoch: 3, issued: 10, expires: 20, challenge: w.challenge.clone() });
    assert!(select(&[o], &w, 3, 10).is_ok());
}
#[test] fn duplicate_provider_is_ambiguous() {
    assert_eq!(select(&[offer("a", 1), offer("a", 2)], &workload(Profile::PublicCpu), 3, 10), Err(Error::Invalid));
}
#[test] fn depletion_has_no_delete_stage() {
    let t = Thresholds { stop: 30, drain: 20, export: 10 };
    for (funds, expected) in [(31, Recommendation::Continue), (30, Recommendation::StopNewAdmission), (20, Recommendation::Drain), (10, Recommendation::Export), (0, Recommendation::Retain)] { assert_eq!(recommend(funds, t), Ok(expected)); }
    assert_eq!(recommend(0, Thresholds { stop: 1, drain: 1, export: 1 }), Err(Error::Invalid));
}

#[test] fn cheaper_wrong_billing_unit_is_ineligible() {
    let w = workload(Profile::PublicCpu);
    for unit in ["synthetic-EUR:10^-6/slot-second", "synthetic-USDC:10^-6/slot-hour"] {
        let wrong = Offer { billing_unit: unit.into(), ..offer("cheap", 1) };
        assert_eq!(select(&[wrong.clone()], &w, 3, 10), Err(Error::NoEligible));
        let offers = [wrong, offer("compatible", 90)];
        assert_eq!(select(&offers, &w, 3, 10).unwrap().provider, "compatible");
    }
}
#[test] fn billing_unit_must_be_nonempty() {
    for unit in ["", " \t"] {
        let mut w = workload(Profile::PublicCpu);
        w.billing_unit = unit.into();
        assert_eq!(select(&[offer("a", 1)], &w, 3, 10), Err(Error::Invalid));
        let o = Offer { billing_unit: unit.into(), ..offer("a", 1) };
        assert_eq!(select(&[o], &workload(Profile::PublicCpu), 3, 10), Err(Error::Invalid));
    }
}
fn bound_offer(profile: Profile, issued: u64, expires: u64) -> Offer {
    let w = workload(profile);
    Offer { profile, expires: u64::MAX, evidence: Some(Evidence {
        provider: "a".into(), profile, epoch: 3, issued, expires, challenge: w.challenge,
    }), ..offer("a", 1) }
}
#[test] fn evidence_over_age_with_distant_expiry_is_ineligible() {
    let w = workload(Profile::ConfidentialGpu);
    assert_eq!(select(&[bound_offer(w.profile, 0, u64::MAX)], &w, 3, 11), Err(Error::NoEligible));
}
#[test] fn evidence_challenge_mismatch_is_ineligible() {
    let w = workload(Profile::ConfidentialGpu);
    for challenge in [vec![4, 3, 2, 1], vec![], vec![0; 4]] {
        let mut o = bound_offer(w.profile, 10, 100);
        o.evidence.as_mut().unwrap().challenge = challenge;
        assert_eq!(select(&[o], &w, 3, 10), Err(Error::NoEligible));
    }
}
#[test] fn sensitive_work_requires_nonzero_challenge_and_age() {
    for profile in [Profile::TrustedCpu, Profile::ConfidentialCpu, Profile::TrustedGpu, Profile::ConfidentialGpu] {
        let mut w = workload(profile);
        for challenge in [vec![], vec![0], vec![0; 32]] {
            w.challenge = challenge;
            assert_eq!(select(&[bound_offer(profile, 10, 100)], &w, 3, 10), Err(Error::Invalid));
        }
        w = workload(profile);
        w.max_evidence_age_seconds = 0;
        assert_eq!(select(&[bound_offer(profile, 10, 100)], &w, 3, 10), Err(Error::Invalid));
    }
}
#[test] fn correct_bound_evidence_accepts_inclusive_age_boundary() {
    for profile in [Profile::TrustedCpu, Profile::ConfidentialCpu, Profile::TrustedGpu, Profile::ConfidentialGpu] {
        let w = workload(profile);
        let offers = [bound_offer(profile, 10, 100)];
        assert_eq!(select(&offers, &w, 3, 20).unwrap().provider, "a");
        assert!(select(&offers, &w, 3, 10).is_ok());
        assert_eq!(select(&offers, &w, 3, 21), Err(Error::NoEligible));
    }
}
#[test] fn evidence_time_checks_reject_future_and_expiry_without_wrapping() {
    let w = workload(Profile::ConfidentialGpu);
    for (issued, expires, now) in [(u64::MAX, u64::MAX, 0), (11, 100, 10), (0, 10, 10), (10, 9, 10)] {
        assert_eq!(select(&[bound_offer(w.profile, issued, expires)], &w, 3, now), Err(Error::NoEligible));
    }
    let offers = [bound_offer(w.profile, u64::MAX - 11, u64::MAX)];
    assert!(select(&offers, &w, 3, u64::MAX - 1).is_ok());
}
#[test] fn public_profiles_do_not_require_evidence() {
    for profile in [Profile::PublicCpu, Profile::PublicGpu] {
        let w = Workload { max_evidence_age_seconds: 0, challenge: vec![], ..workload(profile) };
        let offers = [Offer { profile, ..offer("a", 1) }];
        assert!(select(&offers, &w, 3, 10).is_ok());
    }
}
#[test] fn profile_matching_is_exact_for_every_pair() {
    let profiles = [Profile::PublicCpu, Profile::TrustedCpu, Profile::ConfidentialCpu,
        Profile::PublicGpu, Profile::TrustedGpu, Profile::ConfidentialGpu];
    for requested in profiles {
        for offered in profiles {
            let offers = [bound_offer(offered, 10, 100)];
            let result = select(&offers, &workload(requested), 3, 10);
            assert_eq!(result.is_ok(), requested == offered);
        }
    }
}
#[test] fn terminal_replays_and_identity_reuse_never_reserve_again() {
    for terminal in [State::Paid, State::Failed, State::Refunded] {
        let mut s = ledger();
        s.reserve("a".into(), 100).unwrap();
        if terminal == State::Refunded { s.transition("a", State::Paid, true).unwrap(); }
        s.transition("a", terminal, true).unwrap();
        let before = s.balances();
        s.transition("a", terminal, false).unwrap();
        s.reserve("a".into(), 100).unwrap();
        assert_eq!(s.balances(), before);
        assert_eq!(s.intent("a").unwrap().state, terminal);
        assert_eq!(s.reserve("a".into(), 99), Err(Error::DuplicateMismatch));
        assert_eq!(s.transition("a", State::Pending, true), Err(Error::Transition));
        assert_eq!(s.balances(), before);
    }
}
#[test] fn reordered_receipts_and_clipping_preserve_charge() {
    let mut m = Meter::new(rate(ByteUnit::DecimalGb, 2, 1)).unwrap();
    m.record(interval("later", 10, 20, 1_000_000_000)).unwrap();
    m.record(interval("earlier", 0, 10, 1_000_000_000)).unwrap();
    assert_eq!(m.charge(5, 15), Ok(20));
    let before = m.charge(0, 20);
    m.record(interval("later", 10, 20, 1_000_000_000)).unwrap();
    assert_eq!(m.charge(0, 20), before);
    assert_eq!(m.charge(0, 21), Err(Error::Gap));
    assert_eq!(m.charge(21, 22), Err(Error::Missing));
}

// P03/P10: exhaustive adjacent partitions in a small reference domain, not
// service measurements. The oracle adds reduced fractions one second at a time,
// independent of Meter's interval-duration multiplication and ByteUnit::bytes.
#[test] fn meter_partition_invariance_bounded() {
    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 { let remainder = a % b; a = b; b = remainder; }
        a
    }
    fn rational_spec(bytes: u128, price: u128, unit_bytes: u128, period: u128, ticks: i64) -> u128 {
        let (mut numerator, mut denominator) = (0_u128, 1_u128);
        let tick_numerator = bytes * price;
        let tick_denominator = unit_bytes * period;
        for _ in 0..ticks {
            let shared = gcd(denominator, tick_denominator);
            numerator = numerator * (tick_denominator / shared)
                + tick_numerator * (denominator / shared);
            denominator *= tick_denominator / shared;
            let reduction = gcd(numerator, denominator);
            numerator /= reduction;
            denominator /= reduction;
        }
        numerator / denominator // Only this division floors the rational value.
    }
    for (unit, unit_bytes) in [(ByteUnit::DecimalGb, 1_000_000_000_u128),
        (ByteUnit::BinaryGib, 1_073_741_824_u128)]
    {
        for price in [0, 1, 2, 7] {
            for period in [1, 2, 3, 7] {
                for bytes in [0, 1, unit_bytes / 3, unit_bytes - 1, unit_bytes, unit_bytes + 1, 2 * unit_bytes] {
                    for length in 1_i64..=6 {
                        let expected = rational_spec(bytes, price, unit_bytes, period, length);
                        let mut whole = Meter::new(rate(unit, price, period)).unwrap();
                        whole.record(interval("whole", 0, length, bytes)).unwrap();
                        assert_eq!(whole.charge(0, length), Ok(expected));
                        // Each bit cuts at one internal second: all 2^(length-1)
                        // contiguous partitions, including the uncut window.
                        for cuts in 0_u32..(1_u32 << (length - 1)) {
                            let mut receipts = Vec::new();
                            let mut start = 0;
                            for end in 1..=length {
                                if end == length || cuts & (1_u32 << (end - 1)) != 0 {
                                    receipts.push(interval(&format!("part-{start}"), start, end, bytes));
                                    start = end;
                                }
                            }
                            let mut partitioned = Meter::new(rate(unit, price, period)).unwrap();
                            for receipt in receipts.into_iter().rev() {
                                partitioned.record(receipt).unwrap();
                            }
                            assert_eq!(partitioned.charge(0, length), Ok(expected),
                                "unit={unit:?} price={price} period={period} bytes={bytes} length={length} cuts={cuts}");
                            assert_eq!(partitioned.charge(0, length), whole.charge(0, length));
                        }
                    }
                }
            }
        }
    }
    // Negative control: flooring independent invoices erases two half-minor
    // entitlements. Receipt partition invariance never licenses that operation.
    assert_eq!(rational_spec(1_000_000_000, 1, 1_000_000_000, 2, 2), 1);
    assert_eq!(2 * rational_spec(1_000_000_000, 1, 1_000_000_000, 2, 1), 0);
}

// P01/P03/P08/P10: all 5! input orders retain the declared eligible winner.
#[test] fn provider_selection_permutation_invariance_bounded() {
    fn visit(offers: &mut [Offer], at: usize, work: &Workload) -> usize {
        if at == offers.len() {
            let selected = select(offers, work, 3, 10).unwrap();
            assert_eq!(selected.provider, "eligible-a");
            assert_eq!(selected.billing_unit, work.billing_unit);
            assert!(selected.capacity >= work.capacity);
            return 1;
        }
        let mut count = 0;
        for index in at..offers.len() {
            offers.swap(at, index);
            count += visit(offers, at + 1, work);
            offers.swap(at, index);
        }
        count
    }
    let w = workload(Profile::PublicCpu);
    for wrong_unit in ["synthetic-EUR:10^-6/slot-second", "synthetic-USDC:10^-6/slot-hour"] {
        let wrong = Offer { billing_unit: wrong_unit.into(), ..offer("cheap-unit", 1) };
        let small = Offer { capacity: w.capacity - 1, ..offer("cheap-capacity", 2) };
        assert_eq!(select(&[wrong.clone(), small.clone()], &w, 3, 10), Err(Error::NoEligible));
        let mut offers = [offer("eligible-a", 90), offer("eligible-b", 90),
            offer("eligible-expensive", 95), wrong, small];
        assert_eq!(visit(&mut offers, 0, &w), 120);
    }
}
