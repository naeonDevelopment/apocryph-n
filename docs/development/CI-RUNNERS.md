# Owner fleet qualification

The manual workflow offers exactly two routes, with no fallback:

| Input route | Runtime requirements | Scope |
| --- | --- | --- |
| naevum-vps-amd64 (default) | self-hosted; Linux; X64; x86_64 GNU Rust host | apocryph-compute-20261021 |
| ci-linux-arm64 | self-hosted; Linux; ARM64; aarch64 GNU Rust host | apocryph-compute-20261021 only |

Both route eligibility and qualified enrollment remain **UNMEASURED**. An input
label is not evidence of enrollment. The coordinator handles image, exact installed
Rust release, Python, isolation, filesystem semantics, offline dependencies,
repository access and enrollment qualification separately. No runner queries or
fleet changes were performed here. A queued/invalid route must not fall back.

`workflow_dispatch` is the only trigger. No push/PR/schedule trigger, toolchain
installation, artifact upload or hosted runner exists. Checkout retains the supplied
immutable action `11d5960a326750d5838078e36cf38b85af677262`; this work did not remotely
re-resolve it. Persisted credentials are disabled. Permissions are contents:read.
The selected ref at dispatch must be the coordinator-reviewed exact candidate.

All workflow input/context strings enter scripts through environment variables;
none is interpolated into shell program text. Route labels are consumed as Actions
scheduling data. `ci_preflight.py` rejects any other route/campaign, hosted runner,
OS/architecture mismatch, dirty tree or HEAD different from `github.sha`. It logs
revision, campaign, route, runner identity, platform and rustc/cargo verbose versions.
The required toolchain input is an exact installed numeric release, never stable,
nightly or an arbitrary option. Preflight verifies release and GNU Linux host.
`rustup run` receives an argument vector and no --install flag; auto-install is
disabled. The caller must provision the chosen toolchain before dispatch. There is
no implicit minimum-version promise; coordinator qualification must cover the
standard library APIs used by the candidate (including io::Error::other).

After preflight and contract --check, `ci_product.py` repeats preflight and executes
`cargo test --offline`, `cargo build --offline` under that exact installed toolchain,
then `commerce_smoke.py`. The latter invokes the built offline CLI against a fresh
synthetic journal twice, compares the hand-authored expected summaries, checks
journal replay adds no bytes and tests malformed input. It uses RUNNER_TEMP and
cleans only its own temporary fixture directory. These commands are product
execution and must never run in this worker. The documentation compiler may run
locally under the canonical contract's explicit exception.

CI WAS NOT RUN. No toolchain/image/runner qualification or independent review receipt
exists in this order. Future receipts must include source commit, checkout/action
identity, exact commands and versions, qualified environment identity, raw results,
failures, expected/observed outputs and separate reviewer/oracle. A source check,
workflow definition, fake sink or whitespace pass cannot qualify filesystem power
loss, payment finality, a marketplace or a deployment. No deployment step exists.
