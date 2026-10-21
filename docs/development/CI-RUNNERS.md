# Owner fleet qualification

The manual workflow offers exactly two routes, with no fallback:

| Input route | Runtime requirements | Scope |
| --- | --- | --- |
| naevum-vps-amd64 (default) | self-hosted; Linux; X64; x86_64 GNU Rust host | apocryph-compute-20261021 |
| ci-linux-arm64 | self-hosted; Linux; ARM64; aarch64 GNU Rust host | apocryph-compute-20261021 only |

## Measured scope and installation compatibility

On 2026-10-22 the coordinator reported qualification of the dedicated ARM64 image
as UID 1001 on aarch64 Linux, with the official standalone Rust/Cargo 1.99.1,
cc 13.3, Python 3.12.3 and Node 24.19.0. Only `ci-linux-arm64` is configured as a
label on the installed target scale set. Multiple configured ARC labels are
possible, but `self-hosted` and `Linux` are not declared on this installation.
Use `runs-on: ${{ inputs.route }}`; retain runtime trust checks. The default amd64
route remains unchanged and has no new qualification receipt from this order.
An ARM64 result cannot qualify amd64. Queued, invalid or skipped jobs never pass
and must never trigger fallback or global fleet changes.

The original source `72a210049f8254fce5a97191854046d56c5bc43a` passed offline
cargo tests/build and CLI fixture/restart/replay according to the coordinator's
external observation. See the [dated evidence note](../EXPERIMENTS.md#external-qualification-note-2026-10-22).
This worker did not execute or independently inspect private receipts. That result
does not validate these new helpers or workflow. The malformed draft workflow was
disabled by the operator; re-enablement after review is an external operator step.

## Manual review and runtime gates

`workflow_dispatch` is the only trigger. `judge_surfaces_reviewed` defaults to
false. Only the coordinator may set it true after independent review of the exact
candidate, tests, expected fixtures, CI helpers and workflow. The job runs only
when that input is true and the campaign is `apocryph-compute-20261021`. This
attestation is a manual gate, not automatic verification of a review receipt.
The selected ref at dispatch must resolve to the independently reviewed candidate.
Never automatically run an unreviewed public pull request on private runners.
Contributor offline feedback is welcome under [CONTRIBUTING](../../CONTRIBUTING.md)
and requires no private infrastructure or accounts.

No push/PR/schedule trigger, toolchain installation, artifact upload or hosted
runner is configured. Checkout retains the immutable action
`11d5960a326750d5838078e36cf38b85af677262`; this work did not remotely re-resolve it.
Persisted credentials are disabled. Permissions are contents:read.

Input/context strings enter scripts through environment variables, never shell
program interpolation. GitHub does not allow `runner` at `jobs.<job_id>.env`:
see its [context availability table](https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#context-availability).
The coordinator observed an HTTP422 rejection of the same malformed pattern
before execution. This workflow now reads the default `RUNNER_ENVIRONMENT`,
`RUNNER_OS` and `RUNNER_ARCH` variables at runtime; job-level env has no runner
expressions. `ci_preflight.py` rejects an unreviewed input, unsupported campaign or
route, hosted runner, reported or actual OS/architecture mismatch, dirty tree,
or HEAD different from `github.sha`. Runner metadata alone is insufficient: actual
Linux and the selected machine architecture must agree.

The only allowed toolchain input is `1.99.1`. Preflight resolves installed `rustc`
and `cargo` paths, rejects rustup proxies and checks each `-Vv` release and GNU
Linux host against that pin and selected route. No rustup installation, toolchain
alias, `+toolchain` invocation or download occurs. The qualified image and PATH
remain trusted provisioning boundaries; version output is not binary attestation.
The coordinator must qualify image identity, isolation, offline dependencies and
filesystem semantics separately. No image digest or fleet configuration is
invented or changed here.

After preflight and contract --check, `ci_product.py` repeats preflight, pins RUSTC
to the verified compiler and executes the verified absolute cargo path for
`test --offline` and `build --offline`, with CARGO_NET_OFFLINE=true. It then runs
the unchanged `commerce_smoke.py`: a fresh synthetic journal, exact expected
summaries, second-process replay with unchanged bytes and malformed-input rejection.
The smoke uses RUNNER_TEMP and cleans only its own temporary directory. These
commands must not execute in this worker; documentation compilation and static
syntax/structure inspection are separately permitted.

## Remaining acceptance and deployment limits

This maintenance candidate is NOT_RUN. Independent review must check a positive
case (reviewed correct campaign, clean exact SHA, matching actual host and both
1.99.1 versions) and negative cases (false review or wrong campaign skips; wrong
route, host, dirty SHA, rustc/cargo release or host rejects). These are required
future witnesses, not executed helper tests. A skipped workflow is never a pass.
A new actual manual CI run must capture the exact source, checkout/action identity,
commands, versions, qualified environment identity, raw results, failures,
expected/observed outputs and independent reviewer/oracle. Existing original-source
qualification cannot stand in for that run or grant checker acceptance.

No deployment step or production acceptance is included. A whitespace check,
workflow definition, injected fault sink or bounded fixture pass cannot qualify
physical power loss, payment finality, a marketplace or deployment readiness.
