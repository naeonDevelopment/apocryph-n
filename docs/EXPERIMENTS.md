# Apocryphⁿ experiment plan

Historical plan status at original source authoring: every experiment below was
**NOT_RUN**. Preserve those records; the dated external note below adds evidence
only for its exact source. This maintenance candidate remains NOT_RUN.
No Rust compilation, test, benchmark,
upstream reproduction, live service measurement or CI execution occurred in this
completion. Test source is an executable specification, not a result. Validation
fields in [IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) remain NOT_RUN; that CSV
is the sole change-status authority. Structure and whitespace checks are separate.

Agent product execution and release acceptance require qualified owner-fleet
receipts. Historical route uncertainty is updated by the scoped external note;
see [CI-RUNNERS](development/CI-RUNNERS.md). Independent contributors may run pure
offline tests on their own systems under [CONTRIBUTING](../CONTRIBUTING.md); this
does not authorize agent execution or substitute for release acceptance.

[Selected P01–P10 principles](SCIENTIFIC-PRINCIPLES.md) govern this plan;
[SCIENCE.csv](../registers/SCIENCE.csv) maps applicability without owning fix status.
Static coordinator baseline review is not reproduced upstream evidence. These
plans have no independent execution receipts; test authorship is not independent
review. Comparative experiments must preregister the task distribution, held-out
cases, sample plan, practical effect size, uncertainty analysis and failure criteria;
retain negative results and distinguish exploratory changes.

## Existing regression specifications

All names refer to `tests/regression.rs`. Expected outcomes are hypotheses encoded
in source, not observations. The 17 restored cases are retained; 11 cases were
added during completion, and two bounded properties in scientific adoption, for 30 total.

| Experiment | Actual fixtures | Positive / negative controls and falsifier | Execution |
| --- | --- | --- | --- |
| E01 dimensions and rounding | `decimal_month_regression`, `gib_is_not_gb`, `cumulative_fraction_survives_partition` | 1 GB × 2592000 s at 2500/GB-month expects 2500; GiB/GB expects 2684; two half-minor intervals yield cumulative 1, separately rounded 0 + 0. Any different cumulative dimensional result falsifies the fixture claim | NOT_RUN |
| E02 receipt integrity and coverage | `duplicate_and_overlap_are_distinct`, `unknown_coverage_never_becomes_zero`, `reordered_receipts_and_clipping_preserve_charge` | Identical replay accepted; altered replay/overlap rejected; reorder and clipping retain charge; zero known coverage accepted; missing/gap rejected. Unknown coverage returning zero falsifies the rule | NOT_RUN |
| E03 invalid/overflow arithmetic | `bad_durations_and_rate_rejected`, `overflow_is_explicit` | Zero/negative duration and zero period reject; valid positive period accepts; numerator and denominator overflow reject. Wrapping or silent saturation falsifies checked arithmetic | NOT_RUN |
| E04 identity and invoice bounds | `constructor_keeps_version`, `reserve_excludes_second_collector`, `balances_cannot_exceed_invoice` | App/version round trip, same-ID no-op, different amount conflict, distinct collector excluded, dispute bounded by invoice. Unexpected balance mutation or lost version falsifies the model | NOT_RUN |
| E05 ambiguity and terminal replay | `timeout_blocks_until_reconciled`, `success_and_refund_are_once_only`, `terminal_replays_and_identity_reuse_never_reserve_again` | Unknown blocks new reservation; explicit Failed releases; Paid/Failed/Refunded repeated with either flag leave state unchanged; resurrection rejects. Replay altering balances falsifies idempotence | NOT_RUN |
| E06 advisory offer constraints | `deterministic_minimum_and_tie`, `constraints_reject_expired_wrong_epoch_budget_region_capacity`, `duplicate_provider_is_ambiguous`, `profile_matching_is_exact_for_every_pair` | Reverse order preserves winner; all six exact profile pairs accept and unequal pairs reject; expiry/epoch/region/capacity/budget violations reject. Ineligible winner or unstable tie falsifies selection | NOT_RUN |
| E07 billing-unit comparison | `cheaper_wrong_billing_unit_is_ineligible`, `billing_unit_must_be_nonempty` | Matching 90 beats wrong-currency/quantum 1; wrong-only NoEligible; empty/whitespace Invalid. Numeric cheapness overriding unit equality falsifies F01 | NOT_RUN |
| E08 evidence freshness and binding | `confidential_gpu_requires_fresh_gpu_evidence`, `evidence_over_age_with_distant_expiry_is_ineligible`, `evidence_challenge_mismatch_is_ineligible`, `sensitive_work_requires_nonzero_challenge_and_age`, `correct_bound_evidence_accepts_inclusive_age_boundary`, `evidence_time_checks_reject_future_and_expiry_without_wrapping`, `public_profiles_do_not_require_evidence` | Positive bound challenge and age boundary accept across sensitive profiles; future/expired/over-age, wrong provider/epoch/profile/challenge reject; zero config invalid; public evidence optional. Acceptance of any prohibited case falsifies the local rule, not a hardware security claim | NOT_RUN |
| E09 depletion stages | `depletion_has_no_delete_stage` | Strict thresholds and boundary values end in Retain at zero; invalid equal thresholds reject. A deletion result or threshold acceptance would falsify policy | NOT_RUN |

