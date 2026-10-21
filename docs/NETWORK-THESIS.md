# Apocryphⁿ network thesis

Current selected **RESEARCH DESIGN**, 2026-10-21. This reference specifies an open
compute, intelligence and operations network; it does not describe an implemented
network. The selected baseline is this repository's own commit
`f1f2df76878d92c2ebe7ddc167f244d225fd9505`, restored by `git archive` without changing
refs or index. Existing product source and tests remain IMPLEMENTED_UNVERIFIED /
NOT_RUN. [IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) is the only implementation
status authority; [FIXES](FIXES.md) explains its missing-logic entries.
[ARCHITECTURE](ARCHITECTURE.md) continues to own the actual v0.1 API semantics.

The design selects accountable service contracts, independently usable clients and
providers, and experiments against conventional service provision. It selects
conventional invoicing with optional supported payment rails as the first economic
comparator. Native transferable issuance is an **unselected research alternative**.
There is no selected supply cap, yield, fixed ROI, token sale or funding promise.
An AI token is a tokenizer-dependent unit of model input/output, not a payment token
or corporate vote. Model output is advice unless separately authorized under a
bounded operations contract; buying inference never grants arbitrary tool authority.

## Claims and supported domain

**Source-backed prior art** is scoped in the reference table below. **Selected
design** means the normative boundaries in this document. **Engineering hypotheses**
are that compatible service envelopes, local control and recall-aware placement can
improve accepted useful work at a fixed quality/SLO and sustainable cost. They may
fail under low demand, high transfer cost, correlated failures or poor calibration.
**Actual measurements** for Apocryphⁿ network, hardware, simulator and commercial
operation: NONE; all associated experiments are NOT_RUN. Literature results are
not reproduced here, and no numerical gains or scientific novelty are claimed.

Initial domain: explicitly public, portable batch, embeddings and evaluation work
with versioned acceptance rules. Online endpoints require separate latency and
operations qualification. Confidentiality, training and distributed expert graphs
need additional apparatus. Missing evidence yields HOLD or a declared unsupported
service; predicted profit cannot bypass eligibility. P01–P10 from
[SCIENTIFIC-PRINCIPLES](SCIENTIFIC-PRINCIPLES.md) remain normative, including separate
permission/effect authority and independent attempts to falsify claims.

## Selected service envelope

A versioned service envelope is the common commitment across offer, reservation,
attempt, acceptance and invoice. This is a schema design, not a new Rust type.
A signature authenticates an issuer's commitment, not the truth of resource or
performance assertions. Select canonical encoding, integer dimensions and explicit
schema rejection before signatures; bind amendments by new versions and consent.
Never reinterpret an existing identifier under changed terms.

| Bound field group | Selected meaning and rejection boundary |
| --- | --- |
| Contract and actors | Stable contract ID/version, tenant/client authority, operator ID, site ID, pool ID and authorized signer with role, key epoch and delegation scope. Record ownership/control and verifier dependencies. Names, keys and machines are different identities; rotation cannot erase obligations. |
| Work and artifacts | Workload ID/version; model digest, tokenizer, precision/quantization and runtime version; executable/container, input and output artifact digests; permitted licensing/use and data handling scope. Bind random seed or allowed nondeterminism and evaluator version. Unknown or substituted artifacts require a new decision. |
| Compatible resources | Vector of slots, host/device memory, device class/count/features, runtime/driver compatibility, storage and link capacity, plus topology/placement constraints. State exclusivity, sharing and fragmentation assumptions. Slot counts alone neither establish memory feasibility nor make devices interchangeable. |
| Time and interruption | Availability interval on a declared clock, reservation expiry, start deadline, maximum duration, owner recall policy, minimum notice, checkpoint interval/format/durability, restore compatibility, resume location and lost-work liability. Abrupt failure remains possible; a notice promise is not a guarantee of checkpoint completion. |
| Locality | Allowed sites/regions and data movement paths, input/cache residency, bandwidth/egress limits, retention/deletion commitments and client-approved relocation. Admission rechecks locality before transfer or resume. |
| Quality and latency | Output acceptance predicate, correctness tolerance and semantic evaluation set; deadline or separately specified TTFT/TPOT/total-latency percentiles, population and measurement window; availability and rejection budgets. Model equivalence cannot be inferred from a common API. |
| Assurance | Exact policy/version, hardware/firmware/driver reference values where applicable, trusted issuers/verifiers, workload/session/tenant binding, issued/expiry/max-age checks, revocation and renewal rules. Expiry or unknown appraisal during a run invokes the declared pause/stop/key policy. |
| Price and exposure | Typed asset identity, integer minor-unit scale, service quantum, rate version/effective interval, cumulative rounding origin, tax/fee responsibility and maximum authorized exposure. Distinguish input/output AI tokens from byte-seconds or device-seconds. Reserve worst-case authorized billable attempts/egress/fees within the cap, or refuse/renegotiate. No implicit currency conversion. |
| Acceptance and remedies | Authorized evaluator and acceptance deadline, required output/evidence receipts, partial acceptance rules, retry limits and payer, dispute window, credit/refund basis, permanent credit identity and collection consequences. Timeout is pending/disputed according to explicit terms, never silently accepted. |

