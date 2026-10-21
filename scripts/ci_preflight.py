#!/usr/bin/env python3
"""Fail-closed campaign preflight. Does not enroll, install or select fallback runners."""
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess


def installed_binary(name, release, host):
    found = shutil.which(name)
    if found is None:
        raise SystemExit(f'{name} is not installed')
    binary = Path(found).resolve(strict=True)
    if binary.name == 'rustup':
        raise SystemExit('standalone Rust binaries required; rustup proxies denied')
    version = subprocess.check_output([str(binary), '-Vv'], text=True)
    fields = version.splitlines()
    if (fields.count(f'release: {release}') != 1
            or fields.count(f'host: {host}') != 1):
        raise SystemExit(f'{name} release or host does not match qualification')
    print(f'{name}: {binary}\n{version}', end='', flush=True)
    return str(binary)


def verify():
    routes = {'naevum-vps-amd64': ('X64', 'x86_64'), 'ci-linux-arm64': ('ARM64', 'aarch64')}
    route = os.environ['CI_ROUTE']
    if route not in routes or os.environ['CI_CAMPAIGN'] != 'apocryph-compute-20261021':
        raise SystemExit('unqualified route or campaign')
    if os.environ.get('CI_JUDGE_SURFACES_REVIEWED') != 'true':
        raise SystemExit('independent source/checker review is required')
    arch, machine = routes[route]
    if (os.environ['RUNNER_ENVIRONMENT'] != 'self-hosted'
            or os.environ['RUNNER_OS'] != 'Linux' or platform.system() != 'Linux'
            or os.environ['RUNNER_ARCH'] != arch or platform.machine() != machine):
        raise SystemExit('self-hosted/Linux/architecture preflight failed')
    toolchain = os.environ['QUALIFIED_RUST_TOOLCHAIN']
    if toolchain != '1.99.1':
        raise SystemExit('only the qualified standalone release 1.99.1 is allowed')
    sha = os.environ['CI_SOURCE_SHA']
    if not re.fullmatch(r'[0-9a-f]{40}', sha):
        raise SystemExit('invalid source revision')
    actual = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    if actual != sha or subprocess.check_output(['git', 'status', '--porcelain'], text=True):
        raise SystemExit('source revision mismatch or dirty checkout')
    print(f'campaign={os.environ["CI_CAMPAIGN"]} route={route} revision={actual}', flush=True)
    print(f'runner={os.environ.get("RUNNER_NAME", "unknown")} os={platform.platform()} arch={machine}', flush=True)
    host = f'{machine}-unknown-linux-gnu'
    rustc = installed_binary('rustc', toolchain, host)
    cargo = installed_binary('cargo', toolchain, host)
    return rustc, cargo

if __name__ == '__main__':
    verify()