## Missing experiments and apparatus

| Experiment | Required implementation / baseline | Controls, observations and falsifier | Execution |
| --- | --- | --- | --- |
| E10 broader independent numerical oracle | Bounded rational specification now present below; broad trace generator and independently reviewed/executed model still missing | Extend to changing usage, duplicate/gap/overflow sequences; log seed and minimize mismatches. Require equivalent cumulative windows to agree | NOT_RUN |
| E11 crash/unknown/reorg settlement | Missing durable journal, external adapter and fault-injection harness; compare against independent conservation model | Crash before/after submission and acknowledgment; duplicate/reorder finality receipts; timeout, conflicting receipts and reorg. No fresh transfer while outcome unknown; stable identity after restart. Any duplicate external charge or vanished reservation falsifies integration | NOT_RUN |
| E12 platform-specific security | Missing signed evidence verifier, hardware inventory and independent key release; compare qualified confidential platform with ordinary CPU/GPU baseline | Fresh genuine evidence positive; forged, stale, wrong challenge, wrong workload, revoked configuration and CPU-for-GPU negatives. Log verifier/version/configuration and rejection reason. Any invalid key release falsifies security boundary | NOT_RUN |
| E13 durable recovery | Missing replica protocol, funded sites, backups, key recovery and restore harness; compare conventional documented backup/restore | Corrupt one copy, lose site, lose client/operator keys, stop during replication; independently verify restored bytes and keys. Record RPO/RTO, completion evidence and unrecoverable cases. Missing data or unjustified completion falsifies guarantee | NOT_RUN |
| E14 performance and economics | Missing runnable provider service and cost instrumentation; matched ordinary service baseline | Pair hardware/workload/SLO; vary utilization, price, egress and failures; measure latency distribution, throughput, energy and all costs. Report uncertainty and break-even. Cost above achievable receipts falsifies assumed viability | NOT_RUN |

Future execution records must identify source commit, qualified runner identity,
OS/architecture/toolchain, exact commands, fixture seeds, raw logs, expected and
observed outcomes, start/end times and failures. A failed or incomplete run remains
such; retain negative controls and disclose absent apparatus. Record separate
hardware, service and commercial evidence rather than extrapolating a unit test
into a privacy, performance, recovery or revenue claim.

## Added bounded invariant specifications

- E15 / P03/P10 / A03: `meter_partition_invariance_bounded` uses both DecimalGb
  and BinaryGib, prices {0,1,2,7}, periods {1,2,3,7} seconds, bytes
  {0,1,unit/3,unit-1,unit,unit+1,2*unit}, and window lengths 1–6 seconds.
  Each constant-usage window is recorded whole and in every adjacent integer-second
  partition, inserted in reverse order. An independent expression adds reduced
  rational per-second fractions using explicit unit constants, then floors once;
  it does not call Meter arithmetic or ByteUnit::bytes. The finite bounds keep
  oracle arithmetic within u128. Zero usage/price and fractional charges are
  included. A two-half-minor negative control shows why separately floored invoice
  fragments are not the invariant. Any whole/partition/oracle mismatch falsifies
  the bounded property. Execution: NOT_RUN.
- E16 / P01/P03/P08/P10 / F01/B02:
  `provider_selection_permutation_invariance_bounded` enumerates all 5! = 120
  permutations for each of two wrong-unit variants (currency and service quantum).
  Two eligible equal-price providers test lexical ties, one eligible higher-price
  provider tests minimum selection, and cheaper wrong-unit and insufficient-capacity
  offers test exclusion before economics. Every ordering must choose eligible-a;
  the two ineligible offers alone must yield NoEligible. A different winner or
  acceptance of the negative-only set falsifies the property. Execution: NOT_RUN.

