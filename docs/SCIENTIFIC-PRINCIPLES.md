# Apocryphⁿ scientific principles

Campaign scope note (2026-10-21): [COMMERCE](COMMERCE.md) now owns the separate
bounded typed-commerce API, local journal, permanent accounting credits and offline
wire contract (C01–C08, IMPLEMENTED_UNVERIFIED). Descriptions below of absent typed
lineage, credit or durable-log mechanisms describe the preserved legacy baseline;
external authentication, legal finality and distributed recovery remain absent.
The original NOT_RUN record is historical; see the [2026-10-22 external
qualification note](EXPERIMENTS.md#external-qualification-note-2026-10-22) for exact-source
evidence. This maintenance candidate remains NOT_RUN; broader claims are unchanged.

These selected engineering requirements derive from the owner-supplied design
reference for this task. Apocryphⁿ selects an auditable deterministic kernel below
probabilistic proposals and separately authorized effect adapters. Adoption is
normative: it is not measured implementation, formal proof, scientific novelty or
permission to operate. The current dependency-free, in-memory Rust reference has
no effect adapters. References below retain the supplied source rationale; they
were not independently fetched or reviewed during this work.

[SCIENCE.csv](../registers/SCIENCE.csv) maps every principle to actual APIs,
witnesses, assumptions and gaps. [IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv)
alone owns fix status. [Experiments](EXPERIMENTS.md) preserve historical NOT_RUN
records and the scoped coordinator observation. [CI routing](development/CI-RUNNERS.md)
distinguishes measured original-source support from pending maintenance validation.

## P01 — Specify transitions and replay boundaries

Select `F_v(state, validated_event) -> (new_state, decision, intents, trace)`,
where `v` fixes schema, policy and arithmetic. Identical validated inputs and
versions must yield identical outputs. Capture external observations, proposals,
time and randomness as inputs; never regenerate them on authoritative replay.
Explicit event ordering follows [Lamport](https://lamport.azurewebsites.net/pubs/time-clocks.pdf);
specification/refinement follows [TLA+](https://lamport.azurewebsites.net/tla/book.html).
Determinism alone neither makes concurrent effects commute nor supplies consensus.

Select versioned envelopes and canonical bytes before signatures, using
[RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html) as the JSON baseline with
schema-defined decimal strings for large monetary integers. Validation, canonical
encoding, hashing and authentication are separate checks. Current APIs implement
local receipt replay, terminal no-ops and stable provider ties, not this complete
transition envelope or canonical log. Falsifiers include input permutation changing
a decision, changed versions accepted as equivalent, duplicate keys, ambiguous
numbers and restart divergence. There is no model-checked implementation.

## P02 — Separate proposal, permission and execution

The AI proposes; an independently specified kernel evaluates eligibility; a bounded
adapter may execute only an exactly authorized intent. Missing, unknown, stale or
false required conditions deny permission, regardless of predicted profit.
[Shielded learning](https://arxiv.org/abs/1708.08611) and
[Cedar authorization](https://docs.cedarpolicy.com/auth/authorization.html) motivate
this separation. Their conclusions depend on the safety model and policy; they
do not establish arbitrary tool safety or business-law compliance.

Evaluator changes, trust roots, jurisdiction rules, reserved human decisions and
spending limits require separate governance. Self-maintenance cannot expand its
own evaluator or authority. `select` checks declarations and `recommend` returns
advice; neither is an authenticated authorization gate. Falsifiers include planner
bypass, forged approval, stale epochs and a repair granting itself permissions.
Complete mediation and authenticated effect enforcement remain absent.

## P03 — Use dimensions and conservation as executable contracts

Select asset identity and scale for amounts, plus service quantum and time basis
for prices; validate dimensions before comparing or accumulating values.
[Kennedy's units-of-measure work](https://www.microsoft.com/en-us/research/publication/relational-parametricity-and-units-of-measure/)
motivates this discipline without proving these Rust interfaces dimensionally safe.
GB, GiB and billing periods differ. Selected invariants are
`sum(allocated) + residue = input` where allocation exists, and
`confirmed_paid + reserved + disputed <= invoiced <= accrued` for settlement.

For complete cumulative coverage select
`floor(sum(bytes_i * seconds_i * price_minor) / (bytes_per_unit * period_seconds))`.
Floor once per cumulative entitlement and invoice its delta; flooring individual
fragments loses value. Missing coverage is unknown, never zero. `Meter::charge`
uses bounded checked arithmetic; `select` requires exact opaque billing labels.
Bare amounts do not authenticate asset identity or couple `Rate`, `Workload` and
`Settlement` into one durable invoice protocol. A complete meter-to-invoice,
credit-note and finality bridge is absent. Current APIs implement bounded
arithmetic and caller-assertion validation only. Falsifiers include wrong-currency
cheap offers, GB/GiB confusion, partition loss, overlap, overflow and double
counting. Nominal cross-ledger types and authenticated telemetry remain future work.

## P04 — Declare fairness axioms and tradeoffs

Allocation and voting are institutional choices, not universal fairness. Specify
denominators, abstentions, exclusions, protected-class rights, caps, rounding and
residue ownership before selecting a method.
[Balinski and Young](https://pmc.ncbi.nlm.nih.gov/articles/PMC433936/) motivate
explicit axioms; apportionment research does not validate a corporate charter.

The supplied Hamilton snapshot counterexample uses weights `(5,3,1)`, no binding
caps and ascending stable IDs: total 4 allocates `(2,1,1)`, while total 5 allocates
`(3,2,0)`. The third recipient loses a unit. Conservation and deterministic ties
therefore do not establish cumulative monotonicity. Retain this falsifier as a
known limitation, and separately review a monotone policy if required.
Apocryphⁿ has no Hamilton allocator or voting API; lexical provider ties are only a
declared selection rule, not a fairness guarantee. No legal charter is implemented.

## P05 — Treat uncertainty as reconciliation state

Specify intent transitions and terminal outcomes before adapters. Select atomic
durable recording of intent and stable business identity before submission, with
uniqueness surviving authority rotation. Timeout means UNKNOWN; require outcome
evidence before resolution or another attempt.
[Gray and Lamport](https://arxiv.org/abs/cs/0408036) inform failure-aware coordination,
not atomicity between a local ledger and arbitrary external services.

`Settlement` retains Unknown reservations and makes identity reuse and repeated
terminal requests no-ops. A successful repeated lookup never authorizes another
payment. Its reconciliation boolean is unverified; all state is in memory.
Select adapter-specific idempotency, fencing and recovery, with no universal
exactly-once claim. Falsifiers include crashes at persistence/send/ack boundaries,
duplicate callbacks, simultaneous reservations, stale writers, provider rollback
and chain reorganization. Durability, finality and distributed concurrency remain
unimplemented; Refunded is dispute quarantine, not a legal credit note.

## P06 — Appraise evidence against explicit trust

Following [IETF RATS](https://www.rfc-editor.org/rfc/rfc9334.html), separate raw
assertions, authenticated evidence, verifier results and authorization decisions.
Select trusted issuers, reference values, appraisal policy and freshness before
reliance. Bind provider, workload, tenant, profile, epoch/version and challenge
as applicable. Key release requires a separately qualified relying-party decision.

`Evidence` fields remain caller assertions. Exact profile/provider/epoch/challenge
checks, positive maximum age and a nonzero challenge reject certain inconsistent
declarations; they verify neither signatures nor workload/tenant identity.
CPU evidence cannot satisfy GPU requirements. Even cryptographic validity would
not establish application correctness or physical security. Falsifiers include
future/expired evidence, excessive age despite distant expiry, wrong challenges,
revoked issuers, firmware downgrade and cross-tenant replay. Hardware appraisal,
revocation, nonce uniqueness and independent key release remain absent.

## P07 — Abstain and measure the cost

Select explicit HOLD/escalation rather than forced action.
[Selective classification](https://arxiv.org/abs/1705.08500) motivates measuring
risk against coverage: incorrect accepted actions, useful work deferred, human
intervention and delay. Self-reported confidence is neither calibrated evidence
nor authority. `NoEligible`, `Invalid` and depletion recommendations are bounded
reference outcomes, not a calibrated abstention system.

[Conformal prediction](https://arxiv.org/abs/2107.07511) is an experimental option
only with defined outcomes, calibration data and defensible assumptions.
Marginal guarantees are not per-case correctness, subgroup guarantees or validity
under distribution shift. Falsifiers include changed providers or jurisdictions,
correlated repeated tasks and rare severe failures. Calibration, escalation
protocols and deployment thresholds are absent; adopt no risk number without
task-specific evidence.

## P08 — Admit feasible providers before economics

Require identity, region, capacity, billing unit, exact security profile, fresh
evidence where required and budget before optimizing a declared objective.
[Borg](https://research.google/pubs/large-scale-cluster-management-at-google-with-borg/)
informs constraint-aware design; [The Tail at Scale](https://research.google/pubs/the-tail-at-scale/)
motivates latency distributions and failure analysis.

`select` minimizes declared price among eligible offers with lexical provider-ID
ties. Identity is a supplied label and selection reserves no capacity. This is
neither global optimization nor a measured scheduler. Compare useful completed-work
cost and SLO attainment against eligible cheapest-price, round-robin and suitable
locality-aware baselines. Include queueing, retries, transfer, energy, idle
capacity, support and settlement; report tail latency, failures and margin.
Falsifiers include cheaper wrong-unit or undersized offers winning, burst load,
correlated site failure and recovery/egress costs erasing margin. No marketplace,
performance, energy or monetization advantage is measured.

## P09 — Preserve provenance without equating it with truth

Select entity/activity/agent distinctions from
[W3C PROV](https://www.w3.org/TR/prov-overview/) to connect decisions to observations,
transformations, versions, actors and approval. Hashes and append-only records
establish neither accuracy nor entitlement. Record origin, time, asserted scope,
authentication, conflicts, retention and correction status. Jurisdiction and
IP-title claims require qualified evidence; a token does not establish title.

Treat partner documents, marketplace metadata and tool output as untrusted data.
[AgentDojo](https://arxiv.org/abs/2406.13352) motivates adversarial instruction tests
alongside benign utility. Falsifiers include injected invoices, poisoned sources,
omitted contrary evidence and authentic but legally ineffective instruments.
The upstream findings here came from coordinator static source review, not an
owner-performed audit or approval. Actual owner instructions remain separate.
Documentation provides provenance context; a full provenance graph, disclosure
controls and legal-pack lifecycle remain absent.

## P10 — Require independent attempts to falsify claims

Adopt [verification-guided development](https://arxiv.org/abs/2407.01688): executable
reference models, explicit properties, independent model/implementation comparisons
and property-based testing. Proofs concern their models and assumptions; finite
tests concern their sampled domain. Neither proves a deployed business system.

Before comparisons, specify hypothesis, strong baseline, task distribution,
held-out cases, sample plan, practical effect size, uncertainty analysis and
failure criteria. [Preregistration](https://doi.org/10.1073/pnas.1708274114)
motivates separating planned tests from exploration. Publish negative results and
plan changes. Each change must link principles, invariant, assumptions, positive
and negative witnesses, source identity, execution environment and an independent
scoped result. Static baseline review is not reproduced evidence.

`meter_partition_invariance_bounded` and
`provider_selection_permutation_invariance_bounded` add finite reference-domain
witnesses, not end-to-end performance evidence. Tests written but unexecuted stay
NOT_RUN. Release requires qualified source-bound receipts and protocol evidence
for each claimed capability; adopting this document satisfies no product acceptance
gate by itself.
