# Apocryphⁿ

`apocryph-n` v0.1.0 is an original, dependency-free Rust reference library for
compute commerce and admission. It models metering, settlement, advisory offer
selection and staged depletion recommendations. It performs no wallet, provider,
network, shell or key-release action. The separate commerce journal performs
explicit local file I/O and removes its own lock only on clean close. This public
Apache-2.0 foundation is a bounded local reference, not a production network.

The coordinator reported that exact original source
`72a210049f8254fce5a97191854046d56c5bc43a` passed 14 commerce and 30 retained tests,
offline build and actual CLI fixture/restart/replay on qualified owner Linux ARM64
with Rust/Cargo 1.99.1. This is external qualification of that source, not execution
by this worker or validation of the maintenance CI helpers. The current maintenance
candidate remains IMPLEMENTED_UNVERIFIED / NOT_RUN pending independent review and
a new manual fleet run. Historical NOT_RUN records are preserved with a
[dated qualification note](docs/EXPERIMENTS.md#external-qualification-note-2026-10-22).
`git diff --check` establishes whitespace only; no benchmark claim follows.

Start with [documentation routing](docs/README.md) and the
[canonical agent contract](agent/contract.md). [Architecture](docs/ARCHITECTURE.md)
owns semantics; [the improvement register](registers/IMPROVEMENTS.csv) owns change
status; [fix explanations](docs/FIXES.md) distinguish successor avoidance from
upstream reproduction. [Selected principles](docs/SCIENTIFIC-PRINCIPLES.md)
and their [applicability map](registers/SCIENCE.csv) define normative requirements,
not measured implementation or a second fix-status register. [CI routing](docs/development/CI-RUNNERS.md) owns fleet limits.

Original project by Comrade Cooperative and contributors. Naevum successor architecture and development by Naevum and contributors.

Wholly new original source: copyright 2026 Theo Georgiev (Naevum), Apache-2.0.
See [LICENSE](LICENSE), [COPYRIGHT](COPYRIGHT), [AUTHORS](AUTHORS.md) and
[ORIGINS](ORIGINS.md). No upstream GPL source, proprietary Manna or Metahive code
was copied. No upstream ownership or official endorsement is claimed.

## Public foundation and commercial boundary

The source, tests, synthetic fixtures and public wire documentation in this
repository are the Apache-2.0 foundation. Independent contributors can build and
run offline tests without proprietary packages, credentials or owner infrastructure;
see [CONTRIBUTING](CONTRIBUTING.md). Report sensitive findings through the
conditional private path in [SECURITY](SECURITY.md).

Commercial applications, operator data and credentials remain private and outside
this repository. Manna, Metahive and other research integration candidates confer
no imported code rights, required dependency or operational readiness. Source
presence and roadmap proposals are distinct from measured release support.
The commerce reference is explicitly single-writer/local; offer, acceptance and
payment evidence remain unauthenticated external assertions. No governance or
token entitlement follows from contributing or using this foundation.

## Selected network research design

[NETWORK-THESIS](docs/NETWORK-THESIS.md) specifies the current service-contract
research architecture for an open compute, intelligence and operations network.
It covers typed service envelopes, acceptance/invoice lineage, local control and
federation, grounded economics, OSS/commercial application layers and the future
independent compute-cluster-sim boundary. It is **RESEARCH DESIGN**, not implemented
v0.1 behavior or evidence of a working network. No simulator dependency or adapter
is established. Conventional invoicing is the first comparator; native transferable
issuance remains unselected. [E17](docs/EXPERIMENTS.md#e17-network-policy-and-simulator-family)
proposes comparative experiments, all NOT_RUN. N01–N11 are additional OPEN obligations
in the sole status [register](registers/IMPROVEMENTS.csv); existing gaps remain.

## Commerce increment

[Typed commerce and exact offline wire contract](docs/COMMERCE.md) specifies the
new reservation/invoice/credit/outbox library and bounded durable journal. The
synthetic TSV fixture supplies a generic external-simulator composition boundary;
no simulator or payment service is imported. Permanent accounting credits and a
local journal are implemented source, while authentication, distributed finality,
legal validity and production readiness remain open. C01–C08 own this bounded
increment's historical IMPLEMENTED_UNVERIFIED status. E18 retains its original
NOT_RUN record and adds the scoped external qualification note; broader claims
and the new maintenance helpers remain unverified.

## Reference API

- `metering::{Rate, ByteUnit, Interval, Meter}`: checked byte-second charges with
  complete coverage, replay handling and one final floor to token minor units.
- `settlement::Settlement`: cumulative accrual/invoicing, reservation identities,
  reconciled state assertions and dispute quarantine. Existing identity success
  never authorizes a fresh transfer; permanent refund/credit-note accounting is open.
- `admission::{Offer, Workload, Evidence, Profile, select}`: advisory minimum price
  within an exact billing unit and exact profile/region/epoch constraints. Sensitive
  profiles require bounded evidence age and matching session challenge.
- `policy::{Thresholds, recommend}`: staged depletion recommendations ending in
  Retain. No actual export, durable retention or deletion is performed.

```rust
use apocryph_n::admission::{select, Evidence, Offer, Profile, Workload};

let work = Workload {
    region: "test-region".into(), capacity: 2, budget_minor: 100,
    billing_unit: "synthetic-USDC:10^-6/slot-second".into(),
    profile: Profile::ConfidentialGpu, max_evidence_age_seconds: 10,
    challenge: vec![1, 2, 3, 4], // Synthetic fixture, not a production nonce.
};
let offers = [Offer {
    provider: "example".into(), region: work.region.clone(), capacity: 4,
    price_minor: 90, billing_unit: work.billing_unit.clone(),
    profile: work.profile, epoch: 3, expires: 100,
    evidence: Some(Evidence {
        provider: "example".into(), profile: work.profile, epoch: 3,
        issued: 10, expires: 100, challenge: work.challenge.clone(),
    }),
}];
let selected = select(&offers, &work, 3, 20).unwrap();
assert_eq!(selected.provider, "example");
```

This example is NOT_RUN. Evidence fields are unverified caller assertions; a
trusted-profile record is no TEE claim. Billing units are explicit opaque labels,
not signed prices. Production marketplace integration, signed offers, hardware
verification, independent key release, durable recovery and funded sites remain
open in the [roadmap](docs/ROADMAP.md) and authoritative register. Manna is separate
optional automation; Metahive is separate research/backend work. Neither is imported.
The manual CI route and exact-source qualification limits are documented in
[CI routing](docs/development/CI-RUNNERS.md). This worker performed no remote or
fleet enrollment. See [experiments](docs/EXPERIMENTS.md) for controls and evidence.

Bare amounts do not authenticate asset identity or couple Rate, Workload and
Settlement into one durable invoice protocol. The legacy APIs remain independent. The new commerce module links its own typed
reservation and cumulative accepted-service counter to local invoice/credit/outbox
records; authenticated metering and external finality remain absent.
