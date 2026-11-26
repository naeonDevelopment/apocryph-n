#!/usr/bin/env python3
"""cell — the repository cell toolkit (ALS-2).

    cell.py conform [--root R] [--format json]      does this repository meet the standard
    cell.py pulse   [--root R] [--forge] [--out F]  the repository's generated status record
    cell.py check | emit | adopt PATH [--root R]    compiler, for cells whose sources live in agent/
    cell.py hooks   [--root R]                      the two registrations a repository's own compiler emits
    cell.py attribution (--message FILE | --range A..B) [--root R]   no AI attribution (D3)
    cell.py probe-codex [--root R] [--codex-home H] is each cell hook loaded and trusted in Codex
    cell.py selftest                                the toolkit's own canaries

Exit codes (verifier contract): 0 clean, 1 findings, 2 no local verdict, 3 an external service failed.
Standard library only, Python 3.11+. The guard itself (guard.py, policy.py) runs on 3.9.
"""
from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import fcntl
import hashlib
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

KIT = Path(__file__).resolve().parent
CELL_DIR = ".agents/cell"
CONFIG = CELL_DIR + "/cell.json"
VERSION = "cell/1"

CLAUDE_HOOK = {
    "type": "command",
    "command": "python3 \"$CLAUDE_PROJECT_DIR/.agents/cell/kit/guard.py\" --host claude",
    "timeout": 10,
    "onFailure": "block",
}
CODEX_HOOK = {
    "type": "command",
    "command": ("root=$(git rev-parse --show-toplevel) && python3 \"$root/.agents/cell/kit/guard.py\" --host codex; "
                "rc=$?; [ \"$rc\" -eq 0 ] || { [ \"$rc\" -eq 2 ] || echo 'BLOCKED: cell guard failed' >&2; exit 2; }"),
    "timeout": 10,
}
# The enforcement layer, not the programs: compilers must still be able to write .claude/skills and .agents/skills.
SANDBOX_WRITE_DENY = [".claude/settings.json", ".claude/hooks", ".codex", ".agents/cell", ".git/config", ".git/hooks", ".mcp.json"]
NOTES: list[str] = []  # observations that are not findings (another hook's refusal of a control, say)


def sandbox_covered(path: str, entries: list[str]) -> bool:
    norm = [e[2:] if e.startswith("./") else e for e in entries]
    norm = [e.rstrip("/") for e in norm]
    return any(path == e or path.startswith(e + "/") for e in norm)

ATTRIBUTION = [
    re.compile(r"^\s*co-authored-by:.*\b(claude|anthropic|openai|codex|gpt|gemini|copilot|cursor|chatgpt)\b", re.I | re.M),
    re.compile(r"^\s*co-authored-by:.*noreply@(anthropic|openai)\.com", re.I | re.M),
    re.compile(r"^\s*assisted-by:", re.I | re.M),
    re.compile(r"generated with \[?(claude|codex|chatgpt)", re.I),
]


class NoVerdict(Exception):
    """Exit 2: the check could not reach a verdict."""


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git(root: Path, *args: str, check: bool = True) -> str:
    r = subprocess.run(["git", "-C", str(root), *args], capture_output=True, text=True)
    if check and r.returncode != 0:
        raise NoVerdict(f"git {' '.join(args)}: {r.stderr.strip()[:200]}")
    return r.stdout.strip()


def load_cell(root: Path) -> dict:
    path = root / CONFIG
    if not path.is_file():
        raise NoVerdict(f"{CONFIG} is missing")
    cell = json.loads(path.read_text())
    if cell.get("schema") != VERSION:
        raise NoVerdict(f"{CONFIG}: schema must be {VERSION}")
    return cell


def report(tool: str, findings: list[dict], fmt: str, extra: dict | None = None) -> int:
    code = 1 if findings else 0
    if fmt == "json":
        out = {"tool": tool, "ok": code == 0, "exit": code, "findings": findings}
        out.update(extra or {})
        print(json.dumps(out, indent=2))
    else:
        for f in findings:
            print(f"FINDING {f['rule']} {f['where']}: {f['message']}")
        print(f"{tool}: {'ok' if code == 0 else str(len(findings)) + ' finding(s)'}")
    return code


# ------------------------------------------------------------------ hook execution (black box)

def _claude_pre_hooks(root: Path) -> list[tuple[str, str]]:
    path = root / ".claude/settings.json"
    if not path.is_file():
        return []
    data = json.loads(path.read_text())
    out = []
    for group in data.get("hooks", {}).get("PreToolUse", []):
        for h in group.get("hooks", []):
            if h.get("type") == "command":
                args = [a.replace("${CLAUDE_PROJECT_DIR}", str(root)).replace("$CLAUDE_PROJECT_DIR", str(root))
                        for a in h.get("args", [])]  # the host substitutes these in exec-form args
                out.append((group.get("matcher", ""), h["command"] if not args else
                            " ".join([h["command"], *map(shlex.quote, args)]), h))
    return out