Clients must be able to inspect/export the complete envelope before reservation.
The contract must say whether reserved capacity itself is billable. A capacity
availability service can have its own accepted delivery quantum; it must not be
silently charged as accepted application output. Changes to price, runtime,
assurance or output rules do not retroactively change completed accepted usage.
N01/N02 track these additional obligations; B03 still owns signed offers/contracts,
B09 hardware appraisal and B04 permanent invoice credits.

## Lifecycle, identities and accounting boundary

The selected lifecycle separates facts that the current APIs cannot authenticate:

| Stage | Fact recorded and authority boundary |
| --- | --- |
| Advice | Candidate provider and reason from an observed snapshot. No reservation, budget spend, authentication or permission follows from `select`. |
| Authenticated reservation | Client-authorized contract version, resource/time allocation and bounded exposure recorded with a stable reservation ID. A site controller arbitrates capacity; broker advice cannot sell the same resource twice. |
| Execution attempt | Unique attempt ID linked to reservation/work ID, actual runtime/artifacts, start/stop/checkpoint events and bounded authorization. Retries are separate attempts; success of transport is not output acceptance. |
| Accepted usage | Evaluator-authenticated acceptance ID over output, attempt lineage, dimensions and accepted quantity. Replayed delivery cannot earn twice; rejected or wasted work is recorded with cost and the contract's liability allocation. |
| Invoice | Stable invoice ID/version and line IDs tied to accepted usage, rate version, asset/scale and rounding origin. No cross-asset aggregation. Amendments/credits preserve original lineage. An invoice is a receivable, not money received. |
| Durable collection intent | Stable operation ID, exact invoice allocation, payee/rail and authorization, reserved collectible balance and transactional outbox record committed together before dispatch. Acknowledging an existing ID grants no new send. |
| Uncertain/reconciled payment | Timeout or crash after possible submission retains exposure and triggers adapter-specific inquiry. Authenticated receipts establish reconciled outcomes under declared finality rules; request-sent, hash or local boolean does not. Conflicts remain quarantined. |
| Permanent credit | Durable credit note linked to invoice/accepted usage and refund intent where money must move. Credit extinguishes the specified obligation permanently; completing a refund is a separate reconciled payment event. Dispute release must never revive credited debt. |

Stable business IDs must survive restart, retries, authority/key rotation and adapter
replacement. Select a namespace plus issuer identity and canonical payload digest;
identical replay returns existing state, while reused IDs with changed payloads
reject. Attempts and payment retries cannot be conflated. Persist authorization,
resource/exposure reservation and outbox atomically at the relevant authority;
select fences against stale writers and authenticated cross-authority acknowledgments.
There is no assumed global transaction spanning sites and payment rails. A partial
resource hold expires/releases under an explicit state machine; an uncertain
payment does not expire into permission to send again.

The next implementation priorities are stable identity/typed lineage (N02),
transactional exposure reservation and outbox (N03), and crash/replay/concurrency
witnesses before any effect adapter. Inject faults before/after commit, submission,
external acceptance and receipt persistence. Reconcile the same operation after
restart. If a rail cannot establish an outcome, retain UNKNOWN and escalate rather
than retry under a fresh ID. Local ledger idempotence **cannot establish exactly-once
external transfer**. B07 recovery and B04 permanent credits remain prerequisites.

For each asset, a future accounting oracle must distinguish gross invoice obligation,
permanent credits, net confirmed collections, open collectible debt, disputed debt
and reserved collection debt. Their sum reconciles to the net obligation, with
any overpayment/refund payable separately recorded as a customer liability.
Customer cash is a separate stock: opening cash + external receipts - external
payments = closing cash. Neither a credit note nor an incentive creates cash.
Current `Balances` cannot represent this complete accounting model.