These deterministic enumerations need no random seed; record their source identity
and exact domain with any future receipt. They do not measure end-to-end performance,
prove arbitrary inputs, independently authenticate evidence or qualify a marketplace.
The full meter-to-invoice/credit-note/finality bridge and its integration experiments
remain absent. Structure compiler and whitespace success cannot change these statuses.

## E17 network policy and simulator family

**Proposed preregistration targets, not measurements. Execution: NOT_RUN.**
The [network thesis](NETWORK-THESIS.md) selects this research family; N01–N11 in
[IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) own the additional missing logic.
E11–E14 and B03–B09 retain their existing apparatus requirements. No simulator,
policy adapter, contract integration harness or calibrated hardware profile is
established here. The separate compute-cluster-sim project is being prepared; its
initial source, if any, is unqualified and was not inspected in this work order.
This plan neither executes it nor imports a dependency.

### E17 protocol and hypotheses

Supported initial domain: public portable batch/embeddings/evaluation with declared
resource compatibility and acceptance; endpoints form a separate workload stratum.
Freeze model/artifact, precision, tokenizer, semantic-quality evaluator/tolerance,
input distribution and latency/deadline SLO before policy comparisons. No model
quality reduction, output shortening or changed acceptance population may count as
a scheduling gain. P01/P03/P07/P08/P10 apply; authority/assurance/lineage subfamilies
also apply P02/P05/P06/P09, and incentive/governance separation applies P04.

H1: service-aware/recall-aware policy lowers all-in cost per accepted SLO-conforming
unit against the strongest feasible ordinary baseline under the same customer
quality and latency constraints. H2: gains persist without incentives after idle,
transfer, verification, retry, support and refund costs. H3: simulation can predict
held-out behavior within a declared domain. Failure of any is a valid research
outcome; do not rescue a failed result by changing the primary endpoint afterward.

| Comparator | Frozen behavior and information allowance |
| --- | --- |
| Eligible-cheapest | Minimize declared compatible price with deterministic stable-ID ties. Enforce the same common compatibility, acceptance and exposure limits as every online policy. Current `select` is only an abstract-slot reference, not this implementation. |
| Strict isolation | Dedicated compatible capacity, no opportunistic sharing or cross-tenant migration; account for idle/provisioning/turnover. It supplies a strong interference/control baseline, not free standby capacity. |
| Ordinary queue/backfill | FIFO queue with a declared reservation/backfill rule and estimated durations using only present information; preserve earlier reservations. Charge checkpoint/transfer and failed estimates consistently. |
| Service-aware/recall-aware | Optimize predicted accepted SLO-conforming useful work and all-in cost under resource, notice, locality and exposure constraints; forecasts use training data and observations only. |
| Perfect-information bound | Explicitly labeled nondeployable offline bound with future arrivals/failures/recalls known. Solve small cases exactly or report a certified optimization bound and gap; a clairvoyant heuristic is not a proven bound. Keep the same physics, quality, cost and exposure constraints. |

Online policies see only their versioned observation snapshots, current/announced
rates and notice that has actually arrived. Engine-private future traces, true
remaining duration and future acceptance labels must not leak into policies.
Randomness is injected with separately named streams. Pair policies on identical
exogenous workload/failure/recall traces and seeds; policy-dependent queues, energy
and exits evolve under the same transition rules, not forced identical outcomes.
Use common random numbers indexed by stable exogenous event identities, so differing
numbers of attempts cannot shift the shared arrival/failure draws.

Proposed initial sample plan: 30 paired seeds per prespecified workload/load/failure
stratum, with homogeneous and heterogeneous devices, low/moderate/overloaded demand,
long/short idle windows, independent/correlated faults and price-change conditions.
Freeze the stratum list and profile versions before execution. Estimate required
sample size from separate pilot variance if needed; version any amendment before
confirmatory runs. Separate pilot, calibration, validation and final comparison
traces; hold out time periods, workloads and hardware/runtime configurations where
supported. An unsupported held-out configuration remains out of domain.

Bound every run in integer ticks: proposed one-day scenario horizon, maximum
1,000,000 events, maximum 1,000 events at one tick and explicit per-contract retry
caps. These are starting preregistration targets, not product limits. Event-cap or
time-limit exhaustion marks an incomplete run and preserves remaining work and
liabilities. Predeclare warm-up treatment; report its startup/energy cost even if
a steady-state latency view separately excludes it. Include both fixed-horizon
results and a separately bounded drain-down sensitivity, without quietly extending
only unfavorable policy runs.