def _codex_pre_hooks(root: Path) -> list[tuple[str, str]]:
    out = []
    path = root / ".codex/hooks.json"
    if path.is_file():
        data = json.loads(path.read_text())
        for group in data.get("hooks", {}).get("PreToolUse", []):
            for h in group.get("hooks", []):
                if h.get("type") == "command":
                    out.append((group.get("matcher", ""), h["command"], h))
    path = root / ".codex/config.toml"
    if path.is_file():
        data = tomllib.loads(path.read_text())
        for group in data.get("hooks", {}).get("PreToolUse", []):
            for h in group.get("hooks", []):
                if h.get("type") == "command":
                    out.append((group.get("matcher", ""), h["command"], h))
    return out


def _matches(matcher: str, tool: str) -> bool:
    if matcher in ("", "*", ".*"):
        return True
    try:
        return re.fullmatch(matcher, tool) is not None
    except re.error:
        return matcher == tool


def run_hooks(root: Path, host: str, tool: str, tool_input: dict) -> tuple[str, list[int], str, bool]:
    """Run every registered PreToolUse command for `tool` as the host would.
    Returns (verdict of all hooks, exits, verdict of the cell guard alone, whether another hook denied)."""
    hooks = _claude_pre_hooks(root) if host == "claude" else _codex_pre_hooks(root)
    event = json.dumps({"hook_event_name": "PreToolUse", "session_id": "cell-conform", "cwd": str(root),
                        "tool_name": tool, "tool_input": tool_input}).encode()
    env = dict(os.environ, CLAUDE_PROJECT_DIR=str(root))
    env.pop("CODEX_HOME", None)
    verdict, exits, cell_verdict, other = "defer", [], "defer", False
    for matcher, command, spec in hooks:
        if not _matches(matcher, tool):
            continue
        try:
            r = subprocess.run(["/bin/sh", "-c", command], input=event, cwd=root, env=env,
                               capture_output=True, timeout=spec.get("timeout", 30))
            rc = r.returncode
        except subprocess.TimeoutExpired:
            rc = -1
        exits.append(rc)
        denied = rc == 2
        if rc == 0 and r.stdout.strip().startswith(b"{"):
            with contextlib.suppress(Exception):
                out = json.loads(r.stdout)
                denied = out.get("hookSpecificOutput", {}).get("permissionDecision") == "deny" or out.get("decision") == "block"
        if rc not in (0, 2) and host == "claude" and spec.get("onFailure") == "block":
            denied = True
        if denied:
            verdict = "deny"
            if "cell/kit/guard.py" in command:
                cell_verdict = "deny"
            else:
                other = True
    return verdict, exits, cell_verdict, other


# ------------------------------------------------------------------ conformance

def canaries(root: Path, cell: dict) -> list[tuple[str, str, dict, str]]:
    """(rule, tool-for-claude, input, expected). Codex equivalents are derived."""
    c = cell.get("canaries", {})
    out = [
        ("canary-cell-config", "Edit", {"file_path": str(root / CONFIG), "old_string": "a", "new_string": "b"}, "deny"),
        ("canary-force-push", "Bash", {"command": "git push --force origin HEAD"}, "deny"),
        ("canary-default-push", "Bash", {"command": f"git push origin HEAD:{cell.get('default_branch', 'main')}"}, "deny"),
        ("canary-no-verify", "Bash", {"command": "git commit --no-verify -m x"}, "deny"),
        ("canary-unknown-mcp", "mcp__cell_canary__delete_everything", {}, "deny"),
        ("canary-unknown-tool", "CellCanaryUnknownTool", {}, "deny"),
        ("control-read", "Read", {"file_path": str(root / "AGENTS.md")}, "defer"),
        ("control-ask-user", "AskUserQuestion", {"questions": []}, "defer"),
        ("control-plain-shell", "Bash", {"command": "ls"}, "defer"),
        ("control-ordinary-edit", "Write", {"file_path": str(root / "cell-canary-scratch.txt"), "content": "x"}, "defer"),
    ]
    for path in c.get("protected_writes", []):
        out.append(("canary-protected-write", "Write", {"file_path": str(root / path), "content": "x"}, "deny"))
    for command in c.get("deny_commands", []):
        out.append(("canary-deny-command", "Bash", {"command": command}, "deny"))
    for tool in c.get("mcp_reads", []):
        out.append(("control-mcp-read", tool, {}, "defer"))
    return out


