# Apocryphⁿ fixes and regression reasoning

[IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) is the sole change-status
authority. This document explains witnesses and limits, not observed outcomes.
All product validation is NOT_RUN. The 17 original regression cases are preserved;
11 additional cases cover new admission constraints, terminal replay, reordered
receipts and clipping; two bounded properties bring the preserved total to 30. Test names below identify source in `tests/regression.rs`.
No upstream bug was reproduced and this independent Rust library patches no
upstream Go code or deployed service.

The coordinator supplied static source-review findings for
`comrade-coop/apocryph@e622a1074261905d8ace3b65571d93a6c036ffc4`.
The old Go code was not run. A01–A10 are conditional source-review findings,
not findings measured against a present service. The pinned
[billing source](https://github.com/comrade-coop/apocryph/blob/e622a1074261905d8ace3b65571d93a6c036ffc4/backend/minio-manager/payment.go)
is provenance for the billing review, not a claim that every finding resides there.

| Finding | Local reasoning and regression witness | Boundary |
| --- | --- | --- |
| A01 ledger versus direct ERC20 collection | `Settlement::reserve` excludes already reserved value; `reserve_excludes_second_collector` | No ERC20 path, wallet, distributed lock or actual transfer |
| A02 dropped constructor version | `Settlement::new` retains app/version; `constructor_keeps_version` | No deployment/version migration semantics |
| A03 GB/GiB conversion | `ByteUnit`, explicit period and checked integer charge; `decimal_month_regression`, `gib_is_not_gb` | Caller must choose correct units and cumulative window |
| A04 missing sample coverage | `Meter::charge` distinguishes Missing/Gap from measured zero; `unknown_coverage_never_becomes_zero` | No collection service or proof that samples are true |
| A05 pending/finality reconciliation | Unknown retains reservation; `timeout_blocks_until_reconciled`, `success_and_refund_are_once_only` | Caller assertion is no finality proof; crash/reorg recovery absent |
| A06 broad replica authorization | No replica authorization implementation or regression | Removing I/O surface is not implementing least privilege |
| A07 forced deletion at depletion | `recommend` ends in Retain; `depletion_has_no_delete_stage` | No guarantee another system retains data |
| A08 incomplete replication/restore indication | No replication or restore protocol or regression | Retain is not a restore receipt |
| A09 unqualified full GPU TEE profile | Exact profile/evidence fields; `confidential_gpu_requires_fresh_gpu_evidence`, `profile_matching_is_exact_for_every_pair` | Assertions only; neither trusted nor confidential labels qualify hardware |
| A10 full marketplace/business integration unmeasured | No marketplace integration regression | Local arithmetic cannot establish product readiness or revenue |

## Coordinator review additions

F01 makes `Offer.billing_unit` and `Workload.billing_unit` explicit nonempty
identifiers including token/currency scale and service quantum. Equality precedes
numeric price comparison. `cheaper_wrong_billing_unit_is_ineligible` presents a
1-unit wrong-currency or wrong-quantum offer against a compatible 90-unit offer;
the compatible one is expected, and wrong-only yields NoEligible.
`billing_unit_must_be_nonempty` covers empty and whitespace-only labels. Equal
opaque labels are not authenticated contract terms; ledger currency scoping is
still a caller obligation.

F02 adds `Workload.max_evidence_age_seconds`: positive for sensitive profiles,
checked elapsed subtraction, inclusive age bound, exclusive expiry.
`evidence_over_age_with_distant_expiry_is_ineligible` rejects age 11 at maximum 10
even with u64::MAX expiry. `evidence_time_checks_reject_future_and_expiry_without_wrapping`
includes future time, expiry, inverted dates and a near-u64-limit positive control.

F03 binds `Evidence.challenge` to `Workload.challenge`. Empty or all-zero required
challenges are invalid; exact mismatches are ineligible.
`evidence_challenge_mismatch_is_ineligible` and
`sensitive_work_requires_nonzero_challenge_and_age` are negative controls.
`correct_bound_evidence_accepts_inclusive_age_boundary` is the positive control for
all four sensitive profiles; `public_profiles_do_not_require_evidence` preserves
public behavior. A challenge match does not verify its origin, entropy, freshness
across calls or a signature. Trusted-profile records do not create TEE claims.

F04 preserves repeated terminal-state idempotence with or without a new
reconciliation flag. `terminal_replays_and_identity_reuse_never_reserve_again`
covers Paid, Failed and Refunded. Same-ID reserve success is acknowledgment of
existing state, never permission to submit a new transfer. Paid -> Refunded
moves value to disputed; release can make it collectible again. Permanent refunds,
credit notes and invoice cancellation require a separate legal accounting design.

## Business rules and absent capabilities

B01 tracks dispute quarantine and explicit release (`balances_cannot_exceed_invoice`).
B02 tracks deterministic comparison (`deterministic_minimum_and_tie`) without
claiming optimal scheduling or genuine provider economics. B03–B09 track signed
offers/marketplace contracts, permanent refund accounting, provider economics,
independent key release, durable recovery, funded sites and hardware verification. Their missing tests
and modules are explicitly marked NONE in the register rather than invented.
These enhancements need business owners, integration criteria and qualified
measurements; no legal/company facts, partners or commercial commitments are inferred.

## Provenance and verification

This completion restored 21 files from the owner's exact WIP snapshot
`58af7e00b6c846f4c76549378d14299c5e13e336` before edits. The earlier worker exhausted
its time budget; its REJECTED receipt remains historical evidence. This bounded
stage is not a retry of an uncertain external effect and upgrades no prior receipt.
The official Apache-2.0 text was materialized from own-repository blob
`d645695673349e3947e8e5ae42332d0ac3164cd7`, with SHA-256
`cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`.
New source attribution remains Theo Georgiev (Naevum), with Comrade Cooperative
and contributors credited for original project ancestry. No upstream code copying,
GPL relicensing, endorsement, product execution or external effect is claimed.

## Scientific adoption witnesses (NOT_RUN)

A03 additionally links `meter_partition_invariance_bounded` (P03/P10): full windows
and all adjacent short-window receipt partitions must match a per-second exact
rational specification. F01/B02 additionally link
`provider_selection_permutation_invariance_bounded` (P01/P03/P08/P10): every
permutation retains the same eligible winner despite cheaper wrong-unit and
insufficient-capacity offers. Positive and negative controls and bounded domains
are in [E15/E16](EXPERIMENTS.md). Existing fix statuses are unchanged.
[SCIENCE.csv](../registers/SCIENCE.csv) is an applicability map only.

The scientific-adoption worktree restored exact completed own-repository source
commit `c246f6d7b93217de630499bb2f14c4f42b378410` using read-only git archive.
The earlier WIP restoration above remains historical provenance, not the source
selected for this work order. Upstream findings are coordinator static source
review, not owner audit or approval. Actual owner instructions authorized this
bounded adoption. The modifier-N typography in the original order was a coordinator
interpretation, not an owner decision; the current display name is Apocryphⁿ.
The complete meter-to-invoice/credit-note/finality bridge remains absent (B04);
bare amounts authenticate no asset identity or durable cross-API invoice protocol.

## Network design audit

This 2026-10-21 work order restored all 29 tracked files from exact own-repository
commit `f1f2df76878d92c2ebe7ddc167f244d225fd9505` with `git archive`, without ref or
index changes. Earlier restoration paragraphs describe earlier stages only.
The current selected [NETWORK-THESIS](NETWORK-THESIS.md) is RESEARCH DESIGN.
This adoption changes documentation/registers only relative to that baseline;
source, tests, licence, notices, contract, checkers and workflows are preserved.
Static inspection is not product execution or an independent validation receipt.

| Entire current source/test surface | Observed source boundary and existing register ownership | Missing logic relevant to the network |
| --- | --- | --- |
| [lib.rs](../src/lib.rs) | Four module exports, error/result types and checked addition; no I/O or unsafe code permitted by crate attribute | No canonical service schema, authenticated event envelope, persistent store or adapter runtime; N01–N03 cannot be inferred from module names. |
| [admission.rs](../src/admission.rs) | `select` compares exact caller billing labels, profile/region/epoch, abstract slot count and price; F01–F03, A09/B02. Evidence age/challenge checks inspect caller assertions. Returns a borrowed advisory Offer. | No reservation, authentication, memory/device/runtime/topology appraisal, workload/model binding, accepted service or SLO validation. B03/B09 own authentication/appraisal; N01/N04/N05/N08 add service/resource/control/recall semantics. |
| [metering.rs](../src/metering.rs) | A03/A04: checked byte-second arithmetic, explicit GB/GiB/time, coverage/replay checks, cumulative floor. Immutable Rate per Meter; token scale metadata does not multiply price. | No authenticated usage, AI-token/device-time meter, rate-change protocol, shared contract/currency/invoice lineage, application acceptance or durable telemetry; N02/N05/N06. |
| [settlement.rs](../src/settlement.rs) | Single-owner in-memory map and balances; A01/A02/A05/F04/B01. App/version are caller strings; invoice/accrual are cumulative bare amounts; reconciliation is a boolean. Identity/same-state replay is a no-op. | No common Meter/Workload/Settlement authenticated context, transactional exposure/outbox, crash/fencing protocol or authenticated rail receipts (N02/N03). Refunded moves paid into disputed; release permits recollection. Permanent refunds/credit notes remain B04; durable recovery B07. |
| [policy.rs](../src/policy.rs) | A07: ordered thresholds produce Continue/StopNewAdmission/Drain/Export/Retain advice | No execution authority, actual export/retention, recall scheduler, controller or recovery guarantee. A08/B07/B08 remain; N04/N08 specify additional control and overhead obligations. |
| [regression.rs](../tests/regression.rs) | 30 specifications: 9 metering, 6 settlement, 14 admission, 1 depletion. E01–E09/E15–E16 map all names; two bounded properties include a rational expression and permutation enumeration. | All NOT_RUN. No end-to-end authenticated contract, durable crash/replay, capacity concurrency, hardware, economic, simulator or application acceptance tests. Same-session oracle authorship is not independent review; generated tests prove no external effect semantics. |
| [IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) and [SCIENCE.csv](../registers/SCIENCE.csv) | First owns status; second maps P01–P10 applicability and evidence gaps only. Existing rows remain unchanged. | New N entries are OPEN with validation NOT_RUN; prose and documentation checks cannot promote them. B03–B09 retain existing responsibilities. |

### Distinct network obligations

The following are planned positive/negative witnesses, not added executable tests.
For each N entry the authoritative status/validation is **OPEN / NOT_RUN** in the
register. All implementation modules and integration witnesses for the new behavior
are **NONE**; linked current source is a boundary under audit, not its implementation.
Supported domain is the selected public service-envelope research model unless a
row explicitly names later hardware qualification. Common assumptions are declared
clocks/dimensions, finite workloads and explicitly governed effect authority; missing
or unqualified inputs must reject/HOLD. [E17](EXPERIMENTS.md#e17-network-policy-and-simulator-family)
provides future independent experiments. Principle IDs refer to
[SCIENTIFIC-PRINCIPLES](SCIENTIFIC-PRINCIPLES.md).

| ID / principles / source boundary | Invariant and distinct obligation | Positive witness (planned) | Negative witness / missing apparatus (planned) |
| --- | --- | --- | --- |
| N01 / P01 P02 P03 P06 P08 / [Offer and Workload](../src/admission.rs) | One versioned envelope binds actor/site/pool, artifacts, multidimensional compatibility, time/locality, SLO, assurance, price/exposure and remedies. Adds explicit service semantics to B03's signature/contract obligation. | A compatible pinned public batch envelope remains equivalent after export/import and is eligible within its window. | Change model digest, memory, topology, locality, expiry or rate after commitment: reject/re-authorize, despite a cheaper quote. NONE: schema/compatibility conformance harness. |
| N02 / P01 P03 P05 P09 / [Meter](../src/metering.rs), [Workload](../src/admission.rs), [Settlement](../src/settlement.rs) | Stable contract/reservation/attempt/acceptance/invoice/payment identities and typed asset/scale/quantum lineage cross every boundary. Adds bridge identity and version obligations beyond B03/B04; does not duplicate legal-credit implementation. | Two attempts yielding one accepted artifact produce one accepted usage and one invoice allocation; replay/rotation preserves IDs. | Same ID with altered asset, scale, tenant, artifact or rate rejects; duplicate acceptance cannot bill twice. NONE: canonical typed lineage and independent invoice oracle. |
| N03 / P01 P02 P03 P05 P10 / [reserve and transition](../src/settlement.rs) | Atomically reserve exposure and persist authorized intent/outbox before effects; unknown retains bounds; stale writers fenced. Refines the collection protocol beyond B07's general recovery and A05's local transition. | Crash at each persistence/send/ack boundary then reconcile the same ID with conserved exposure. | Concurrent reservations, lost ack, rotated authority and duplicate callbacks cannot permit overspend or a fresh transfer; unsupported reconciliation remains UNKNOWN. NONE: durable store/outbox/adapter fault harness. |
| N04 / P02 P06 P08 P10 / [select](../src/admission.rs), [recommend](../src/policy.rs) | Local control remains bounded under stale regional advice/partition; shared operator/key/verifier domains are correlated. Adds control behavior beyond B08 site enrollment and B06 key release. | Site acknowledges one feasible reservation; broker loss still permits safe local recall/stop. | Two brokers race for one slot, expired summaries, one key controlling many nodes and verifier outage cannot count as independent capacity/assurance. NONE: federation and correlated-failure harness. |
| N05 / P02 P06 P07 P09 P10 / [Evidence](../src/admission.rs), [Meter](../src/metering.rs) | Application acceptance is distinct from execution correctness, semantic quality, confidentiality and availability. B09 appraises hardware; N05 evaluates useful delivered outputs with versioned rules. | Pinned evaluator accepts conformant output and records quantity/SLO with attempt lineage. | Authentic hardware returning wrong/low-quality/late output, provider self-grading or evaluator drift cannot silently earn accepted usage. NONE: independent application evaluators, quality fixtures and dispute harness. |
| N06 / P03 P04 P05 P09 / [Balances](../src/settlement.rs) | Customer money/liabilities, service credit obligations, provider collateral/delivery and contributor/governance records remain separate. Adds cross-record conservation beyond B04 credit-note semantics. | Redeeming a service credit reduces its obligation and links delivery without creating cash or votes. | Incentive issuance, a collateral deposit, an AI token count or a credit note cannot become earned customer revenue or corporate authority. NONE: multi-record accounting oracle; B04 still supplies permanent credits. |
| N07 / P03 P04 P08 P10 / [price selection](../src/admission.rs) | Rewards cannot substitute for independent demand; coalition profit and provider exit are evaluated net of internal flows. Adds adversarial incentive/exit experiments to B05 margin accounting. | Real customer receipts sustain declared margin with zero incentives and observed repeat demand in a qualified future cohort. | Self-buy/wash trades, split identities, colluding verifiers, price erosion and subsidy removal reveal unearned rewards or negative margins; report failure. NONE: economic/coalition and exit-response model. |
| N08 / P03 P07 P08 P10 / [capacity selection](../src/admission.rs), [depletion advice](../src/policy.rs) | Admit interruptible work only with feasible checkpoint/transfer/warm-up/recall costs and bounded customer exposure. Adds time-window feasibility beyond B05 aggregate costs/B07 restore. | A long compatible window completes accepted work after paying all overheads. | Short window, failed checkpoint, incompatible resume or owner recall leaves no profitable useful time; refuse or report waste/refund, never assume costless migration. NONE: recall-aware scheduler and calibrated overhead profiles. |
| N09 / P01 P03 P08 P09 P10 / [lib.rs](../src/lib.rs) | Versioned Scenario/Decision/Attempt/AcceptedUsage/EventTrace/RunManifest and pure adapter preserve causality, bounds, conservation and censoring. Distinct independent simulator contract; no current simulator implementation. | Same manifest yields the same ordered trace; simple exact fixture conserves work/resource/money/energy accounts. | Hidden future-event access, reordered ties, double-restored work, wrong units, infinite same-tick events or dropped censored jobs fail. NONE: simulator, adapter and conformance oracle in this repository. |
| N10 / P02 P06 P09 P10 / [public module boundary](../src/lib.rs) | Interchangeable SDK/provider adapters, self-host/export and maintained application versions support independent participation without proprietary integration. Adds portability/onboarding/maintenance obligations beyond A10 readiness. | Independent client/provider implementations round-trip contracts and results, export state and upgrade with conformance evidence. | Provider lock-in, proprietary-only schema, silent evaluator/model replacement or abandoned compatibility blocks qualification. NONE: SDK, provider adapters and adoption/maintenance evidence. |
| N11 / P01 P03 P07 P08 P10 / [regression boundary](../tests/regression.rs) | Policy advantage requires fixed quality/SLO, paired traces/seeds, held-out calibration and independent oracles; unknown parameters remain explicit. Distinct simulator-validity obligation beyond B05 hardware economics. | Frozen calibrated profile predicts held-out cases within preregistered bounds; paired comparison includes censored/failed work. | Future-informed policy, recycled calibration traces, discarded incomplete jobs or transferred paper gains invalidates the comparison. NONE: qualified simulator calibration and independent experiment harness. |

B03–B09 are not closed or reclassified by these additions. Permanent credit/refund,
hardware appraisal, independent key release, recovery and funded operations remain
missing. Documentation validation and preservation checks are distinct from all
product/hardware/simulator execution, which remains NOT_RUN.

## Commerce campaign increment (353c85212ea6)

This work restored own candidate a942d815ca441f48f988754cc39b55fe951d3885 via git
archive. It is new bounded work, not a retry of 38eb5bec0b67. Earlier sections are
historical baseline audits. Their assertions that all typed commerce, credits and
local journals are absent are superseded only for the new reference subset in
[COMMERCE](COMMERCE.md); legacy APIs and 30 regression witnesses are unchanged.
No upstream/sibling/private source was imported and no product was run.

C01–C08 in the sole status register own typed identity, atomic reservation,
acceptance/invoice lineage, outbox reconciliation, permanent credit/refund,
framed append/replay, generic CLI and manual campaign CI. All remain
IMPLEMENTED_UNVERIFIED / NOT_RUN. N01–N11 and B03–B09 retain their broader OPEN
obligations: these source additions cannot satisfy authenticated/legal/distributed,
physical-control, simulator-calibration or commercial requirements. E18 maps the
new named witnesses and independent evidence still required. The canonical
contract change grants only this campaign's independent coordinator review and
qualified manual fleet CI; no worker execution or self-approval is authorized.

## Campaign compatibility and public foundation review (2026-10-22)

Maintenance order f6bfe2f78105 started clean at exact own source
`72a210049f8254fce5a97191854046d56c5bc43a`; restoration was already satisfied.
M01–M03 record corrections; Q01 records the coordinator's scoped external
qualification in [EXPERIMENTS](EXPERIMENTS.md#external-qualification-note-2026-10-22).
Historical NOT_RUN authoring records above are preserved. This maintenance has
no product execution receipt and grants no source/checker acceptance.

- M01 / P01 P02 P10: the workflow required undeclared self-hosted/Linux labels
  and used runner expressions at job-level env, which GitHub rejects before
  execution. Use the selected route alone and GitHub runtime runner variables.
  Retain actual Linux/architecture, campaign and exact clean-commit checks.
- M02 / P01 P02 P09 P10: the dedicated image has standalone Rust/Cargo 1.99.1,
  while the helper required rustup and did not verify cargo's release/host.
  Resolve both installed binaries, reject proxies, verify both verbose versions
  against the exact pin/host, and pass the verified paths to offline execution.
  No toolchain installation, alias or network dependency is introduced.
- M03 / P01 P02 P06 P10: there was no explicit independent-checker review gate,
  and private-only documentation blocked standalone public contributions.
  Add the default-false manual gate and document the public Apache foundation,
  contributor offline feedback, conditional private vulnerability reporting and
  commercial/operator boundary. Skips never count as passes. The coordinator
  attests review; the boolean does not itself prove review or authorize deployment.

The positive/negative compatibility witnesses are specified in CI-RUNNERS and
remain unexecuted: matching reviewed inputs and both installed versions should
reach offline tests; false review/wrong campaign skips, and wrong route/host,
version or source rejects. The supported domain is the qualified trusted campaign
image, not arbitrary PATHs, hostile runners or another fleet. Static syntax and
structure checks are separate evidence; new manual owner-fleet CI remains needed.

Static disclosure and consistency inspection covered commerce source, its 14
witnesses, synthetic TSV fixtures and the unchanged smoke. Original Apache source
and synthetic labels are retained; no literal owner paths, credentials, operator
secrets, private imports or proprietary build dependencies were found in this
checkout review. This is a scoped static review, not a security audit or license
for referenced research candidates. Commerce still trusts external evidence IDs
and physical-resource labels; local lock/fsync assumptions do not provide
hostile-filesystem security, distributed fencing, authenticated acceptance or
payment finality. No semantic defect requiring a core change was found within
this maintenance scope. All Rust APIs, source, tests and fixture bytes remain intact.

Actual maintenance checks (2026-10-22): `agent_sync.py` generation and `--check`
passed; Python AST parsing passed for all scripts without importing or executing
the product helpers; both registers parsed with consistent column counts and
unique IDs (46 improvement rows and 10 principle rows). YAML parsing and static
workflow assertions confirmed manual-only dispatch, the default-false review gate,
exact campaign, route-only scheduling, runtime-context placement, read-only
permissions and no artifact upload. Helper AST inspection confirmed the verified
cargo variable and offline argument. Local documentation targets resolved; a
checkout text scan found no literal owner paths or selected credential signatures.
These are syntax/structure observations, not GitHub dispatch validation or a
security audit. `git diff --check` and the explicit scan including new documents
found no whitespace errors. Own-object comparison against the exact baseline
confirmed no changes to src/, tests/, fixtures/, Cargo.toml, LICENSE, COPYRIGHT,
AUTHORS.md, commerce_smoke.py or agent_sync.py. No product command, CI dispatch,
remote/fleet operation, commit or release acceptance was performed by this worker.