### E17 subfamilies and witnesses

All rows have execution NOT_RUN and require missing independent apparatus. Static
inspection of existing 30 Rust specifications does not satisfy these experiments.

| Subfamily / obligations | Positive witness and independent oracle | Negative controls and falsifier |
| --- | --- | --- |
| E17a envelope and lineage / N01 N02 N05; B03 B04 B09 | Independent schema/units oracle follows a compatible public request through two attempts, one accepted result, invoice and permanent credit. Expected identities/amounts are hand-derived for small fixtures. | Changed model/tenant/asset/scale/rate, insufficient memory or wrong topology; valid attestation with invalid output; duplicate acceptance; expired evidence before resume. Any prohibited acceptance or duplicated entitlement falsifies the boundary. |
| E17b exposure and crash replay / N03; A05 F04 B07 | Independently specified state machine checks reservation/outbox atomicity and per-asset exposure; crash each persistence/submission/ack boundary and recover same ID. Fake rail models durable external acceptance independently of local acknowledgment. | Lost ack, duplicated/reordered/conflicting receipt, concurrent collector, stale writer, identity rotation and credit then dispute-release. Any fresh transfer from UNKNOWN, overspend, lost obligation or resurrected credited debt falsifies integration. |
| E17c control and recall / N04 N08; B08 | Exact tiny scheduling oracle and physical occupancy trace confirm one feasible reservation per resource; safe recall after durable checkpoint and broker partition. | Stale regional state, two brokers, shared-key/verifier outage, zero notice, noncheckpointable task, failed checkpoint and transfer longer than idle window. Double occupancy, invented independent replicas or ignored overhead falsifies the model. |
| E17d money and adversarial incentives / N06 N07; B04 B05 | Independent double-entry/accounting calculation reconciles customer cash/liabilities, credits, collateral and reward funding; subsidy-free accepted receipts cover full provider costs in a declared synthetic positive case. | Zero demand/incentives, price erosion, provider exit, delayed collection, self-buy/wash-trade coalitions, verifier collusion and identity splitting. Internal transfers counted as new demand, unfunded rewards or omitted losses falsify accounting; negative real margin falsifies that scenario's viability. |
| E17e simulator conformance / N09 | Same manifest/seed yields identical trace; tiny exact schedules and a separately authored resource/work/money/energy oracle agree. Check chunked replay, equal-time order and horizon termination. | Leak future trace, reorder completion/recall ties, duplicate checkpoint work, change unit scale, create same-tick cascade, suppress censored jobs or double-count batch/host energy. Any undetected violation fails conformance. |
| E17f comparative policy and calibration / N08 N11; B05 | Paired traces compare all policies with independently measured held-out timing/power and frozen quality/SLO. Fit only on calibration split; preserve raw residuals and per-stratum uncertainty. | Oracle-informed online policy, recycled held-out data, unsupported extrapolation, cheap low-quality model, zero-overhead migration, free idle energy and dropping failed/incomplete jobs must expose apparent false gains. |
| E17g portability and adoption / N10; A10 | Independently implemented client/provider round-trip schema and export without proprietary services; a future qualified cohort records onboarding, maintenance, repeat demand and provider retention. | Operator-specific hidden fields, unavailable maintainer, breaking model/evaluator upgrades, failed export and reward-dependent repeat traffic block the corresponding adoption claim. No live cohort is authorized by this plan. |

A deliberately flawed controller or accounting mutation must be caught by the
independent oracle, not just agree with another call into the same implementation.
Separate authorship/review of oracles and raw receipts is required. A perfect-
information optimizer is a scheduling comparison only; it cannot serve as an
independent security, money or empirical timing oracle.

### E17 outcomes, proposed targets and censoring

Primary outcome: total attributable cost divided by accepted SLO-conforming useful
units, with fixed quality/evaluator. Include all attempted/rejected/retried work,
idle hardware/power, replacement, network/egress, checkpoints/warm-up, verification,
fees, support, refunds and provider turnover exactly once. If there are zero
accepted units, report undefined/infinite cost with total spend and zero delivery;
do not omit the run. Report customer price, earned revenue and collected cash
separately from modeled incentive transfers, and report provider margin with and
without incentives.