## Control planes and independence

Site-local scheduling/control owns device placement, memory, queues, batching,
checkpointing, admission and immediate recall. Regional brokers/federation operate
more slowly over versioned availability summaries, prices, locality and assurance
results. They must tolerate stale summaries and partitions; the site's authenticated
reservation acknowledgment is decisive. The client SDK and provider adapters are
operator-independent interfaces, with bounded capabilities and exportable records.
Application evaluators determine output acceptance. Separate evidence appraisal
services decide whether a specified trust policy is satisfied; separate settlement
adapters handle invoice delivery, collections, receipts and reconciliation.

No blockchain belongs in the per-token inference loop or five-second physical
control loop. Local operational safety and stopping admission must work during
broker/payment unavailability. A five-second period is a scenario parameter, not a
measured scheduler capability. Any future settlement rail is asynchronous to
physical execution and separately qualified. Operations proposals cannot expand
trust roots, evaluators, reserved human decisions or spending limits.

Multiple nodes owned by one operator, controlled by one key or dependent on one
verifier are not independent failure domains. Record shared power, network, site,
software, administration, custody and verifier control. Model correlated outages
and collusion by these groups, not a binomial count of node names. B08 owns funded
site qualification; N04 adds control-plane and correlated-domain behavior tests.
Recovery requires verified replicas, funded retention and recoverable keys (B07/A08);
`Retain` supplies none of those guarantees.

Manna is an optional, separately qualified proprietary integration. No Manna runtime,
source or terms are required for network participation, scheduling, settlement or
simulation. The network and independent simulator must remain useful without it.
No proprietary configuration, source imports or licence changes are selected.

## Economics and incentives

Keep four distinct records: customer money and liabilities; service credits and
their redemption obligations; provider delivery and collateral; contributor and
governance records. Service credits have explicit issuer, denomination, redemption,
expiry and refund rules; they are not assumed transferable, cash-equivalent or
voting rights. Collateral is encumbered provider property, not operating revenue.
Contributor rewards recognize independently accepted work and ongoing maintenance;
governance rights require a separately chosen institutional policy (P04), not
inference token counts, payment balances or claimed hardware supply.

The following is a selected accounting derivation for experiments, not measured
prices or investment advice. Fix one asset and accounting period of H hours. Let
K be qualified capacity, q the calibrated service quanta per capacity-hour under
fixed quality/SLO, and u_a = Q_accepted/(K q H), the accepted billable utilization.
Physical busy utilization u_b includes retries, rejected work and verification;
u_a cannot be substituted for u_b in energy costs. For a homogeneous illustrative
profile with constant power by state:

```text
Q_accepted = K * q * H * u_a
Gross invoice = p * Q_accepted
Customer receipts = Gross invoice - credits - noncollection - receivable increase
Energy kWh = H * [u_b * P_busy + (1-u_b) * P_idle] + E_transfer_warm_checkpoint
Cost = energy_price * Energy + hardware_replacement + egress + verification_retries
     + rail_platform_fees + support + refunds_paid + other_operating_costs
Provider cash margin excluding incentives = Customer receipts - Cost
Provider margin including incentives = Customer receipts + incentives_received - Cost
```

Here P values cover the declared fleet in kW; host/cooling and idle power must be
included. Use an integral over calibrated states for heterogeneous equipment.
Hardware replacement allocates net replacement cost over realistic economic life,
including obsolescence and failure, even when equipment is already owned.
`verification_retries` includes only costs not already counted in energy, hardware
or egress. Credits reduce invoices; refunds_paid are cash outflows. Do not subtract
the same refund twice: for a prior-period overpayment, track liability settlement;
for a current invoice credit already netted from receipts, exclude that same cash
reversal from a second cost deduction. Reconcile cash and accrual margin separately.
Receivable increase includes collection timing; prepaid cash remains a liability
until delivery, not instant earned margin. Report period alignment and attribution.

Under a simplified collected unit contribution c > 0, fixed non-incentive cost F
and remaining variable cost already deducted in c, break-even accepted utilization
is F/(K q H c). If c <= 0 or this utilization exceeds feasible availability, there
is no sustainable operating point in that case. Derive c from accepted paid work,
not advertised device throughput or nominal token rewards. Include collateral
financing/lockup opportunity cost and required return separately, with assumptions;
no return is promised.

