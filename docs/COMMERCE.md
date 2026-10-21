# Bounded typed commerce reference

Historical authoring status: **IMPLEMENTED_UNVERIFIED**, product execution and
independent review **NOT_RUN**, campaign `apocryph-compute-20261021`, order
`353c85212ea6`. The [2026-10-22 external qualification note](EXPERIMENTS.md#external-qualification-note-2026-10-22)
records coordinator-observed results for exact source `72a210049f8254fce5a97191854046d56c5bc43a`.
The current compatibility/documentation maintenance preserves these semantics and
witnesses; its changed CI helpers require new independent review and execution.
Restored own candidate `a942d815ca441f48f988754cc39b55fe951d3885` with `git archive`.
This is new original Apache-2.0 source, not a retry of order 38eb5bec0b67.
Legacy admission/metering/settlement/policy APIs and 30 regression witnesses remain
unchanged. `commerce`, `commerce_wire`, `commerce_store` and the offline `commerce`
binary are a separate reference. No upstream or sibling source was imported.

## Model and invariants

P01/P02/P03/P05/P06/P08/P09/P10 apply. P04 allocation/governance and P07 empirical
abstention calibration are not implemented by this increment.

An asset is the exact tuple (namespace, network, issuer-or-identifier, decimal
scale). The scale is metadata, not a multiplication factor or exchange rate.
`Amount` uses checked u128 minor units; addition/subtraction require exact identity.
`ServiceUnit` is an exact nominal label for one capacity-quantum tick of accepted
service, for example `slot-tick`. No token issuance, custody, wallet, provider or
network actions exist. Asset and service strings remain unauthenticated labels.

An offer pins provider, provider epoch, version, resource pool, capacity, window,
expiry and rate. Epochs increase monotonically; zero is invalid. Offer versions
increase within an epoch. Only the newest published offer for a provider/resource
can admit a new reservation. Old committed reservations retain the original rate;
a provider epoch increase fences new attempts and acceptances on old reservations.
Closing, expiring and financial reconciliation remain possible across that fence.
A pool's service unit cannot change; use a new resource identity for a different
unit. Published capacity changes must cover all outstanding active reservations,
including reservations from old epochs. Capacity cannot be created by changing an
offer ID. Distinct resource IDs denote independent caller-declared physical pools;
this code cannot detect two IDs describing the same machine.

Reservation payloads pin offer/provider/epoch/version/resource/unit, account,
quantity, half-open [start,end), expiry and maximum typed money exposure. Start is
not in the past. Expiry is at least end and no later than the offer window end;
acceptance requires now < expiry, so choose expiry > end for end-of-window receipts.
Offer expiry is an admission deadline, not an automatic cancellation of existing
contracts. At each active reservation start, summed overlapping quantities must
fit the current pool capacity. Adjacent windows may reuse capacity. Reservations
continue holding capacity and budget until an explicit close or expire command;
clock advancement alone does not delete obligations. Close may occur early and
prevents further accepted work; it is an asserted release, not proof a worker stopped.
Expire requires now >= expiry. There is no distributed scheduler or physical fence.

Admission atomically reserves account budget and capacity. The maximum must cover
floor(quantity * duration * price / per_units). Overflow rejects even if a reduced
rational calculation could fit. The account has an immutable asset and cumulative
exposure limit, not a bank balance or funded deposit. Active reservations consume
their full maximum; closed reservations consume gross invoice minus permanent
credits, including paid, disputed, collectible and unknown-reserved amounts.
Credits do not free the unused maximum of an active reservation. There are no
account top-ups, rolling budget periods, asset conversions or shared custody.

One invoice per reservation is created by the first attempt; subsequent attempts
must bind the same invoice. Invoice provenance copies the original offer and rate.
Acceptance binds attempt, reservation, invoice, service unit and an external
evidence ID. Receipt identity replay must be identical. Evidence IDs cannot be
rebound to another receipt/credit/payment/refund. A cumulative quantity is measured
from the reservation origin across ALL attempts, never separately per retry.
It is bounded by quantity * elapsed ticks and never decreases. The invoice gross
is floor(cumulative * price / per_units); only the resulting cumulative total is
stored, so receipt partitions cannot add extra rounding or double bill. Accepting
an equal cumulative total under a distinct evidence ID adds zero entitlement.
This is a reported cumulative accepted-work counter, not sample integration or
proof of gap-free telemetry. The adapter must supply this convention correctly.
Assertions establish neither useful work nor authenticated customer acceptance.

`Commerce::apply` validates canonical input, recognizes exact command replay,
clones state, executes and audits on the clone, then commits. Every error preserves
all old maps, the logical clock and evidence-use registry. The global caller clock
is monotone for new commands; exact historic command replay is allowed. Commands
and domain identities are retained forever within the bounded journal. Reusing a
command ID with a different timestamp or payload fails; domain-ID replay under a
new command ID must still have the identical original domain payload. No deletion
or identity rotation resets uniqueness. Successful domain lookup/replay grants no
new external effect permission.

## Collection, credits and conservation

Collection creates one stable ID with an immutable invoice/asset/amount payload
and reserves only collectible value. The states are READY -> UNKNOWN -> PAID or
FAILED. The explicit `submit` command is a *local durable claim*, not a payment.
Only a new successful durable claim is a candidate handoff to a future adapter.
Exact command replay returns `Replay`, never a second handoff. A second submit
with a new command ID fails. No adapter is included, and the CLI performs no handoff.
A crash before a hypothetical send still leaves UNKNOWN. Unknown retains reserved
value across reservation closure/expiry. There is no timeout retry. Reconciliation
requires a nonempty external evidence ID bound to this intent and one terminal
outcome. Only identical terminal evidence/outcome replays; conflicts reject.
A terminal FAILED intent may release value for a new explicitly named collection;
it never resets the original ID to READY. This trusts the failure assertion and
is NOT an exactly-once payment guarantee.

Unpaid permanent credit can consume only collectible value. Unknown-reserved and
disputed value must first be explicitly resolved/released. Paid credit consumes
retained paid allocation and creates a refund liability. A refund assertion binds
a separate evidence ID to that paid credit, moves refund_due to refunded, and can
never restore collectible value. Partial refunds of a credit are unsupported;
issue smaller separate credits. Credit IDs and evidence are immutable. These are
accounting reference notes, not jurisdiction-qualified legal instruments.

For EACH invoice and its ONE asset, `Invoice::audit` exposes:

```
gross = collectible + reserved + disputed + paid + credited
refund_due + refunded <= credited
cash_received = paid + refund_due + refunded
net_cash_retained = cash_received - refunded = paid + refund_due
```

`credited` includes both unpaid reductions and all paid credit notes, whether or
not their refund is asserted complete. `paid` is retained uncredited payment
allocation, not cumulative external receipts. `cash_received` is derived from
asserted payments and includes cash later refunded. `refund_due` is the modeled
cash liability. Disputes quarantine collectible debt; release restores only that
quarantined debt. Credits never enter dispute storage, so dispute release cannot
resurrect a credit. For example gross 20, paid credit/refund 7 yields paid 13,
credited 7, refunded 7, collectible 0, cash_received 20. Closing the reservation
leaves account exposure 13. No subtraction treats credit as new cash or revenue.

This subset does not model customer deposits, prepaid service liabilities,
provider payables, collateral, fees, tax, exchange, bad-debt expense, legal finality,
chargebacks/reorgs, contributor claims, rewards or equity. It is not a full
double-entry general ledger or proof of funded liabilities. Unknown external
money outcomes remain held; refund evidence is trusted, not rail verification.

## Durable journal and recovery

`FileStore` owns a cooperating single writer in a trusted local directory. It
canonicalizes the parent, rejects a symlink/non-file log and (on Unix) multiply
linked files, and creates a sibling
`<filename>.lock` directory atomically before opening the log. Directory syncing
and file syncing are required; failures reject. No stale-PID heuristic or automatic
lock deletion exists. Clean explicit `close()` synchronizes then removes the lock;
Drop, panic, crash, open/replay failure or ambiguous write leaves it. Lock removal
need not survive a crash: a reappearing stale lock fails closed. This is a local
filesystem assumption, not a cross-host lease, consensus service or hostile-user
security boundary. Keep the directory private, prohibit hardlinks/renaming or
outside edits, and use filesystems with qualified create-directory and fsync
semantics. Do not share the directory through NFS or object-store mounts.

Every new validated deterministic command is framed, appended and `sync_all`'d
before success and before publishing in-memory state. Replay validates the exact
same commands and invariants. Exact duplicate commands are not appended again.
The public `AppendSync` trait permits deterministic fault injection; a custom sink
is trusted to implement append AND sync, and `Journal::recover` requires that
sink to correspond exactly to the supplied bytes under exclusive ownership.
It does not independently establish those facts. Use FileStore for real local I/O.

The log consists of adjacent records, with no magic inferred outside this frame:

```
J1<TAB>sequence<TAB>payload_byte_length<TAB>checksum<LF>
canonical_C1_payload<LF>
```

Sequence starts at 1 and is contiguous. Length and sequence are canonical unsigned
decimal. Checksum is 16 lowercase hex digits of wrapping 64-bit FNV-1a over payload
bytes (offset 0xcbf29ce484222325, prime 0x100000001b3). This is accidental-corruption
detection, NOT cryptographic tamper evidence. Replay compares the entire canonical
frame, including sequence, length, checksum and delimiters. Unsupported versions,
invalid operations, altered duplicates, malformed tokens, truncated records,
trailing garbage, oversize input and failed state invariants reject the entire
open. No salvage or prefix fallback is automatic. Deletion of a whole valid suffix
or replacement with a different valid history is not detectable without a trusted
external high-water manifest; checksum collisions are possible. No claim otherwise.

Any append or sync error poisons that Journal even if zero bytes appear written.
No further apply, including retry of the same ID, is allowed. A complete frame may
have reached disk despite the error; a short frame may be corrupt. Never infer
which outcome from the error. The old in-memory snapshot is not an authority for
recovery. FileStore retains its lock and refuses clean close after poisoning.

Explicit recovery is an operator/coordinator procedure, not an automatic CLI mode:

1. Fence and prove termination of every previous writer, prevent new writers and
   preserve the lock, full journal bytes and failure receipts in an offline backup.
2. Independently inspect the exact bytes and compare the last externally recorded
   command/sequence/source identity. Full valid replay may include the uncertain
   command; partial/corrupt history requires an explicitly reviewed recovery
   decision. Never silently truncate an unknown record or rotate business IDs.
3. Reconcile any external collection/refund outcomes against the SAME stable IDs.
   This reference has no rail and cannot establish such evidence itself. Record
   the recovery decision/evidence in the coordinator's audit record.
4. Only after those checks, remove the stale lock under exclusive administrative
   control and explicitly reopen the reviewed journal. Replay will either accept
   the full validated history or fail closed again. Do not automatically retry
   the poisoned command. No recovery procedure establishes exactly-once effects.

Current fixed bounds: 64-byte ASCII identifiers; asset scale 0..38; u64 nonnegative
caller ticks/epochs/versions/capacities; maximum positive interval 31,536,000 ticks;
u128 minor money and cumulative service; 8,192-byte command; 100,000 distinct
commands; 64 MiB journal. Exhaustion rejects without eviction or compaction. The
clone-per-command and interval scan can be expensive well below these input caps;
these are rejection limits, not performance/SLO claims. No snapshot/migration,
multi-writer, replication, backup authentication, confidentiality or adversarial
filesystem protection is implemented.

## Generic offline wire contract

No JSON parser or dependencies are used. The deliberately small TSV envelope is
not JSON and must not be advertised as JSON-lines. Each input record is exactly
one canonical C1 command followed by LF. Blank lines, comments, CRLF, escaping,
quoted fields, extra/missing columns and an unterminated final line reject. Tokens
are 1..64 ASCII bytes in `[A-Za-z0-9._:-]`. Numeric fields are unsigned base-10 with
no sign, spaces, exponent or leading zeros except `0`. IDs have no hidden meaning.
The command version is literal `C1`; time is caller-supplied integer ticks using
one consistent tick duration per journal. There are no implicit defaults.

Every row begins `C1 command_id now operation` (shown with spaces here; actual
separators MUST be TAB). Subsequent fields in order:

| Operation | Remaining fields |
| --- | --- |
| epoch | provider epoch |
| account | account ASSET minor_limit |
| offer | offer provider epoch version resource capacity starts ends expires ASSET price_minor service_unit per_units |
| reserve | reservation account offer provider epoch version resource service_unit quantity start end expires ASSET maximum_minor |
| close / expire | reservation |
| attempt | attempt reservation invoice |
| accept | receipt attempt reservation invoice external_evidence service_unit cumulative_units |
| collect | intent invoice ASSET amount_minor |
| submit | intent |
| reconcile | intent external_evidence outcome (`paid` or `failed`) |
| credit | credit invoice ASSET amount_minor source (`paid` or `unpaid`) external_evidence |
| refund | credit external_evidence |
| dispute / release-dispute | invoice ASSET amount_minor |

`ASSET` expands to FOUR fields: namespace network issuer-or-identifier scale.
A completed-service simulator bridge should emit setup (epoch/account/offer),
reserve, attempt and accept rows with cumulative service from the reservation
origin; multiple attempts must share the same origin and invoice. A simulator may
then exercise local outbox/credit assertions using synthetic evidence. It must
not label these as authenticated payments or actual business receipts. There are
no sibling reads, provider calls, payment integrations or simulator-specific keys.

On qualified fleet only, `target/debug/commerce JOURNAL < fixtures/commerce.tsv`
executes real durable library commands. Earlier commands survive a later bad input
line; input batches are not atomic. Re-running the exact file returns replay
acknowledgments without new append or submission. Errors go to stderr, exit 1;
output loss does not undo already synchronized commands. Exit 0 includes explicit
clean store close. Success is a local command acknowledgment, never proof of an
external transfer. An open/poisoned store requires recovery above.

Stdout uses `S1` TSV. Each command acknowledgment is
`S1 command command_id new|replay`. At successful EOF, rows are sorted by ID within
invoice, credit and outbox groups, with these exact columns:

| Kind | Fields following `S1 kind` |
| --- | --- |
| invoice | invoice reservation namespace network identifier scale service_unit gross collectible reserved paid credited disputed refund_due refunded cash_received cumulative_units offer |
| credit | credit invoice amount_minor paid-or-unpaid external_credit_evidence |
| outbox | intent invoice amount_minor ready-or-unknown-or-paid-or-failed evidence-or-`none` |

Credit/outbox asset and unit provenance is inherited from the referenced invoice.
`none` is interpreted as absent only for a nonterminal outbox; a terminal evidence
ID may itself equal `none`. These summaries are exports, not replay input; retain
the journal for full command/offer/attempt/evidence lineage.

The synthetic fixture hand-derives ten accepted slot-ticks * 2 minor/slot-tick = 20,
collects 20 by assertion, permanently credits/refunds 7 and closes exposure at 13.
`fixtures/commerce.expected.tsv` is the exact expected summary; `commerce_smoke.py`
checks a second process replay leaves journal bytes unchanged and malformed input
fails. At original authoring the expected fixture had not been independently
reviewed or run; the dated qualification note records the later coordinator
observation without changing fixture bytes or promoting the new CI helpers.
External bridge preparation belongs to the coordinator; this worker inspected no
other simulator or private integration.

## Qualification and remaining gaps

`tests/commerce.rs` adds bounded positive and adversarial witnesses: identity and
dimension checks, integer overflow, capacity overlap and adjacency, budget rollback,
epoch/version fencing, close/expiry, cumulative retry/rounding, duplicate evidence,
unknown and terminal reconciliation, permanent paid/unpaid credits and disputes,
restart replay, partial append/lost sync, framing corruption and stale writer lock.
The synthetic fault sink tests logical boundaries; it cannot reproduce physical
power loss or validate a filesystem's fsync implementation. E18 in EXPERIMENTS.md
requires separate reviewer/oracle and qualified source-bound receipts.

Remaining gaps include authenticated acceptance/meters/offers, physical resource
identity/fencing, Byzantine actors, legal credit instruments, real payment and
refund finality, distributed consensus/recovery, privacy, hostile filesystem
security, pricing/market demand and cost calibration. No performance, useful-work,
profitability, hardware security or production-readiness claim follows from this
source. Independent coordinator source/checker review and qualified manual fleet
CI are permitted only by the narrowly amended campaign contract. Worker product
execution, self-approval, deployment, remote effects and fleet changes remain
prohibited. Vagus commits; the worker leaves only reviewable edits.

### Worker handoff evidence (documentation/structure only)

On 2026-10-21, `python3 scripts/agent_sync.py` and its `--check` mode completed.
Python AST parsing checked syntax of the scripts without invoking CI/product
code. CSV checks confirmed unique register IDs/column structure and eight C rows
at IMPLEMENTED_UNVERIFIED / NOT_RUN. Static fixture checks confirmed input column
counts and bounded token alphabet; they did not run commerce semantics. Source,
test, script, documentation, register, fixture and workflow whitespace scans were
clean, as was `git diff --check` (the empty-main worktree is untracked until Vagus
commits, so the explicit file scan covers what Git's tracked diff does not).

An own-object `git archive` stream comparison confirmed unchanged bytes for the
four legacy modules, tests/regression.rs (30 witnesses), scripts/agent_sync.py,
LICENSE, COPYRIGHT, AUTHORS.md and ORIGINS.md against the restored candidate.
Direct `git show commit:path` comparison was denied by the managed filesystem;
no protected ancestor was read, and the explicitly authorized archive interface
supplied the bounded comparison. There are 14 new commerce test specifications.
No Rust compilation, product tests, CLI smoke, CI, fleet query, external effect,
commit, push, PR, merge or deployment was performed. These structure receipts do
not change implementation or experiment status and do not constitute independent
source/checker approval. Vagus owns the resulting commit and the coordinator owns
qualified execution and acceptance `apocryph-n.research-whitespace`.