Report per-stratum and aggregate acceptance, deadlines, TTFT/TPOT and total latency
where relevant, throughput, queue wait, resource occupation, energy, overhead,
provider exits, rejected/deferred jobs and customer exposure. Show p50/p95/p99 with
uncertainty and sample counts. Carbon is optional only with time/site-specific
intensity and declared host/cooling/embodied boundary; no carbon number follows
from GPU utilization alone.

At horizon retain every arrived job in the denominator. Separate accepted,
rejected, failed, abandoned, queued, running and checkpointed outcomes; list
unsettled payment/credit liabilities separately. Report completed-job latency only
alongside censoring counts and deadline attainment for all arrived jobs with
deadlines within the horizon. Use a prespecified survival/bounds analysis for
unfinished latency, or explicitly label it unknown. Censored jobs are not successes
and their incurred costs remain included. An event-cap abort is not an ordinary
completed sample.

Proposed numerical acceptance targets for future preregistration:

- Zero detected resource/work/money violations, unauthorized acceptances or future
  information leaks in bounded conformance fixtures. Finite zero failures is no
  proof of arbitrary-input correctness or fraud detection probability.
- Held-out median relative error at most 10% for latency and energy, and p95 relative
  error at most 20%, per supported stratum. Define absolute tolerances for zero/near-
  zero denominators before data collection. Report errors outside targets; do not
  pool away a failing configuration or inherit accuracy from another simulator.
- For an H1 advantage claim, at least 5% lower all-in cost than the strongest feasible
  ordinary comparator, with a paired 95% confidence interval excluding zero gain;
  quality must remain within its frozen acceptance criterion and SLO attainment
  must not worsen by more than 1 percentage point or breach the contract's own
  limit, whichever is stricter. Choose primary strata/comparisons in advance and
  correct multiplicity for additional confirmatory claims.
- H2 requires nonnegative full-cost margin without incentives in each claimed viable
  provider cohort, including replacement and exit costs. Price-erosion and adverse
  scenarios may fail; those failures constrain the claim rather than get averaged
  away. No fixed return or universal provider profitability is promised.

These numbers are proposed decision targets, not results, validated risk limits or
contractual service promises. Freeze confidence interval method (paired bootstrap
by independent trace, preserving within-trace correlation), treatment of missing
data and stop/amendment rules before execution. Sensitivity analyses must vary
unknown recall/detection/correlation/cost assumptions; unresolved privacy and
collusion cannot be converted into safe operation by a favorable economic mean.

Each future RunManifest must carry source/config/schema/backend/profile/seed and
oracle identities, exact commands, qualified runner/hardware/software, data rights,
calibration split, event ordering, bounds, raw event traces, expected and observed
outcomes, censoring, failed runs and independent review. Publish negative findings
and distinguish exploratory follow-up from confirmatory evidence. All product,
hardware, simulator and commercial experiments in this order remain NOT_RUN.

## E18 bounded commerce source qualification

Status: NOT_RUN. C01–C08 own IMPLEMENTED_UNVERIFIED source; P01/P02/P03/P05/P06/
P08/P09/P10 apply. [COMMERCE](COMMERCE.md) specifies exact domain, assumptions,
wire format, conservation equations and recovery limitations. E17 remains the
larger external simulator/distributed/security/market research family; this source
does not supply its missing empirical apparatus.

