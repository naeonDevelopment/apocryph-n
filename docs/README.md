# Documentation authorities

Each scope has one current authority; links elsewhere explain or route to it.

| Scope | Authority |
| --- | --- |
| Agent constraints and permissions | [contract](../agent/contract.md) |
| Current selected network RESEARCH DESIGN (not v0.1 behavior) | [NETWORK-THESIS](NETWORK-THESIS.md) |
| Legacy API semantics and invariants | [ARCHITECTURE](ARCHITECTURE.md) |
| Typed commerce; local journal; generic TSV input/output | [COMMERCE](COMMERCE.md) |
| Adversaries and trust boundaries | [THREAT-MODEL](THREAT-MODEL.md) |
| Selected normative principles P01–P10 | [SCIENTIFIC-PRINCIPLES](SCIENTIFIC-PRINCIPLES.md) |
| Principle-to-source/witness/gap applicability (no fix status) | [SCIENCE.csv](../registers/SCIENCE.csv) |
| Research claims, evidence and falsifiers | [RESEARCH](RESEARCH.md) |
| Change status and original evidence | [IMPROVEMENTS.csv](../registers/IMPROVEMENTS.csv) |
| Human-readable change explanations | [FIXES](FIXES.md) |
| Preregistered experiment procedures and results | [EXPERIMENTS](EXPERIMENTS.md) |
| Future product scope | [ROADMAP](ROADMAP.md) |
| Runner qualification and CI route | [CI-RUNNERS](development/CI-RUNNERS.md) |
| Public contributor workflow | [CONTRIBUTING](../CONTRIBUTING.md) |
| Safe vulnerability reporting | [SECURITY](../SECURITY.md) |
| Provenance | [ORIGINS](../ORIGINS.md) |

`AGENTS.md` and `CLAUDE.md` are generated pointers, not alternative contracts.
IMPROVEMENTS.csv is the sole change-status authority; SCIENCE.csv maps principle applicability only. EXPERIMENTS specifies planned
procedures and the [dated external qualification note](EXPERIMENTS.md#external-qualification-note-2026-10-22).
Historical NOT_RUN fields remain historical; the new maintenance candidate has no
execution receipt. README's verification boundary applies repository-wide.

This public v0.1.0 foundation preserves 30 legacy and 14 commerce witnesses.
Their exact original-source qualification is attributed to the coordinator, not
this worker or the changed CI helpers.
Read FIXES for A01–A10, explicit billing-unit/age/challenge checks and business
limitations; RESEARCH and THREAT-MODEL separate local assertions from runtime proof.
No marketplace, hardware verifier or durable service is implied by the API.

The network thesis owns the selected service-contract architecture, economic and
application boundaries, and future independent simulator interface. It does not
replace ARCHITECTURE's actual API semantics or the register's implementation
statuses. FIXES audits every source/test boundary and explains N01–N11; the
[E17 family](EXPERIMENTS.md#e17-network-policy-and-simulator-family) defines proposed
preregistration targets, not measurements. ROADMAP sequences the missing work.
No adapter or runtime dependency on compute-cluster-sim is established.
