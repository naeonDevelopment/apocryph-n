# Apocryphⁿ research boundaries

This dependency-free in-memory Rust reference studies whether explicit units,
coverage, intent identity and admission constraints make incorrect local decisions
observable. It is not a new cryptographic protocol, deployed marketplace or
scientific demonstration. Source-review reasoning is provisional; experiments
are all NOT_RUN. [The register](../registers/IMPROVEMENTS.csv) owns change status.

## Questions, baselines and falsifiers

| Question | Strong baseline/control | Falsifier or missing evidence |
| --- | --- | --- |
| Does metering preserve dimensions and cumulative rounding? | Independent exact rational calculation of byte-seconds divided by declared byte-unit seconds; fixed-origin aggregate versus partitioned receipts | A complete equivalent receipt set changes cumulative charge with arrival order/partition, or an unknown gap bills as known zero |
| Does intent identity prevent local duplicate collection? | Single-owner reference accounting model with accrued/invoiced/reserved/paid/disputed conservation | Duplicate ID or terminal replay changes balances; Unknown permits a competing reservation; failed mutation changes state |
| Does explicit admission reject incompatible offers? | Exhaustive table of all six profiles and independent eligibility oracle, then price/ID ordering among eligible entries | Wrong currency/quantum, profile, epoch, region, stale/future evidence or mismatched challenge wins |
| Does confidential execution protect a specified workload? | Matched ordinary CPU/GPU execution and hardware-qualified confidential configuration, with an independent verifier and adversarial controls | Forged/replayed evidence accepted, plaintext/key exposed outside declared boundary, or unresolved device/driver/firmware coverage; no such apparatus exists here |
| Is the product viable and recoverable? | Ordinary centralized service with comparable SLO, documented restore procedure and full cost accounting | Cost exceeds achievable price under plausible utilization, restore fails under site/key loss, or settlement cannot reconcile uncertain outcomes |

Integer correctness is separable from confidentiality, operational availability and
business viability. Local unit tests can address the first three questions only
within their small model. They cannot establish the last two. The model has no
cryptographic attestation, runtime isolation, authenticated offers or independent
key release. A TrustedGpu/TrustedCpu record is not a TEE measurement or claim.

## Measurement design

Meter units are bytes and seconds, with DecimalGb = 10^9 and BinaryGib = 2^30.
Price and budget are integer token minor units per explicit service quantum;
`minor_units_per_token` is metadata, not another multiplier. Floor once at the
cumulative query boundary. Separately rounded invoices are not expected to sum to
the same value. Include zero-price known coverage, missing coverage, overflow and
one-minor-unit thresholds as controls. Report absolute rounding loss and the
chosen invoice window; no floating approximation is an oracle.

Future performance studies must pair identical workload, hardware, firmware,
driver, dataset, batch size and allocation policy. Report cold/warm startup,
attestation latency, p50/p95/p99 latency, throughput, peak memory, GPU utilization,
energy and egress, repetitions and uncertainty. Keep security failures separate
from timing measurements. No zero-overhead, universal-privacy or demonstrated
novelty claim follows from a label or successful arithmetic fixture.

Provider economics should compare net receipts against amortized equipment,
energy, network/egress, storage replicas, idle capacity, verification, payment
fees, support and expected failure/refund costs. Vary utilization, price, outage
rate and recovery cost; report break-even assumptions and downside cases. A
price-minimum selector neither measures provider margin nor proves revenue.

## Primary reference context

These owner-supplied references identify future research obligations. They were
not fetched or executed during this bounded stage, and do not qualify this runtime.

- [RFC 9334, RATS architecture](https://www.rfc-editor.org/rfc/rfc9334.html): use as
  the baseline for explicitly separating evidence, appraisal and relying-party
  decisions; our Evidence struct implements none of the cryptographic roles.
- [NVIDIA GPU and switch attestation documentation](https://docs.nvidia.com/attestation/attestation-client-tools-sdk/latest/gpu_and_switch_attestation.html):
  future hardware-specific qualification must pin the actual supported device,
  firmware, driver, verifier and deployment configuration. A GPU enum is insufficient.
- [TEE.fail](https://tee.fail/) and [arXiv:2505.16501](https://arxiv.org/abs/2505.16501):
  adversarial research inputs for future platform-specific threat review, not proof
  that any unspecified platform is secure or broken. Record applicable assumptions
  and mitigations before designing an experiment.
- [Pinned upstream billing source](https://github.com/comrade-coop/apocryph/blob/e622a1074261905d8ace3b65571d93a6c036ffc4/backend/minio-manager/payment.go):
  coordinator-supplied static source review finding, not measured runtime behavior. No code
  was copied and no upstream execution was performed.

Manna remains separate optional automation. Metahive remains separate
research/backend work. No proprietary implementation, private context, legal
status or company facts are imported from either.

## Selected design adoption

[SCIENTIFIC-PRINCIPLES](SCIENTIFIC-PRINCIPLES.md) adopts P01–P10 from the supplied
design reference; [SCIENCE.csv](../registers/SCIENCE.csv) maps actual APIs, tests
and gaps. This selection is normative, not measured implementation or scientific
novelty. The two added bounded properties address partition invariance and provider
permutation invariance only; [E15/E16](EXPERIMENTS.md) state their domains and
falsifiers. Execution remains NOT_RUN. Bare amounts do not authenticate asset
identity or bridge Rate, Workload and Settlement into a durable invoice protocol.