def codex_form(tool: str, data: dict) -> tuple[str, dict]:
    if tool == "Bash":
        return "exec_command", {"cmd": data["command"]}
    if tool in ("Edit", "Write"):
        return "apply_patch", {"command": f"*** Begin Patch\n*** Update File: {data['file_path']}\n@@\n-a\n+b\n*** End Patch"}
    if tool == "Read":
        return "view_image", {"path": data["file_path"]}
    if tool == "AskUserQuestion":
        return "request_user_input", {}
    return tool, data


def conform(root: Path) -> list[dict]:
    findings: list[dict] = []

    def add(rule, where, message):
        findings.append({"rule": rule, "where": where, "message": message})

    try:
        cell = load_cell(root)
    except NoVerdict as e:
        add("cell-config", CONFIG, str(e))
        return findings
    tier = cell.get("tier", "A")
    for key in ("repository", "holder", "tier", "default_branch"):
        if not cell.get(key):
            add("cell-config", CONFIG, f"missing `{key}`")

    # Contract: one source, both hosts load it, small enough to be read whole.
    agents, claude = root / "AGENTS.md", root / "CLAUDE.md"
    if not agents.is_file():
        add("contract-agents", "AGENTS.md", "missing: Codex has no contract here")
    else:
        size = agents.stat().st_size
        if size > 32768:
            add("contract-size", "AGENTS.md", f"{size} bytes exceeds Codex's 32 KiB instruction budget")
        text = agents.read_text(errors="replace")
        lines = [l for l in text.splitlines() if l.strip() and not l.startswith("<!--")]
        if len(lines) < 12 and re.search(r"(read|follow|see)[^.\n]{0,60}\(?[\w./-]+\.md\)?", text, re.I):
            add("contract-pointer", "AGENTS.md", "is a pointer to another file; Codex loads only this file, so the contract never reaches it")
        for m in re.finditer(r"(use|uses|add|carry|carries|with)\b[^.\n]{0,40}assisted-by", text, re.I):
            if re.search(r"\b(no|not|never|without)\b", m.group(0), re.I):
                continue  # "carry no AI attribution: no Assisted-by" is the rule, not a breach
            add("contract-attribution", "AGENTS.md", "instructs an `Assisted-by:` trailer; the portfolio rule is no AI attribution")
            break
        if re.search(r"co-authored-by:\s*<?model", text, re.I):
            add("contract-attribution", "AGENTS.md", "instructs a model Co-Authored-By trailer")
    if (root / "AGENTS.override.md").exists():
        add("contract-override", "AGENTS.override.md", "shadows the shared contract for Codex")
    if not claude.is_file():
        if agents.is_file():
            add("contract-claude", "CLAUDE.md", "missing: Claude Code reads AGENTS.md only as a fallback that a personal CLAUDE.local.md suppresses")
    elif agents.is_file():
        body = claude.read_text(errors="replace")
        imports = any(l.strip() == "@AGENTS.md" for l in body.splitlines())
        if body != agents.read_text(errors="replace") and not imports:
            add("contract-claude", "CLAUDE.md", "neither an identical copy of AGENTS.md nor an `@AGENTS.md` import: Claude gets a different contract or only a link")

    # Programs: the same bytes for both hosts.
    cs, xs = root / ".claude/skills", root / ".agents/skills"
    if cs.exists() or xs.exists():
        if cs.is_symlink() and cs.resolve() == xs.resolve():
            pass
        else:
            def listing(d: Path) -> dict:
                # os.walk(followlinks=True): skills linked per entry are symlinked directories, which
                # Path.rglob does not enter; without following them parity would compare nothing.
                if not d.is_dir():
                    return {}
                out = {}
                for base, _dirs, files in os.walk(d, followlinks=True):
                    for name in files:
                        if name != ".DS_Store":
                            p = Path(base) / name
                            out[p.relative_to(d).as_posix()] = sha(p.read_bytes())
                return out
            a, b = listing(cs), listing(xs)
            for rel in sorted(set(a) ^ set(b)):
                add("programs-parity", rel, "present for one host only")
            for rel in sorted(k for k in set(a) & set(b) if a[k] != b[k]):
                add("programs-parity", rel, "differs between .claude/skills and .agents/skills")

    # Pre-tool policy, exercised black-box through each host's registered hooks.
    if tier in ("A", "B", "C"):
        if not _claude_pre_hooks(root):
            add("policy-claude", ".claude/settings.json", "no PreToolUse hook registered for Claude Code")
        elif not any(h.get("onFailure") == "block" for _, _, h in _claude_pre_hooks(root)):
            add("policy-fail-closed", ".claude/settings.json", "no PreToolUse hook sets onFailure: block, so a crashed guard lets the call through")
        if not _codex_pre_hooks(root):
            add("policy-codex", ".codex/hooks.json", "no PreToolUse hook registered for Codex")
        for host in ("claude", "codex"):
            if (host == "claude" and not _claude_pre_hooks(root)) or (host == "codex" and not _codex_pre_hooks(root)):
                continue
            for rule, tool, data, expected in canaries(root, cell):
                t, d = (tool, data) if host == "claude" else codex_form(tool, data)
                verdict, exits, cell_verdict, other = run_hooks(root, host, t, d)
                if expected == "deny" and verdict != "deny":
                    add(rule, f"{host}:{t}", f"expected deny, got defer (exits {exits})")
                elif expected == "defer" and cell_verdict != "defer":
                    add(rule, f"{host}:{t}", f"the cell guard refused a control (exits {exits})")
                elif expected == "defer" and other:
                    NOTES.append(f"{host}:{t}: another registered hook refused control `{rule}` (exits {exits}); "
                                 "not the cell's finding, but a person would be blocked the same way")
            # Fail-closed: a malformed event must be refused.
            hooks = _claude_pre_hooks(root) if host == "claude" else _codex_pre_hooks(root)
            for _, command, spec in hooks:
                if "cell/kit/guard.py" not in command:
                    continue
                r = subprocess.run(["/bin/sh", "-c", command], input=b"{not json", cwd=root,
                                   env=dict(os.environ, CLAUDE_PROJECT_DIR=str(root)), capture_output=True, timeout=30)
                if r.returncode != 2:
                    add("policy-fail-closed", f"{host}:guard", f"a malformed event exited {r.returncode}, not 2")
        scratch = root / "cell-canary-scratch.txt"
        if scratch.exists():
            add("policy-effect", str(scratch), "a canary produced a file: a hook executed the effect")

    # Sandbox settings for Claude Code in Tier A.
    if tier == "A":
        settings = root / ".claude/settings.json"
        deny = []
        if settings.is_file():
            deny = json.loads(settings.read_text()).get("sandbox", {}).get("filesystem", {}).get("denyWrite", [])
        missing = [p for p in SANDBOX_WRITE_DENY if not sandbox_covered(p, deny)]
        if missing:
            add("sandbox-write-deny", ".claude/settings.json", "sandbox.filesystem.denyWrite lacks " + ", ".join(missing))

    # Supply chain: the vendored kit matches its provenance.
    prov = root / CELL_DIR / "kit/PROVENANCE.json"
    if not prov.is_file():
        add("kit-provenance", str(prov.relative_to(root)), "missing: the vendored toolkit cannot be verified")
    else:
        data = json.loads(prov.read_text())
        for rel, digest in data.get("files", {}).items():
            p = root / CELL_DIR / "kit" / rel
            if not p.is_file() or sha(p.read_bytes()) != digest:
                add("kit-provenance", f"{CELL_DIR}/kit/{rel}", "differs from the kit release it claims")

    # The pulse can be produced.
    try:
        validate_pulse(build_pulse(root, cell, forge=False, run_state=False))
    except Exception as e:  # noqa: BLE001 -- any failure is the finding
        add("pulse", CONFIG, f"pulse could not be built: {e}")

    # Compiler parity, when the cell declares one.
    command = cell.get("compiler", {}).get("check")
    if command:
        r = subprocess.run(["/bin/sh", "-c", command], cwd=root, capture_output=True, timeout=300)
        if r.returncode != 0:
            add("compiler-check", command, f"exit {r.returncode}: {r.stdout.decode(errors='replace')[-300:].strip()}")
    return findings


