# Deterministic v0.1 semantics

Campaign scope note (2026-10-21): [COMMERCE](COMMERCE.md) now owns the separate
bounded typed-commerce API, local journal, permanent accounting credits and offline
wire contract (C01–C08, IMPLEMENTED_UNVERIFIED). Descriptions below of absent typed
lineage, credit or durable-log mechanisms describe the preserved legacy baseline;
external authentication, legal finality and distributed recovery remain absent.
The original NOT_RUN record is historical; see the [2026-10-22 external
qualification note](EXPERIMENTS.md#external-qualification-note-2026-10-22) for exact-source
evidence. This maintenance candidate remains NOT_RUN; broader claims are unchanged.

Pure Rust 2021, standard library only, `#![forbid(unsafe_code)]`. No persistence,
cryptographic verification, service runtime or business-effect authority. Public
input fields are untrusted declarations. All monetary arithmetic uses checked
u128; errors return before mutation. Error variants are Invalid, Overflow,
DuplicateMismatch, Overlap, Gap, Missing, Insufficient, Transition and NoEligible.

## Metering

`Rate` declares token label, minor_units_per_token, price_minor, ByteUnit and
period_seconds. DecimalGb = 1,000,000,000 bytes; BinaryGib = 1,073,741,824 bytes.
The minor-unit factor is metadata: price and output are already minor units.
A 30-day month is explicitly 2,592,000 seconds. This API uses byte-seconds;
byte-minutes are unsupported and must be normalized before ingestion.

`Interval` is stable string ID, nonnegative i64 start/end seconds, u128 bytes
constant on the half-open interval [start,end). Empty IDs, zero/negative durations
and negative timestamps are invalid. Zero bytes and zero price are legitimate
known measurements. Identical replay is idempotent; altered replay is an error;
different IDs with overlapping time are rejected. Each Meter represents one
resource and one immutable rate. Independent simultaneous resources need separate
meters or prior explicit aggregation.

`charge(start,end)` clips intervals to the requested window and requires complete
contiguous coverage. No intervals yields Missing; missing prefix/interior/suffix
yields Gap even at zero price. It sums `bytes * seconds * price_minor` before one
floor division by `byte_unit.bytes() * period_seconds`. Intermediate overflow
returns Overflow, even when an algebraically reduced result might fit. Fractional
minor units persist only through querying a cumulative billing window: callers
must use a fixed billing origin and feed cumulative charge to `accrue_to`, not sum
independently rounded window charges. This is an explicit caller obligation.

The regression witness is 1 decimal GB for 30 days at 2500 micro-USDC/GB-month:
2500 minor units, not 2,500,000. One GiB priced per GiB has the same 2500 result;
GiB priced per GB has a different result. No floating-point arithmetic is used.

## Settlement

`Settlement::new(app,version)` preserves both nonempty strings. `Balances` is a
copy-only snapshot with cumulative accrued and invoiced, reserved, net
confirmed_paid and disputed. `accrue_to` and `invoice_to` are monotonic absolute
updates; invoiced <= accrued. Available = invoiced - paid - reserved - disputed.
All accepted operations maintain nonnegative available and the corresponding
sum <= invoiced. Accrual is token minor units, not a payment or receipt.

`reserve(id,amount)` accepts a positive amount only within available funds.
An ID is permanent: same ID/amount is a no-op even after termination; changed
amount is DuplicateMismatch. Sequential exclusive mutable access prevents
concurrent duplicate collection inside one ledger. This is not distributed locking.
Successful identity reuse is never permission to submit a fresh transfer. A failed
intent stays terminal; any deliberate retry would need a new identity and separately
authorized external reconciliation policy, which this library does not implement.

Transitions: Pending -> Unknown on timeout keeps reservation. Pending or Unknown
-> Paid/Failed requires `reconciled=true`; Paid adds confirmed-paid once and
releases reservation, Failed only releases reservation. Paid -> Refunded requires
reconciliation and moves paid value into disputed, preventing automatic recollection.
Repeated same-state transition is idempotent; other transitions fail. The boolean
is a caller assertion, never proof of finality. A transaction hash is not a receipt.
Disputes can only claim currently available amounts; explicit release makes the
amount collectible again. Partial refunds, paid-amount dispute adjudication,
currency conversion and invoice cancellation are unsupported. A Refunded state
quarantines the amount in disputed; release_dispute makes it collectible again.
It does not reduce invoiced, issue a legal credit note or implement a permanent
refund. Legal accounting is tracked separately in the improvement register.
Each ledger must be scoped by the caller to one currency and billing context; the
ledger has no currency field or connection to Meter/Offer. This is an integration
obligation, not end-to-end dimensional enforcement.

## Admission

`Offer` declares provider ID, region, capacity (u64 abstract slots), price_minor
(u128 declared price for one service quantum), nonempty billing_unit, Profile,
epoch, expiry and optional Evidence. `Workload` declares region, positive required
slots, budget_minor, billing_unit, Profile, max_evidence_age_seconds and challenge.
A billing_unit must encode currency/token identity, minor-unit scale and service
quantum, for example `synthetic-USDC:10^-6/slot-second`. The identifier is opaque:
exact string equality is required before comparing price with budget or selecting
a minimum. Whitespace-only units invalidate the request/set. No aliases, exchange,
normalization or dimensional conversion occur. Equal labels do not prove equal
real-world service; capacity does not multiply the quoted price automatically.

Profiles are PublicCpu, TrustedCpu, ConfidentialCpu, PublicGpu, TrustedGpu and
ConfidentialGpu. Exact matching is deliberate; no implicit security ranking exists.
`select(offers,work,epoch,now)` requires exact region/profile/epoch and billing_unit,
sufficient capacity, price <= budget and now < offer.expires. Empty provider/region
or duplicate provider IDs invalidate the set, even if that offer would be ineligible.
Eligible offers are ordered by price then lexical provider ID. No candidate yields
NoEligible. Invalid workload configuration yields Invalid before selection.

All four nonpublic profiles are sensitive. They require a positive
max_evidence_age_seconds and a nonempty challenge with at least one nonzero byte.
Evidence must exactly match provider/profile/epoch/challenge, be unexpired, and
satisfy `now.checked_sub(issued) <= max_evidence_age_seconds`. Checked subtraction
rejects future issue times; the inclusive age boundary is accepted, expiry is
exclusive. Empty/all-zero required challenges or zero maximum age yield Invalid;
bad evidence makes that offer ineligible. Public profiles need no evidence and
ignore those evidence requirements. CPU evidence never satisfies a GPU profile.

Epoch is a caller-selected u64 generation. All timestamps and ages use u64 seconds
in the same caller-selected clock domain. The caller supplies trusted time and a
fresh session challenge; the library neither generates entropy nor tracks challenge
reuse or authenticates these values. Evidence remains **unverified caller
assertions**, even after all checks. A trusted-profile reference record creates no
TEE claim. There is no quote parser, signature verification, hardware appraisal,
revocation check, runtime measurement or binding to workload code/keys. Selection
returns a borrowed advisory offer; it creates no reservation or capacity decrement.

A complete construction example is in README; all constructors in
`tests/regression.rs` supply the new fields. The API is part of the public local v0.1
foundation. The dated qualification note above scopes the coordinator-observed
original-source tests; it does not validate the new maintenance helpers.

## Depletion

`recommend(remaining_minor, Thresholds)` requires stop > drain > export > 0.
Above stop: Continue; at/below stop: StopNewAdmission; at/below drain: Drain;
at/below export: Export; zero: Retain. Each result is a cumulative stage recommendation:
later stages imply stopping new admissions and retaining data. No deletion,
fund transfer, export success or key release occurs. Recovery durability and replica
completion are future work, not established by retaining a recommendation enum.

## Selected principles and evidence boundary

[P01–P10](SCIENTIFIC-PRINCIPLES.md) are normative engineering requirements;
[SCIENCE.csv](../registers/SCIENCE.csv) maps their actual partial applicability.
P01 covers replay and ordering; P02 advice versus authority; P03 metering and
balance invariants; P05 reconciliation; P06 assertion binding; P08 eligibility
before price. P04 allocation/voting is absent, P07 calibrated escalation is absent,
and P09 provenance remains documentation. P10 requires scoped independent evidence.

The P03/P10 `meter_partition_invariance_bounded` witness compares complete-window
charges with every adjacent partition of windows 1–6 seconds long and a separate
per-second reduced-rational specification, with one final floor. P01/P03/P08/P10
`provider_selection_permutation_invariance_bounded` checks all 120 permutations
per fixture, including cheaper wrong-unit and insufficient-capacity offers.
Both are reference-domain properties, NOT_RUN, not end-to-end measurements.

Bare amounts authenticate no asset identity. Rate, Workload and Settlement are
not coupled into one durable invoice protocol; the complete meter-to-invoice,
credit-note and finality bridge remains absent. No nominal cross-ledger currency,
canonical encoding, actual attestation or durable settlement is implied.
