#!/usr/bin/env python3
"""Pre-tool adapter for Claude Code and Codex: one event on stdin -> the cell's policy -> a native
denial, or nothing (defer to the host).

    guard.py --host claude|codex

Fail-closed: any exception, malformed event, oversized input or repository mismatch is a denial
with exit 2 and a fixed message that never echoes the payload. Claude Code registrations also set
`"onFailure": "block"`; Codex has no such option, so its launcher maps every non-zero exit to 2.

Each decision's code (never its payload) is appended to `<git-dir>/cell/decisions.jsonl` so the
cell can measure how often it blocks harmless work.

Runs on Python 3.9.
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

KIT = Path(__file__).resolve().parent
ROOT = KIT.parent.parent.parent  # <root>/.agents/cell/kit/guard.py
sys.path.insert(0, str(KIT))

from policy import DENY, Decision, Operation, Policy, Target, evaluate  # noqa: E402

MAX_EVENT = 2 * 1024 * 1024
WRITE_TOOLS = {"Edit", "Write", "MultiEdit", "NotebookEdit"}
SHELL_TOOLS = {"Bash", "exec_command", "shell", "shell_command", "local_shell"}


class Invalid(ValueError):
    pass


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def parse_event(raw: bytes) -> dict:
    def no_duplicates(pairs):
        seen = {}
        for key, value in pairs:
            if key in seen:
                raise Invalid("duplicate key")
            seen[key] = value
        return seen

    event = json.loads(raw.decode("utf-8"), object_pairs_hook=no_duplicates)
    if not isinstance(event, dict):
        raise Invalid("event is not an object")
    return event


def rel_target(root: Path, cwd: Path, value) -> Target:
    if not isinstance(value, str) or not value or "\x00" in value:
        raise Invalid("missing write path")
    path = Path(value)
    if not path.is_absolute():
        path = cwd / path
    # Resolve symlinks on both sides (macOS /var -> /private/var; a link that points at a protected file).
    path = Path(os.path.realpath(str(path)))
    try:
        rel = path.relative_to(Path(os.path.realpath(str(root))))
    except ValueError:
        # A write outside the repository is the host sandbox's job; the cell cannot judge it.
        return Target("../" + path.name, path.exists())
    return Target(rel.as_posix(), path.exists())


def patch_paths(text: str):
    paths = []
    for line in text.splitlines():
        m = re.match(r"^\*\*\* (?:Add File|Update File|Delete File|Move to): (.+)$", line)
        if m:
            paths.append(m.group(1).strip())
    if not paths:
        raise Invalid("patch names no file")
    return paths


def current_branch(root: Path) -> str:
    try:
        out = subprocess.run(["git", "-C", str(root), "symbolic-ref", "--quiet", "--short", "HEAD"],
                             capture_output=True, text=True, timeout=3)
        return out.stdout.strip()
    except Exception:
        return ""


def normalize(root: Path, event: dict) -> Operation:
    if event.get("hook_event_name", "PreToolUse") != "PreToolUse":
        raise Invalid("not a PreToolUse event")
    cwd = Path(event.get("cwd") or os.getcwd())
    tool = event.get("tool_name")
    args = event.get("tool_input")
    if not isinstance(tool, str) or not tool:
        raise Invalid("missing tool name")
    if not isinstance(args, dict):
        args = {}
    if tool.startswith("functions."):
        tool = tool[len("functions."):]
    if tool in WRITE_TOOLS:
        value = args.get("file_path") or args.get("notebook_path") or args.get("path")
        return Operation("write", tool, (rel_target(root, cwd, value),))
    if tool == "apply_patch":
        text = args.get("command") or args.get("input") or args.get("patch")
        if isinstance(text, list):
            text = "\n".join(str(t) for t in text)
        if not isinstance(text, str):
            raise Invalid("missing patch")
        return Operation("write", tool, tuple(rel_target(root, cwd, p) for p in patch_paths(text)))
    if tool == "Monitor" and isinstance(args.get("command"), str):
        tool, args = "Bash", {"command": args["command"]}  # Monitor runs its command in a shell
    if tool in SHELL_TOOLS:
        command = args.get("command") if "command" in args else args.get("cmd")
        if isinstance(command, list):
            command = " ".join(str(c) for c in command)
        if not isinstance(command, str) or not command.strip():
            raise Invalid("missing command")
        return Operation("shell", tool, command=command, branch=current_branch(root))
    if tool.startswith("mcp__"):
        return Operation("mcp", tool)
    return Operation("tool", tool)


def load_policy(root: Path) -> Policy:
    cell = load_json(root / ".agents/cell/cell.json")
    registry = load_json(KIT / "tools.json")
    policy = Policy.from_data(cell, registry)
    policy.root = os.path.realpath(str(root))
    policy.exists = lambda rel: (root / rel).exists()
    return policy


def record(root: Path, host: str, decision: Decision) -> None:
    try:
        common = subprocess.run(["git", "-C", str(root), "rev-parse", "--git-common-dir"],
                                capture_output=True, text=True, timeout=3).stdout.strip()
        if not common:
            return
        log = (root / common if not os.path.isabs(common) else Path(common)) / "cell" / "decisions.jsonl"
        log.parent.mkdir(parents=True, exist_ok=True)
        with open(log, "a", encoding="utf-8") as f:
            f.write(json.dumps({"t": int(time.time()), "host": host, "action": decision.action,
                                "code": decision.code}) + "\n")
    except Exception:
        pass  # measurement must never change the decision


def emit(decision: Decision) -> int:
    if decision.action != DENY:
        return 0
    reason = f"{decision.code}: {decision.reason}"
    print(json.dumps({"hookSpecificOutput": {"hookEventName": "PreToolUse",
                                             "permissionDecision": "deny",
                                             "permissionDecisionReason": reason}}))
    print("BLOCKED by the cell policy: " + reason, file=sys.stderr)
    return 2


def decide(root: Path, raw: bytes) -> Decision:
    if len(raw) > MAX_EVENT:
        raise Invalid("event too large")
    return evaluate(normalize(root, parse_event(raw)), load_policy(root))


def main(argv=None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    host = argv[argv.index("--host") + 1] if "--host" in argv else "unknown"
    root = ROOT
    try:
        raw = sys.stdin.buffer.read(MAX_EVENT + 1)
        decision = decide(root, raw)
    except Exception:
        decision = Decision(DENY, "guard-failed", "the event, the repository or the policy could not be read; nothing is authorized")
    record(root, host, decision)
    return emit(decision)


if __name__ == "__main__":
    sys.exit(main())