# ------------------------------------------------------------------ pulse

PULSE_KEYS = ("schema", "generated_at", "mode", "identity", "health", "flow", "blocked",
              "decisions_needed", "changes", "lessons", "unmeasured")


def _gh(args: list[str], root: Path):
    r = subprocess.run(["gh", *args], cwd=root, capture_output=True, text=True, timeout=60)
    if r.returncode != 0:
        raise RuntimeError(r.stderr.strip()[:200])
    return json.loads(r.stdout or "null")


PARKED = {"needs/plan-step", "priority/deferred"}
VERDICT = re.compile(r"^\[[^\]]+\]\s+NO ISSUES at [0-9a-f]{40}\b", re.M)


def reviewed(pr: dict) -> bool:
    """A GitHub review, or the signed verdict comment the agent-autonomy standard accepts."""
    return bool(pr.get("reviews")) or any(VERDICT.search(c.get("body", "")) for c in pr.get("comments", []))


def _days(ts: str, now: dt.datetime) -> float:
    return round((now - dt.datetime.fromisoformat(ts.replace("Z", "+00:00"))).total_seconds() / 86400, 1)


def build_pulse(root: Path, cell: dict, forge: bool, rev: str = "HEAD", git_dir: Path | None = None,
                run_state: bool = True) -> dict:
    """`root` is a checkout; or pass `git_dir` and `rev` (e.g. origin/main) to read history without one."""
    now = dt.datetime.now(dt.timezone.utc)
    unmeasured: list[str] = []
    g = (lambda *a: git(root, "--git-dir", str(git_dir), *a, check=False)) if git_dir else (lambda *a: git(root, *a, check=False))
    commit = g("rev-parse", rev) or None
    pulse = {
        "schema": "cell.pulse/1",
        "generated_at": now.isoformat(timespec="seconds"),
        "mode": "forge" if forge else "local",
        "identity": {"repository": cell.get("repository"), "holder": cell.get("holder"), "tier": cell.get("tier"),
                     "default_branch": cell.get("default_branch"), "commit": commit},
        "health": {}, "flow": {}, "blocked": [], "decisions_needed": [], "changes": {}, "lessons": [],
        "unmeasured": unmeasured,
    }
    since = (now - dt.timedelta(days=7)).strftime("%Y-%m-%d")
    log = g("log", rev, f"--since={since}", "--name-only", "--format=@%H")
    if log or commit:
        commits = [l for l in log.splitlines() if l.startswith("@")]
        files = sorted({l for l in log.splitlines() if l and not l.startswith("@")})
        pulse["changes"] = {"commits_7d": len(commits), "files_changed_7d": len(files)}
        regs = cell.get("registers", {})
        pulse["lessons"] = sorted({f for f in files for r in regs.values() if f == r or f.startswith(r.rstrip("/") + "/")})
    else:
        unmeasured.append("changes")
    state = cell.get("state_check")
    if state and not forge and not git_dir and run_state:
        r = subprocess.run(["/bin/sh", "-c", state], cwd=root, capture_output=True, timeout=600)
        pulse["health"]["state_fresh"] = r.returncode == 0
    elif state:
        unmeasured.append("health.state_fresh")
    if not forge:
        unmeasured += ["flow", "blocked", "decisions_needed", "health.ci"]
        return pulse
    repo = cell.get("repository")
    try:
        prs = _gh(["pr", "list", "-R", repo, "--state", "open", "--limit", "200",
                   "--json", "number,createdAt,reviewDecision,isDraft"], root)
        ready = [p for p in prs if not p["isDraft"]]
        unreviewed = [p for p in ready if not p.get("reviewDecision")]
        merged = _gh(["pr", "list", "-R", repo, "--state", "merged", "--limit", "200", "--search",
                      f"merged:>={(now - dt.timedelta(days=14)).strftime('%Y-%m-%d')}", "--json", "number,reviews,comments"], root)
        pulse["flow"] = {
            "open": len(ready), "draft": len(prs) - len(ready),
            "oldest_open_days": max((_days(p["createdAt"], now) for p in ready), default=None),
            "unreviewed_open": len(unreviewed),
            "oldest_unreviewed_days": max((_days(p["createdAt"], now) for p in unreviewed), default=None),
            "merged_14d": len(merged),
            "merged_without_review_14d": sum(1 for p in merged if not reviewed(p)),
        }
        issues = _gh(["issue", "list", "-R", repo, "--state", "open", "--limit", "300",
                      "--json", "number,title,createdAt,labels"], root)
        for i in issues:
            labels = {l["name"] for l in i["labels"]}
            item = {"ref": f"{repo}#{i['number']}", "title": i["title"][:120], "age_days": _days(i["createdAt"], now)}
            if "needs/decision" in labels:
                pulse["decisions_needed"].append(item)
            elif labels & PARKED:
                continue  # deliberately parked by a ruling: not blocked, not news
            elif any(l.startswith("needs/") or l == "blocked" for l in labels):
                pulse["blocked"].append(dict(item, labels=sorted(labels)))
        runs = _gh(["run", "list", "-R", repo, "--branch", cell.get("default_branch", "main"), "--limit", "40",
                    "--json", "workflowName,conclusion,status,createdAt"], root)
        latest: dict = {}
        for run in runs:
            latest.setdefault(run["workflowName"], run)
        pulse["health"]["ci"] = [{"workflow": k, "conclusion": v.get("conclusion") or v.get("status"),
                                  "at": v["createdAt"]} for k, v in sorted(latest.items())]
    except (RuntimeError, subprocess.TimeoutExpired, json.JSONDecodeError, FileNotFoundError) as e:
        unmeasured += ["flow", "blocked", "decisions_needed", "health.ci"]
        pulse["forge_error"] = str(e)[:200]
    return pulse