| Witness in tests/commerce.rs | Expected positive outcome and adversarial falsifier |
| --- | --- |
| typed_assets_units_scale_and_overflow | Same-asset 1+2=3; every differing identity component/unit and u128 overflow rejects. Any dimensional mixing or wrap fails. |
| atomic_budget_capacity_and_interval_overlap | Two quantity-one overlaps fit capacity two; adjacent windows reuse it. Third overlap, undersized budget or capacity downgrade rejects with all state unchanged. |
| epoch_version_expiry_and_release_are_fenced | Existing invoice retains original rate after offer replacement; close converts maximum hold to actual obligation. Stale offer/epoch and expired acceptance reject; early expiry cannot release. |
| changed_identity_replay_never_mutates | Exact command returns Replay. Changed timestamp or reservation/offer payload under the same identity rejects without mutation. |
| cumulative_origin_rounds_once_across_attempts | At price 1 per 3 units, cumulative 1/2/3 yields 0/0/1 despite two attempts and duplicate acceptance. Rebound evidence, backward/future units and wrong invoice/unit reject. |
| timeout_then_bound_terminal_reconciliation | UNKNOWN survives expiry and blocks another collector/submission. Explicit evidence resolves to FAILED and permits collectible release; conflicting outcomes and empty evidence reject. |
| permanent_credit_refund_and_dispute_conservation | Hand-derived gross20/paid13/credited7/refunded7/cash20; unpaid credit15 then dispute release5 yields collectible5. Recollecting credits or refunding unpaid credit fails. |
| failed_commands_rollback_evidence_and_overflow | Rejected paid credit consumes neither clock nor evidence. Oversized rate product rejects without capacity or budget residue. |
| durable_restart_replay_and_changed_duplicate | Recovered state equals synchronized state; UNKNOWN and exact duplicate retain identity with no new bytes. Changed duplicate or second submit rejects. |
| ambiguous_partial_write_and_lost_sync_poison_without_retry | Inject zero bytes, short append and full append with failed sync acknowledgment. Every case poisons; no automatic retry writes. A full record may replay despite caller error. |
| strict_framing_corruption_truncation_and_bounds | Replay rejects every proper nonempty prefix of one record, every one-bit byte mutation in a tiny log, appended duplicate, malformed/version/operation/numeric input and oversize input. Whole-suffix deletion remains explicitly undetectable without external manifest. |
| replay_revalidates_semantics_even_with_valid_framing | Independently framed records with valid checksums still reject oversubscription and changed or redundant command IDs. Framing correctness cannot bypass the state machine. |
| every_operation_has_one_canonical_wire_spelling | Every enum variant round-trips; extra columns and CR suffixes reject. |
| file_store_excludes_writers_restarts_and_retains_stale_lock | Real qualified-filesystem clean restart retains state; second writer and stale lock reject; malformed log remains locked. Fixture explicitly proves its own handles dropped before removing a stale lock. |

`fixtures/commerce.tsv` and `fixtures/commerce.expected.tsv` specify an independent
hand arithmetic target (authored in this session, not an independent review).
`scripts/commerce_smoke.py` must execute real CLI commands and compare exact summary
bytes on first run and second-process replay, then reject malformed input without
losing prior state. Preserve all 30 original tests and run the full offline suite,
not only new witnesses. Finite positive results are not universal proofs.

Before execution, a separate coordinator reviewer must inspect source, checker,
workflow and test boundaries, derive conservation and interval examples without
calling the implementation, and qualify exact source/toolchain/image/fleet route.
Record manual route (default amd64 or campaign-only arm64), raw cargo and smoke
logs, expected versus observed outcomes, compiler version/host and filesystem,
independent reviewer/oracle identity and any failing case. No architecture result
is inferred from the other route. No artifacts are uploaded by this workflow.

Further apparatus required: actual power-loss/fsync testing, stale-writer and
hostile filesystem tests under a qualified isolation model, external high-water
manifest, authenticated offer/acceptance/payment/refund evidence, simulator bridge,
physical resource oracle and legal/market evaluation. The injected sink models
possible I/O outcomes but is not a power-loss receipt. Any duplicate external effect,
credited debt resurrection, unqualified local execution or unsupported assurance
claim falsifies the corresponding integration claim. Preserve failures and keep
statuses unverified until scoped external receipts are recorded.

## External qualification note (2026-10-22)

Coordinator-observed evidence supplied to maintenance order f6bfe2f78105:
exact original commerce source `72a210049f8254fce5a97191854046d56c5bc43a` passed
`cargo test --offline` (14 commerce plus 30 retained tests), `cargo build --offline`
and actual CLI fixture/restart/replay on qualified owner Linux ARM64 with standalone
Rust/Cargo 1.99.1. This worker did not execute those commands, access private
qualification material or independently reproduce the observation. Q01 in the
register records that attribution; historical NOT_RUN rows above and C01–C08
remain as authored rather than being silently overwritten.

This observation is limited to the named source and environment. It does not
qualify amd64, other toolchains, physical power loss, hostile storage, authenticated
external assertions, legal accounting, a deployed service or the research roadmap.
The coordinator also reported external qualification of a paired simulator; no
sibling files or private fixtures were read, and no simulator result is used here
to qualify commerce or assert integration readiness.

The malformed draft workflow was disabled pending correction. This maintenance
source changes workflow/toolchain compatibility and documentation only; original
Rust source, tests, fixtures and smoke semantics are preserved. A new actual manual
CI run of the maintenance commit and independent source/checker review remain
required. Runtime helper positive/negative witnesses in
[CI-RUNNERS](development/CI-RUNNERS.md) remain NOT_RUN. Static documentation checks
are neither checker acceptance nor product qualification.
