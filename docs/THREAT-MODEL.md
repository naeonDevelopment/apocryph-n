# Apocryphⁿ threat model

Campaign scope note (2026-10-21): [COMMERCE](COMMERCE.md) now owns the separate
bounded typed-commerce API, local journal, permanent accounting credits and offline
wire contract (C01–C08, IMPLEMENTED_UNVERIFIED). Descriptions below of absent typed
lineage, credit or durable-log mechanisms describe the preserved legacy baseline;
external authentication, legal finality and distributed recovery remain absent.
The original NOT_RUN record is historical; see the [2026-10-22 external
qualification note](EXPERIMENTS.md#external-qualification-note-2026-10-22) for exact-source
evidence. This maintenance candidate remains NOT_RUN; broader claims are unchanged.

The protected objects in this reference are local arithmetic invariants and
explicit advisory decisions. There is no protected running workload, wallet,
secret store, network channel or durable database. Inputs are untrusted caller
assertions. Correctness of Rust source itself is unverified until qualified tests
and builds run. The [register](../registers/IMPROVEMENTS.csv) owns change status.

| Actor or failure | Reference boundary | Residual exposure / required integration |
| --- | --- | --- |
| Dishonest provider changes price, capacity or identity | Exact billing-unit/profile/region/epoch checks and deterministic choice | All fields forgeable; signed offers, authenticated identity, contract terms and capacity reservation absent |
| Receipt producer duplicates, reorders, alters or omits intervals | Stable IDs, exact replay, conflict/overlap rejection, time ordering and complete coverage checks | Receipt content is unauthenticated; attacker may invent consistent data; no durable ingestion or cross-meter identity |
| Caller retries after timeout or repeats terminal transitions | Unknown retains funds; same-ID/amount and same-state transitions are no-ops | Caller can lie about reconciliation; local success never grants another transfer; no multi-process exclusion |
| Crash after transfer submission but before recording result | No external submission exists in this library | Memory loss erases identity/balances; future durable journal/outbox and provider reconciliation must preserve identity across restart |
| Chain reorg or contradictory finality receipts | No chain reader or finality model | Must quarantine ambiguity, authenticate receipts, define finality and reconcile canonical state; Refunded is not a reorg algorithm |
| Replay of old or other-session evidence | Positive maximum age, checked elapsed subtraction, exclusive expiry and exact nonzero challenge match | Trusted clock and challenge freshness assumed; all evidence can be forged; no signature, revocation, nonce store or workload digest binding |
| Wrong CPU/GPU or assurance profile | Exact enum matching; no profile promotion | Trusted records do not create TEE claims; confidential enum does not establish device/memory/interconnect protection |
| Malicious host, firmware, accelerator or verifier | No mitigation implemented | Hardware-specific appraisal, attested channels, workload binding, revocation and independent key-release policy required |
| Tenant-to-tenant replica access | No replica API exists | Least-privilege credentials, tenant/resource/action scoping, rotation and revocation still need implementation |
| Depletion, key loss, replica corruption or site loss | Policy recommends Retain at zero funds | No storage funding, key escrow, backup, replication or restore verification; recommendations cannot prevent loss |
| Huge hostile input or clock manipulation | Checked arithmetic rejects overflow | Unbounded collections and sorting can exhaust resources; caller must bound sizes and authenticate time; no availability guarantee |

## Settlement and accounting trust

One mutable ledger models one caller-scoped currency/billing context. Independent
ledgers can double-count externally. Reconciliation is a boolean assertion; it
must never be wired to “request sent” or a transaction hash as proof of finality.
An external adapter would need a stable authorized operation identity, durable
state before submission, authenticated final outcome and a policy for uncertain
or reordered receipts. Such an adapter is absent. There is no permission or
mechanism to retry external effects in this stage.

Refunded shifts confirmed_paid into disputed. Explicit release makes that amount
collectible again. Permanent invoice credits, partial refunds, tax handling and
legal credit notes are absent; local state labels confer no legal authority.

## Recovery and confidentiality research obligations

A future recovery design must specify independent failure domains, verified
replica manifests, completion receipts and restore tests. Measure process crash,
whole-site loss, corrupt replica, lost client key, lost operator key and compromised
recovery authority separately. Independent key release needs a declared quorum,
revocation policy and evidence-to-workload/key binding; backup copies alone do not
solve key loss. Define recovery point and time objectives and data that is allowed
to remain unrecoverable. No funded sites or recovery guarantees exist here.

Side channels, traffic analysis, access patterns, compromised clients, denial of
service and coercion are not solved by the reference. Future platform-specific
threat review must state which are covered, which are excluded, and how exclusions
affect users. Universal privacy is not a supported claim. See [RESEARCH](RESEARCH.md)
for supplied references and [EXPERIMENTS](EXPERIMENTS.md) for planned falsifiers.
