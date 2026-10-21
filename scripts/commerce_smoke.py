#!/usr/bin/env python3
"""Product smoke test: qualified fleet ONLY. Executes the real offline CLI twice."""
import os
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
binary = root / 'target/debug/commerce'
fixture = (root / 'fixtures/commerce.tsv').read_bytes()
expected = (root / 'fixtures/commerce.expected.tsv').read_bytes()
with tempfile.TemporaryDirectory(prefix='apocryph-commerce-', dir=os.environ['RUNNER_TEMP']) as directory:
    journal = Path(directory) / 'journal'
    prior = None
    for outcome in (b'new', b'replay'):
        result = subprocess.run([str(binary), str(journal)], input=fixture, capture_output=True, check=True)
        rows = result.stdout.splitlines(keepends=True)
        commands = [r for r in rows if r.startswith(b'S1\tcommand\t')]
        if len(commands) != len(fixture.splitlines()) or any(r.rstrip(b'\n').split(b'\t')[-1] != outcome for r in commands):
            raise SystemExit('wrong command acknowledgments')
        summaries = b''.join(r for r in rows if not r.startswith(b'S1\tcommand\t'))
        if summaries != expected:
            raise SystemExit(f'summary mismatch: {summaries!r}')
        current = journal.read_bytes()
        if prior is not None and current != prior:
            raise SystemExit('replay appended new bytes')
        prior = current
        print(result.stdout.decode('ascii'), end='')
    # Truncated generic input is rejected without discarding previously durable state.
    result = subprocess.run([str(binary), str(journal)], input=b'C1\tbroken', capture_output=True)
    if result.returncode == 0 or journal.read_bytes() != prior:
        raise SystemExit('truncated-input rejection failed')
print('CLI synthetic fixture and restart replay passed; no external effect exercised.')
