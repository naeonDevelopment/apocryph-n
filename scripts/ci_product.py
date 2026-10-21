#!/usr/bin/env python3
"""Qualified fleet product execution only; never invoke in a worker."""
import os
import subprocess
import sys
from ci_preflight import verify

rustc, cargo = verify()
os.environ['CARGO_NET_OFFLINE'] = 'true'
os.environ['RUSTC'] = rustc
for action in ('test', 'build'):
    command = [cargo, action, '--offline']
    print(command, flush=True)
    subprocess.run(command, check=True)
subprocess.run([sys.executable, 'scripts/commerce_smoke.py'], check=True)