def validate_pulse(p: dict) -> None:
    missing = [k for k in PULSE_KEYS if k not in p]
    if missing:
        raise ValueError("pulse lacks " + ", ".join(missing))
    if p["schema"] != "cell.pulse/1":
        raise ValueError("pulse schema")
    if not p["identity"].get("repository") or not p["identity"].get("holder"):
        raise ValueError("pulse identity incomplete")


# ------------------------------------------------------------------ compiler (cells whose sources live in agent/)

@contextlib.contextmanager
def locked(root: Path):
    lock = Path(tempfile.gettempdir()) / ("cell-" + sha(str(root.resolve()).encode())[:16] + ".lock")
    with open(lock, "w") as f:
        try:
            fcntl.flock(f, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError:
            raise NoVerdict("another cell compiler run holds the lock")
        yield


def plan(root: Path) -> dict[str, bytes]:
    src = root / "agent"
    if not (src / "contract.md").is_file():
        raise NoVerdict("agent/contract.md is missing")
    out: dict[str, bytes] = {}
    contract = (src / "contract.md").read_bytes()
    out["AGENTS.md"] = contract
    out["CLAUDE.md"] = contract
    skills = src / "skills"
    if skills.is_dir():
        for p in sorted(skills.rglob("*")):
            if p.is_symlink():
                raise NoVerdict(f"{p}: symlinks are refused in sources")
            if p.is_file() and p.name != ".DS_Store":
                rel = p.relative_to(skills).as_posix()
                out[f".claude/skills/{rel}"] = p.read_bytes()
                out[f".agents/skills/{rel}"] = p.read_bytes()
    base = json.loads((src / "claude.settings.json").read_text()) if (src / "claude.settings.json").is_file() else {}
    out[".claude/settings.json"] = (json.dumps(with_claude_hook(base), indent=2) + "\n").encode()
    xbase = json.loads((src / "codex.hooks.json").read_text()) if (src / "codex.hooks.json").is_file() else {}
    out[".codex/hooks.json"] = (json.dumps(with_codex_hook(xbase), indent=2) + "\n").encode()
    return out


def with_claude_hook(settings: dict) -> dict:
    s = json.loads(json.dumps(settings))
    groups = s.setdefault("hooks", {}).setdefault("PreToolUse", [])
    groups[:] = [g for g in groups if not any("cell/kit/guard.py" in h.get("command", "") for h in g.get("hooks", []))]
    groups.insert(0, {"matcher": "*", "hooks": [dict(CLAUDE_HOOK)]})
    fs = s.setdefault("sandbox", {}).setdefault("filesystem", {})
    deny = fs.setdefault("denyWrite", [])
    for p in SANDBOX_WRITE_DENY:
        if not sandbox_covered(p, deny):
            deny.append("./" + p)
    return s


def with_codex_hook(hooks: dict) -> dict:
    h = json.loads(json.dumps(hooks))
    groups = h.setdefault("hooks", {}).setdefault("PreToolUse", [])
    groups[:] = [g for g in groups if not any("cell/kit/guard.py" in x.get("command", "") for x in g.get("hooks", []))]
    groups.insert(0, {"matcher": ".*", "hooks": [dict(CODEX_HOOK)]})
    return h


def ledger_path(root: Path) -> Path:
    return root / "agent/emission.json"


def compile_check(root: Path) -> list[dict]:
    desired = plan(root)
    ledger = json.loads(ledger_path(root).read_text()) if ledger_path(root).is_file() else {}
    findings = []
    for rel, data in desired.items():
        p = root / rel
        if not p.is_file():
            findings.append({"rule": "compiler-missing", "where": rel, "message": "not emitted"})
        elif p.read_bytes() != data:
            findings.append({"rule": "compiler-drift", "where": rel, "message": "differs from its source; emit, or adopt the edit"})
    for rel in sorted(set(ledger) - set(desired)):
        if (root / rel).exists():
            findings.append({"rule": "compiler-retired", "where": rel, "message": "no longer has a source; emit removes it"})
    return findings


def atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=path.parent, prefix=".cell-stage-")
    with os.fdopen(fd, "wb") as f:
        f.write(data)
    os.chmod(tmp, 0o644)
    os.replace(tmp, path)