Required stress cases are zero incentives, falling customer prices, higher idle
power/egress/refunds, slow collection, correlated outage and provider exit. Exit
removes capacity and raises queue/replacement/turnover costs; rerun service
feasibility rather than holding supply fixed. Incentives are separately funded
transfers, never evidence of real demand or a substitute for customer receipts.

N07 tests self-buy and wash-trade coalitions: combine client, provider and verifier
accounts under shared control; net their internal payments away and include fees,
energy, fraud losses and incentive receipts. Replayed work, synthetic demand,
collusive acceptance and split identities must not mint extra reward or appear as
independent paying customers. Compare subsidy-free cohort retention and provider
margin. If a coalition profits only by circular rewarded traffic, revise or reject
the incentive mechanism; do not report that traffic as adoption.

For an idle window W, useful time is bounded by
`max(0, W - transfer - warmup - checkpoints - restore - recall_drain)` with
non-overlapping overhead durations and explicit lost work on interruption.
Expected accepted revenue must exceed all attributable costs and owner opportunity
cost, including the tail risk of recall before an accepted checkpoint. Transferring
large state, warming model weights and reserving safe recall time can consume the
entire window. Repeated migrations can cost more than the work they salvage.
Checkpointability is a workload/runtime compatibility requirement, not a scheduler
wish. N08 tests short windows, no-checkpoint tasks and failed checkpoint storage.

Collateral does not repair unknown fraud detection probability, correlated verifier
collusion or irreversible privacy harm. A stylized deterrence inequality comparing
attack gain with detection probability times collectible penalty is meaningful
only when detection, independence and enforceability are established. Unknowns
remain explicit; a larger bond cannot turn them into a confidentiality guarantee.

## Application and adoption layers

| Application sequence | Qualification and delivery obligation |
| --- | --- |
| Public batch, embeddings, evaluation | Portable input/output contracts, pinned artifacts, repeatable evaluators, deadlines and explicit retry costs. Public workloads still need integrity and real customer acceptance. |
| Maintained managed endpoints | Versioned model catalogue, compatibility, incident ownership, latency/availability budgets, upgrades, calibration and support. Cheap one-off inference is not a maintained service. |
| Dedicated nodes | Reservation/exclusivity, provisioning and sanitization, tenant turnover, reimaging, cold start and idle-between-tenant costs. Charge terms must identify capacity delivery separately from application results. |
| Confidential service | Only the exact independently qualified hardware/software configuration, threat assumptions, appraisal and key-release path. Enum/profile labels are insufficient; physical attack and side-channel exclusions must be stated. |
| Later expert/DAG routing | Per-edge artifacts/locality, bounded fanout, evaluator propagation, compositional cost/latency and partial failure. An expert label is no semantic-quality guarantee. |
| Later checkpointed training and specialized compute | Compatible optimizer/checkpoint formats, reproducibility/convergence criteria and calibrated specialist profiles; qualification per task/device domain rather than a universal interchangeable-compute claim. |

Qualify correctness, semantic quality, confidentiality, availability and commercial
viability separately. A byte-identical output may be semantically wrong; a useful
answer need not prove its execution; attestation need not imply either. Each
service class needs its own evaluator, adversarial negatives and evidence expiry.
Acceptance must record disagreements and permit dispute, not let providers grade
their own work without declared trust. N05 adds this application boundary beyond
B09 hardware verification.

The selected OSS scope is core contracts, SDK, protocol, independent simulator and
conformance specifications. Commercial scope is maintained managed operations,
support, calibration, custody and integrations under separately qualified authority.
No licence or proprietary rights change follows from this division. Network client
and provider implementations must be replaceable and support self-host/export.

Adoption criteria include reproducible contributor setup and experiment manifests;
client budget/acceptance/export onboarding; provider capacity/assurance/recall/cost
onboarding; clear maintenance ownership and rewards for sustained compatible
releases; and provider margins sustained by real customer receipts. Measure time to
first conformant contribution/job, migration/export success, support burden,
repeat customers, maintenance continuity and provider survival after incentives
end. These are future observations, not claims of present adoption (N10).

## Independent simulator boundary

A separate general-purpose project, **compute-cluster-sim**, is being prepared.
Its initial source, if any, is not qualified. This work did not inspect or modify
that project. No runtime dependency or adapter is established. The selected
boundary is data exchange and a future pure policy adapter; the simulator must
support non-Apocryph policies and ordinary clusters without network accounts,
proprietary integrations or settlement rails.

