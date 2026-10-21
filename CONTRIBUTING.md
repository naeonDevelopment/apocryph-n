# Contributing to Apocryphⁿ

This public Apache-2.0 foundation builds without proprietary packages, service
credentials, private infrastructure or a contributor account on the owner's fleet.
Read [README](README.md), [commerce semantics](docs/COMMERCE.md) and the
[canonical agent contract](agent/contract.md) before changing behavior.

## Standalone offline feedback

From your own checkout, with Rust/Cargo already installed:

```sh
rustc -Vv
cargo -Vv
cargo test --offline
cargo build --offline
python3 scripts/agent_sync.py --check
git diff --check
```

There are no Cargo dependencies. The coordinator-observed reference environment
is Linux ARM64 with standalone Rust/Cargo 1.99.1; this is not a minimum-version or
cross-platform support promise. The tests use synthetic data; the file-store test
creates and removes its own temporary directory. Use a local filesystem that
supports directory synchronization. Do not use production journals or credentials.
Do not invoke `ci_product.py` locally: it is an owner-fleet entrypoint with runtime
trust checks. Independent contributors may run the offline commands above on their
own systems and report feedback; this agent's product execution and release
acceptance still require qualified owner-fleet receipts.

## A useful first change

Extend `typed_assets_units_scale_and_overflow` in `tests/commerce.rs` with an
explicit zero-`per_units` rejection witness and a nearby valid division case.
Derive the expected result by hand from [COMMERCE](docs/COMMERCE.md), preserve
existing APIs, and explain the invariant in the proposed change. Small corrections
to wire-format examples or documentation links are also useful.

For behavior changes, name the applicable [P01–P10 principles](docs/SCIENTIFIC-PRINCIPLES.md),
invariant, assumptions and supported domain. Include a meaningful positive witness
and a negative or boundary witness that would fail if the invariant were broken.
Use only original code and synthetic fixtures; include expected versus observed
results, exact commands, source revision and toolchain/host. Mark tests not executed
as NOT_RUN. Update the relevant [register](registers/IMPROVEMENTS.csv) entry and
[fix explanation](docs/FIXES.md), preserving historical evidence.

Compatibility review must cover public APIs, C1/S1 wire spelling, replay identity,
accounting conservation and local journal recovery assumptions. Distinguish source
presence, contributor feedback, qualified measurements and research proposals.
Changes to the canonical agent contract require regenerating AGENTS.md/CLAUDE.md
with `python3 scripts/agent_sync.py` and checking the result.

Submit a focused public pull request with the rationale and checks. Public pull
requests never automatically run on private runners. A coordinator must independently
review the exact source and checker before requesting a manual qualified-fleet run;
a skipped job is not a pass. Contributors need no access to that infrastructure.
For vulnerabilities, follow [SECURITY](SECURITY.md) rather than publishing details
in an issue or pull request. Contributions create no governance, payment or token
entitlement; no separate CLA is introduced here.