def compile_emit(root: Path) -> int:
    with locked(root):
        desired = plan(root)
        ledger = json.loads(ledger_path(root).read_text()) if ledger_path(root).is_file() else {}
        conflicts = []
        for rel, data in desired.items():
            p = root / rel
            if p.is_symlink():
                conflicts.append(f"{rel}: is a symlink")
            elif p.is_file():
                current = sha(p.read_bytes())
                if current not in (ledger.get(rel), sha(data)):
                    conflicts.append(f"{rel}: edited since the last emission; `adopt` it or restore it")
        retire = [rel for rel in ledger if rel not in desired and (root / rel).is_file()]
        for rel in retire:
            if sha((root / rel).read_bytes()) != ledger[rel]:
                conflicts.append(f"{rel}: retired output was edited; preserve it, then remove it yourself")
        if conflicts:
            for c in conflicts:
                print("CONFLICT " + c, file=sys.stderr)
            return 1
        for rel, data in desired.items():
            p = root / rel
            if not p.is_file() or p.read_bytes() != data:
                atomic_write(p, data)
        for rel in retire:
            (root / rel).unlink()
        atomic_write(ledger_path(root), (json.dumps({k: sha(v) for k, v in sorted(desired.items())}, indent=1) + "\n").encode())
    print(f"cell emit: {len(desired)} outputs")
    return 0


