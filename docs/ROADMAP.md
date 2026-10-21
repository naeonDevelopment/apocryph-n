# Apocryphⁿ integration roadmap

Campaign scope note (2026-10-21): [COMMERCE](COMMERCE.md) now owns the separate
bounded typed-commerce API, local journal, permanent accounting credits and offline
wire contract (C01–C08, IMPLEMENTED_UNVERIFIED). Descriptions below of absent typed
lineage, credit or durable-log mechanisms describe the preserved legacy baseline;
external authentication, legal finality and distributed recovery remain absent.
The original NOT_RUN records are preserved; the [dated external qualification
note](EXPERIMENTS.md#external-qualification-note-2026-10-22) scopes subsequent evidence.
This note changes no principle or broader research claim.

This is the completion boundary for the public v0.1.0 foundation, not production
acceptance. [IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) alone owns change
status. The new maintenance candidate remains NOT_RUN. No next-stage spending, disclosure, deployment or external effect is
authorized by this roadmap.

1. **Qualify the maintenance candidate.** Preserve the coordinator-observed
   qualification of the exact original commerce source. Independently review the
   corrected helpers/workflow, then obtain a new manual qualified-fleet receipt
   for the maintenance commit, including all 30 legacy and 14 commerce witnesses
   and CLI replay. ARM64 qualification does not establish amd64 support; keep the
   default route and no-fallback policy. Add the independent numerical oracle
   and boundary traces from E10 after the base fixtures can be measured.
2. **Design durable settlement and ingestion.** Prioritize stable business IDs,
   typed contract/attempt/acceptance/invoice lineage, transactional exposure
   reservation and outbox, then crash/replay/concurrency tests (N02/N03, E17a/b).
   Define one authenticated billing
   context across Rate, Offer, Workload and Settlement; the current modules are
   separate. Bind receipts to tenants/resources and preserve IDs across restart.
   Specify durable journaling, unknown-result reconciliation and finality/reorg
   policy before any wallet/provider adapter. E11 must show crash-safe conservation
   and no fresh submission from identity-reuse success. B04 requires explicit
   permanent credit/refund semantics and qualified legal accounting review.
3. **Qualify offers and execution.** B03/A10 require signed quote terms, provider
   identity, capacity reservation, cancellation/dispute policy and measured service
   integration. A09/F02/F03 are local assertion checks only. B09 requires a hardware verifier that
   pins the exact CPU/GPU/software configuration, appraises signed evidence,
   binds workload and challenge, handles revocation and meets E12 negative controls.
   B06 requires an independent key-release service and its own threat analysis.
4. **Prove recovery and operations.** A06/A08/B07 require scoped replica authority,
   independently verified completion and restoration across key/site loss. B08
   requires explicitly funded and qualified sites with independent failure domains.
   E13 establishes RPO/RTO and key recovery limits; Retain alone establishes neither.
5. **Measure commercial viability.** B05 and E14 require comparable service SLOs,
   full provider costs, utilization and outage sensitivities, reconciliation and
   support burden. Define accountable operational owners and contracts before any
   production marketplace claim. No revenue, benchmark or partner evidence exists.

Production marketplace, signed offers, hardware verifier, independent key release,
durable recovery and funded sites remain OPEN in the register. A06 replica
authorization is outside this in-memory v0.1 scope and requires a separately scoped
integration. The library remains dependency-free and performs no I/O; new service
adapters must not silently turn pure reference calls into external actions.

Manna is separate optional automation, with its own language, gates and qualification.
Metahive is separate research/backend work. Potential future interfaces may exchange
explicitly approved public schemas, but this stage imports no private code, legal
or company facts, credentials or execution authority from either. No `.manna`
program or private integration is part of this deliverable.


## Network research adoption sequence

[NETWORK-THESIS](NETWORK-THESIS.md) is the selected current RESEARCH DESIGN;
[ARCHITECTURE](ARCHITECTURE.md) remains the v0.1 semantic reference. This order
implements none of the roadmap. B03–B09 remain OPEN and their responsibilities
are extended, not replaced, by distinct N01–N11 obligations explained in
[FIXES](FIXES.md#network-design-audit).

1. Freeze service-envelope semantics, canonical typed identities and acceptance
   lineage (N01/N02/N05) alongside B03 signed contracts and B04 permanent credit
   rules. Keep advice, authenticated reservation, attempt, accepted usage, invoice,
   collection intent, uncertain/reconciled payment and credit distinct. Model
   arbitrary amount labels and ambiguous outcome handling as missing evidence.
2. Specify and then qualify atomic resource/exposure reservation and durable outbox
   with stable IDs, fencing and bounded authority (N03). E11/E17b crash/replay and
   concurrent-writer evidence precede any provider or payment effect adapter.
   A local ledger supplies no exactly-once external transfer guarantee. B07 durable
   recovery and B04 permanent credits remain independent requirements.
3. Establish open contracts/SDK/provider conformance and site-local control with a
   slower regional federation (N04/N10); qualify failure domains by operator,
   administrative key and verifier control, not node counts. Test stale summaries,
   partitions and owner recall. No blockchain enters the per-token or five-second
   physical loop. B08 still requires qualified funded operations.
4. Agree versioned Scenario/Decision/Attempt/AcceptedUsage/EventTrace/RunManifest
   boundaries with the separately prepared general-purpose compute-cluster-sim
   project (N09). Its separately reported source qualification does not qualify
   integration here; no runtime dependency or adapter exists. Future pure policy integration must pass causality, bounds,
   conservation, replay and censoring conformance before calibrated E17 comparisons
   (N11). Specialist backends are optional future integrations, not timing models
   to rebuild here. No simulator or sibling source work is authorized by this order.
5. Qualify public batch/embeddings/evaluation first, maintained endpoints and dedicated
   nodes with turnover costs next (N05/N08/N10). Confidential services require exact
   hardware/software qualification under B09/B06. Expert/DAG routing, checkpointed
   training and specialized compute are later research; correctness, semantic quality,
   confidentiality, availability and commercial viability have separate gates.
6. Compare conventional invoicing and optional supported payment rails with full-cost
   service delivery, separate money/credit/collateral/contributor records (N06),
   subsidy-free margin, price erosion, exit and coalition tests (N07; B05/E14/E17d/f).
   No issuance, supply cap, yield, fixed ROI or sale is selected. Measure onboarding,
   export/self-host portability, maintained releases and repeat customer demand
   before adoption claims. OSS core contracts/SDK/protocol/simulator/conformance
   and commercial managed operations/support/calibration/custody/integrations
   preserve source licences and separate authority.

Manna remains optional and separately qualified; the network and simulator must be
independently useful. Numerical E17 thresholds are proposed preregistration targets,
not observed gains or production promises. No pending TODO is marked solved by
this research adoption. Acceptance may hold on restored judge surfaces relative to
empty bootstrap main; preserving those surfaces takes precedence over weakening
contracts, checkers or workflows to obtain acceptance.
