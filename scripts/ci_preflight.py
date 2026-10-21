#!/usr/bin/env python3
"""Fail-closed campaign preflight. Does not enroll, install or select fallback runners."""
import os
import platform
import re
import subprocess


def verify():
    routes = {'naevum-vps-amd64': ('X64', 'x86_64'), 'ci-linux-arm64': ('ARM64', 'aarch64')}
    route = os.environ['CI_ROUTE']
    if route not in routes or os.environ['CI_CAMPAIGN'] != 'apocryph-compute-20261021':
        raise SystemExit('unqualified route or campaign')
    arch, machine = routes[route]
    if (os.environ['CI_RUNNER_ENVIRONMENT'] != 'self-hosted'
            or os.environ['CI_RUNNER_OS'] != 'Linux' or platform.system() != 'Linux'
            or os.environ['CI_RUNNER_ARCH'] != arch or platform.machine() != machine):
        raise SystemExit('self-hosted/Linux/architecture preflight failed')
    toolchain = os.environ['QUALIFIED_RUST_TOOLCHAIN']
    if not re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+', toolchain):
        raise SystemExit('an exact installed stable release is required; aliases denied')
    sha = os.environ['CI_SOURCE_SHA']
    if not re.fullmatch(r'[0-9a-f]{40}', sha):
        raise SystemExit('invalid source revision')
    actual = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    if actual != sha or subprocess.check_output(['git', 'status', '--porcelain'], text=True):
        raise SystemExit('source revision mismatch or dirty checkout')
    print(f'campaign={os.environ["CI_CAMPAIGN"]} route={route} revision={actual}', flush=True)
    print(f'runner={os.environ.get("RUNNER_NAME", "unknown")} os={platform.platform()} arch={machine}', flush=True)
    installed = subprocess.check_output(['rustup', 'toolchain', 'list'], text=True)
    # rustup run with no --install flag never installs a missing toolchain.
    if not any(line.split()[0] == toolchain or line.split()[0].startswith(toolchain + '-') for line in installed.splitlines() if line.split()):
        raise SystemExit('qualified toolchain is not installed')
    version = subprocess.check_output(['rustup', 'run', toolchain, 'rustc', '-Vv'], text=True)
    if f'release: {toolchain}\n' not in version or f'host: {machine}-unknown-linux-gnu\n' not in version:
        raise SystemExit('toolchain version or host does not match qualification')
    print(version, end='', flush=True)
    subprocess.run(['rustup', 'run', toolchain, 'cargo', '-Vv'], check=True)
    return toolchain

if __name__ == '__main__':
    verify()