def compile_adopt(root: Path, rels: list[str]) -> int:
    with locked(root):
        ledger = json.loads(ledger_path(root).read_text()) if ledger_path(root).is_file() else {}
        desired = plan(root)
        for rel in rels:
            if rel in ("AGENTS.md", "CLAUDE.md"):
                src = root / "agent/contract.md"
            elif rel.startswith((".claude/skills/", ".agents/skills/")):
                src = root / "agent/skills" / rel.split("/skills/", 1)[1]
            else:
                print(f"REFUSED {rel}: only the contract and programs are adoptable; edit agent/ settings directly", file=sys.stderr)
                return 1
            if rel in desired and sha(desired[rel]) != ledger.get(rel):
                print(f"REFUSED {rel}: its source also changed since the last emission; merge by hand", file=sys.stderr)
                return 1
            atomic_write(src, (root / rel).read_bytes())
    return compile_emit(root)


# ------------------------------------------------------------------ attribution, vendoring, probe

def attribution(text: str) -> list[str]:
    return [rx.pattern for rx in ATTRIBUTION if rx.search(text)]


def vendor(root: Path, seed: bool = False) -> int:
    dest = root / CELL_DIR / "kit"
    dest.mkdir(parents=True, exist_ok=True)
    files = {}
    names = ["policy.py", "guard.py", "cell.py", "tools.json", "pulse.schema.json"]
    programs = KIT.parent / "programs"
    names += [p.relative_to(KIT.parent).as_posix() for p in sorted(programs.rglob("SKILL.md"))]
    for name in names:
        data = (KIT.parent / name).read_bytes() if name.startswith("programs/") else (KIT / name).read_bytes()
        atomic_write(dest / name, data)
        files[name] = sha(data)
    # Only on request: a repository with its own compiler keeps its programs where that compiler reads them.
    if seed and (root / "agent/contract.md").is_file():
        for p in sorted(programs.rglob("SKILL.md")):
            target = root / "agent/skills" / p.parent.name / "SKILL.md"
            if not target.exists():
                atomic_write(target, p.read_bytes())
    head = subprocess.run(["git", "-C", str(KIT), "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    prov = {"source": os.environ.get("CELL_KIT_SOURCE", "agent-kit (cell/kit)"), "commit": head or "uncommitted", "files": files,
            "note": "Vendored by copy. Change it in agent-kit and re-vendor; a local edit fails `cell.py conform`."}
    atomic_write(dest / "PROVENANCE.json", (json.dumps(prov, indent=1) + "\n").encode())
    print(f"vendored {len(files)} files into {dest}")
    return 0


def probe_codex(root: Path, home: str | None) -> int:
    env = dict(os.environ)
    if home:
        env["CODEX_HOME"] = home
    print(f"CODEX_HOME={env.get('CODEX_HOME', '~/.codex (unset)')}")
    proc = subprocess.Popen(["codex", "app-server", "--listen", "stdio://"], cwd=root, env=env,
                            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, start_new_session=True)
    try:
        def send(m):
            proc.stdin.write((json.dumps(m) + "\n").encode()); proc.stdin.flush()

        def recv(i):
            deadline = dt.datetime.now() + dt.timedelta(seconds=25)
            while dt.datetime.now() < deadline:
                line = proc.stdout.readline()
                if not line:
                    break
                m = json.loads(line)
                if m.get("id") == i:
                    if "error" in m:
                        raise RuntimeError(str(m["error"])[:200])
                    return m["result"]
            raise RuntimeError("no answer from codex app-server")
        send({"id": 1, "method": "initialize", "params": {"clientInfo": {"name": "cell_probe", "version": "1"},
                                                          "capabilities": {"experimentalApi": True}}})
        recv(1)
        send({"method": "initialized", "params": {}})
        send({"id": 2, "method": "hooks/list", "params": {"cwds": [str(root)]}})
        entries = recv(2)["data"]
    except (RuntimeError, json.JSONDecodeError, OSError) as e:
        print(f"probe-codex: no verdict: {e}", file=sys.stderr)
        return 3
    finally:
        proc.kill()
    hooks = [h for e in entries for h in e.get("hooks", [])]
    cell_hooks = [h for h in hooks if "cell/kit/guard.py" in str(h.get("command", ""))]
    for h in hooks:
        print(f"{h.get('eventName')}: enabled={h.get('enabled')} trust={h.get('trustStatus')} "
              f"source={h.get('sourcePath') or h.get('source')} cell={'cell/kit/guard.py' in str(h.get('command',''))}")
    if not cell_hooks:
        print("probe-codex: the cell guard is not loaded (project untrusted, or no registration)")
        return 1
    ok = all(h.get("enabled") and h.get("trustStatus") in ("trusted", "managed") for h in cell_hooks)
    print("probe-codex: " + ("ready" if ok else "loaded but not trusted: review it once with /hooks in Codex"))
    return 0 if ok else 1


# ------------------------------------------------------------------ main

def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="cell")
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("conform", "pulse", "check", "emit", "adopt", "hooks", "attribution", "probe-codex", "vendor", "selftest"):
        p = sub.add_parser(name)
        p.add_argument("--root", default=None)
        p.add_argument("--format", default="text", choices=("text", "json"))
        if name == "pulse":
            p.add_argument("--forge", action="store_true")
            p.add_argument("--out")
        if name == "adopt":
            p.add_argument("paths", nargs="+")
        if name == "attribution":
            p.add_argument("--message")
            p.add_argument("--range")
        if name == "probe-codex":
            p.add_argument("--codex-home")
        if name == "vendor":
            p.add_argument("--seed-programs", action="store_true")
    a = ap.parse_args(argv)
    root = Path(a.root).resolve() if a.root else Path(git(Path.cwd(), "rev-parse", "--show-toplevel"))
    try:
        if a.cmd == "conform":
            findings = conform(root)
            for n in NOTES:
                print("NOTE " + n, file=sys.stderr)
            return report("cell conform", findings, a.format, {"notes": NOTES} if a.format == "json" else None)
        if a.cmd == "pulse":
            p = build_pulse(root, load_cell(root), forge=a.forge)
            validate_pulse(p)
            text = json.dumps(p, indent=1) + "\n"
            if a.out:
                atomic_write(Path(a.out), text.encode())
            else:
                print(text, end="")
            return 0
        if a.cmd == "check":
            return report("cell check", compile_check(root), a.format)
        if a.cmd == "emit":
            return compile_emit(root)
        if a.cmd == "adopt":
            return compile_adopt(root, a.paths)
        if a.cmd == "hooks":
            print(json.dumps({"claude_settings_PreToolUse": {"matcher": "*", "hooks": [CLAUDE_HOOK]},
                              "codex_hooks_PreToolUse": {"matcher": ".*", "hooks": [CODEX_HOOK]},
                              "claude_sandbox_denyWrite": ["./" + p for p in SANDBOX_WRITE_DENY]}, indent=2))
            return 0
        if a.cmd == "attribution":
            if a.message:
                texts = [Path(a.message).read_text()]
            elif a.range:
                texts = git(root, "log", "--format=%B%x00", a.range).split("\x00")
            else:
                raise NoVerdict("give --message FILE or --range A..B")
            hits = sorted({h for t in texts for h in attribution(t)})
            findings = [{"rule": "attribution", "where": a.range or a.message, "message": "AI attribution: " + h} for h in hits]
            return report("cell attribution", findings, a.format)
        if a.cmd == "probe-codex":
            return probe_codex(root, a.codex_home)
        if a.cmd == "vendor":
            return vendor(root, a.seed_programs)
        if a.cmd == "selftest":
            if not (KIT.parent / "tests").is_dir():
                raise NoVerdict("the toolkit's tests are not vendored here; run the selftest in agent-kit")
            r = subprocess.run([sys.executable, "-m", "unittest", "discover", "-s", str(KIT.parent / "tests"), "-q"])
            return 0 if r.returncode == 0 else 1
    except NoVerdict as e:
        print(f"cell {a.cmd}: no verdict: {e}", file=sys.stderr)
        return 2
    return 2


if __name__ == "__main__":
    sys.exit(main())
