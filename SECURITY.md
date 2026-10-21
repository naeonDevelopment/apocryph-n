# Security reporting

For sensitive findings, use this repository's GitHub **Security → Advisories →
Report a vulnerability** private reporting path once the maintainer enables it.
Enablement has not been verified by this maintenance worker. If the button is
absent, do not put vulnerability details, exploit code, credentials or operator
data in a public issue or pull request. You may open a public issue asking only
for private vulnerability reporting to be enabled; retain the sensitive details
until a private reporting channel is available. No email address, response SLA
or bounty is promised.

A private report should identify the affected source revision, expected invariant,
observed behavior and a minimal synthetic reproduction. Remove secrets and real
operator/customer data. Do not probe deployed infrastructure to demonstrate a
finding. Report accidental disclosure through the same private path.

This is a bounded reference foundation, not a production security service. External
assertions are unauthenticated; the journal assumes a trusted local directory and
one cooperating writer. Its checksum is not cryptographic tamper evidence. See the
[threat model](docs/THREAT-MODEL.md) and [commerce limits](docs/COMMERCE.md).
Source availability and a scoped test result do not establish security support for
any deployment, platform or research integration. No supported-release matrix or
patch timeline has been established.