| Versioned boundary | Required contents and constraint |
| --- | --- |
| Scenario | Schema/version, initial resource vectors and control/failure domains, workload arrivals, allowed information, availability/recall/failure/rate traces, time units, finite horizon/event cap, seeds, calibrated profile IDs and explicit unknown ranges. Future exogenous events stay private to the engine. |
| Decision | Policy/version and decision ID, observation timestamp and snapshot digest, eligible candidates, allocation/HOLD and reasons, requested resource/exposure changes. A decision is neither execution nor future-trace access. |
| Attempt | Work/reservation/attempt lineage, placement and resource occupation, start/stop, checkpoints, failure/recall causes, transfer/warm-up/retry work and cost; includes unsuccessful attempts. |
| AcceptedUsage | Acceptance ID and evaluator/version, attempt/output references, accepted quality and SLO evidence, typed quantity/asset/rate lineage and rejection/credit reason. Simulated acceptance is labeled modeled, never an authenticated production receipt. |
| EventTrace | Ordered events with event IDs, parents, time, type, resource/work/energy/money deltas, observations, decisions and termination/censoring reason; append corrections explicitly rather than erase history. |
| RunManifest | Exact source/config/schema/policy/backend/profile/oracle versions and digests, seeds, commands, qualified environment, calibration/held-out split, bounds, expected versus observed invariants, raw outputs, unknowns and independent review identity. |

Select `decide(versioned_observation, policy_config, supplied_randomness) -> Decision`
as the future pure adapter boundary. Validate decision feasibility in the engine;
no I/O, credentials, self-modifying evaluator or hidden clock/randomness. Unknown
schema versions reject unless an explicit lossless migration is qualified. Current
`select` cannot implement this boundary without translating abstract-slot semantics
and declaring missing resource, reservation and acceptance behavior.

Use integer simulation ticks and a versioned total event order, for example
`(time, phase, stable_event_id)`. At equal time, first accrue work/energy on the
preceding half-open interval, then apply end/recall/failure and rate/expiry changes,
then admit/dispatch. Explicitly specify completion-versus-recall ties and test both
boundary cases. Bound time, total events, same-tick cascades and retries; exhausting
a bound produces a censored/incomplete run, never silently completed jobs.

Conservation oracles must check component-wise resource capacity and nonnegative
holds; work partition into completed/accepted, rejected/wasted and remaining
without duplicating restored checkpoints; per-asset money/liability reconciliation;
and energy integration including idle, transfer and restart states without double
counting shared hosts/batches. Energy is consumed, not a conserved money-like stock:
its accounting invariant is supplied energy = attributed consumption + declared
losses/residual. Model owner recall, failure/repair, demand and rates changing
mid-run. Preserve original rate commitments or apply explicit amendment semantics.

At the horizon, retain queued, running, checkpointed, unsettled and abandoned jobs
as distinct outcomes with incurred cost and remaining obligations. Report censoring
and SLO denominators; never improve apparent latency by discarding unfinished jobs.
Calibrate task/device/runtime profiles on declared training data, freeze them, and
validate on held-out configurations/traces. Unknown detection rates, correlations,
energy and quality models require ranges or unsupported labels, not invented means.
N09/N11 and [E17](EXPERIMENTS.md#e17-network-policy-and-simulator-family) own the
boundary and planned falsifiers.

Specialist backends are optional future integrations: SimGrid for distributed
systems/resource models, Vidur for profiled LLM inference, ASTRA-sim for distributed
ML systems and SimAI for training/communication studies. They are not dependencies
or rebuilt timing models in this repository. Any future adapter needs pinned
versions, licence review, units, calibration domain and cross-backend checks; an
integration cannot inherit a paper's accuracy outside its measured domain.

## Scoped primary research context

Public landing pages/abstracts and the relevant market/carbon full-text sections
were inspected for this adoption; no source code was imported and no experiments
were reproduced. These are prior art, not implementation-status evidence. The
selected design and hypotheses above are our proposed synthesis.

| Primary source | Supported scope and limit for this design |
| --- | --- |
| [DistServe, OSDI 2024](https://www.usenix.org/conference/osdi24/presentation/zhong-yinmin) | Separates prefill/decode allocation under TTFT/TPOT and bandwidth constraints. Motivates phase-aware service/SLO profiles; its serving results do not establish WAN marketplace economics or transfer here. |
| [Mooncake, FAST 2025](https://www.usenix.org/conference/fast25/presentation/qin) | KV-cache-centric disaggregation and scheduling motivate memory/cache/locality accounting. Its deployment context does not qualify unrelated providers or our network. |
| [Parallax, September 2025](https://arxiv.org/abs/2509.26182) | Two-phase model placement and request pipeline selection address heterogeneous GPUs and limited links in evaluated volunteer settings. It does not prove arbitrary resource fungibility, confidentiality or viable margins. |
| [IR3DE-AXL, 8 September 2026](https://www.gensyn.ai/research/ir3de-axl-collective-inference-network) | Prototype local expert routing over peer networking; explicitly lacks built-in verification and incentives. It motivates optional client composition, not trusted expert claims or a ready commercial network. |
| [Decoupled DiLoCo, April 2026](https://arxiv.org/abs/2604.21428) | Studies resilient decoupled distributed pre-training, including simulated failure conditions. Motivation for later checkpoint/training studies; not evidence arbitrary interactive workloads can migrate profitably. |
| [Perishable-utility market theory, v2 August 2026](https://arxiv.org/abs/2511.16357) | Matching/equilibrium results depend on the model, including resumable, reproducible, verifiable work within performance tiers and specified strategic assumptions. They are not universal matching, truthful-collusion resistance or adoption results for heterogeneous service contracts. |
| [PoUW economics, June 2026](https://arxiv.org/abs/2606.06700) | Conditional competitive-equilibrium analysis relates token adoption, useful-work markets and overheads. Adoption is an assumption/parameter, not established customer demand; it does not select native issuance here. |
| [Usefulness-gap preprint, June 2026](https://arxiv.org/abs/2606.04819) | Authors report an empirical study of Pearl's cuPOW protocol. Use as motivation to test accepted external usefulness and gaming; neither independent reproduction here nor a result about all PoUW. |
| [Verde](https://www.gensyn.ai/research/verde-a-verification-system-for-machine-learning-over-untrusted-nodes) | Reproducible operators and dispute localization under verifier assumptions motivate execution verification. Such verification does not establish semantic quality, confidentiality or independent noncolluding verifiers. |
| [RFC 9334](https://www.rfc-editor.org/rfc/rfc9334.html) | Separates attestation evidence, verifier appraisal and relying-party decisions. It is an architecture, not proof that the current Evidence struct authenticates anything. |
| [TEE.fail](https://tee.fail/) | Physical memory-bus attack research motivates exact platform and physical-adversary scope. Do not generalize its attacks to every TEE or assume collateral reverses a privacy breach. |
| [Carbon–SLO preprint, 10 October 2026](https://arxiv.org/abs/2610.14571) | Models batching/queueing and power under SLOs. End-to-end validation is limited to Llama-3.1-8B within one node, with A100-based profiles; no general hardware/WAN accuracy or carbon gains transfer to this network. |
| [SimGrid](https://simgrid.org/) | General distributed-platform simulation is existing prior art. A future backend's abstractions and accuracy still need workload-specific qualification. |
| [Vidur](https://www.microsoft.com/en-us/research/publication/vidur-a-large-scale-simulation-framework-for-llm-inference/) | Combines operator profiling and prediction for inference simulation. Calibration and model coverage constrain any future reuse. |
| [ASTRA-sim](https://astra-sim.github.io/astra-sim-docs/index.html) | Distributed ML system simulation is a specialist option, not a selected adapter or validation of our service accounting. |
| [SimAI](https://github.com/aliyun/SimAI) | Framework/collective/network simulation motivates a specialist integration boundary. No code, installation, source licence change or performance claim is imported. |

## Missing logic and decision gates

[FIXES](FIXES.md#network-design-audit) audits every current source module and the
30 regression specifications. N01–N11 add distinct network obligations while
B03–B09 retain ownership of contracts, credits, economics, key release, recovery,
funded sites and hardware appraisal. Those gaps remain unsolved.

The [roadmap](ROADMAP.md) sequences identity and contract semantics, atomic
reservations/outbox and crash/replay evidence before provider/payment effects;
independent simulator conformance and calibration before policy comparisons; and
service-class qualification and subsidy-free margin evidence before adoption
claims. No roadmap entry authorizes dispatch, spending, deployment or product tests
outside the owner's qualified fleet. This design supplies no production acceptance.
