"""Pure pre-tool policy: an operation and the repository's policy data in, a decision out.

Decisions are `deny` or `defer`. There is no `allow`: deferring leaves the call to the host's own
permissions and never proves anyone consented. Nothing here reads files, the network or the
environment, and no field of the operation can grant itself anything.

Runs on Python 3.9 (the system interpreter a teammate's hook may get).
"""
from __future__ import annotations

import fnmatch
import re
import shlex
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Sequence, Tuple

DENY, DEFER = "deny", "defer"

# Agent-governing files every cell protects, whatever its own data says.
BUILTIN_PROTECTED = (
    ".agents/cell/**",
    ".claude/settings.json",
    ".claude/hooks/**",
    ".codex/**",
    ".git/**",
    ".mcp.json",
)

# Tool classes. `read`, `interaction` and `orchestration` defer; `external` defers so the host asks;
# `write`, `shell` and `mcp` are evaluated; anything unclassified is refused.
DEFER_CLASSES = ("read", "interaction", "orchestration", "external")


@dataclass(frozen=True)
class Target:
    path: str  # relative to the repository root, POSIX separators
    exists: bool


@dataclass(frozen=True)
class Operation:
    kind: str  # write | shell | mcp | tool
    tool: str
    targets: Tuple[Target, ...] = ()
    command: str = ""
    branch: str = ""  # current branch, measured by the adapter; "" when unknown


@dataclass(frozen=True)
class Decision:
    action: str
    code: str
    reason: str


@dataclass
class Policy:
    default_branch: str = "main"
    protected: List[Tuple[str, str]] = field(default_factory=list)  # (glob, reason)
    protected_existing: List[Tuple[str, str]] = field(default_factory=list)
    generated: List[str] = field(default_factory=list)
    deny_commands: List[Tuple[str, str, str]] = field(default_factory=list)  # (regex, code, reason)
    mcp_reads: Tuple[str, ...] = ()
    tools: Dict[str, str] = field(default_factory=dict)  # tool name -> class

    @classmethod
    def from_data(cls, cell: dict, registry: dict) -> "Policy":
        p = cell.get("policy", {})
        tools: Dict[str, str] = {}
        for host in registry.get("hosts", {}).values():
            for cls_name, names in host.items():
                for name in names:
                    tools[name] = cls_name
        for name, cls_name in p.get("tools", {}).items():
            tools[name] = cls_name
        return cls(
            default_branch=cell.get("default_branch", "main"),
            protected=[(e["glob"], e["reason"]) for e in p.get("protected", [])],
            protected_existing=[(e["glob"], e["reason"]) for e in p.get("protected_existing", [])],
            generated=list(p.get("generated", [])),
            deny_commands=[(e["pattern"], e["code"], e["reason"]) for e in p.get("deny_commands", [])],
            mcp_reads=tuple(p.get("mcp_reads", [])),
            tools=tools,
        )


def _match(path: str, pattern: str) -> bool:
    path, pattern = path.casefold(), pattern.casefold()
    if pattern.endswith("/**"):
        base = pattern[:-3]
        return path == base or path.startswith(base + "/")
    return fnmatch.fnmatchcase(path, pattern)


def protected_reason(policy: Policy, target: Target) -> Optional[str]:
    for pattern in BUILTIN_PROTECTED:
        if _match(target.path, pattern):
            return "agent-governing file; change it through the cell's source and review"
    for pattern in policy.generated:
        if _match(target.path, pattern):
            return "generated output; edit the canonical source and regenerate"
    for pattern, reason in policy.protected:
        if _match(target.path, pattern):
            return reason
    if target.exists:
        for pattern, reason in policy.protected_existing:
            if _match(target.path, pattern):
                return reason
    return None


def protected_patterns(policy: Policy) -> List[str]:
    return list(BUILTIN_PROTECTED) + list(policy.generated) + [g for g, _ in policy.protected]


# ------------------------------------------------------------------ shell

_SEPARATORS = re.compile(r"(?:&&|\|\||;|\||\n)")
_READ_ONLY = re.compile(r"^(?:cat|head|tail|less|rg|grep|ls|wc|stat|file|git\s+(?:diff|show|status|log|blame))\b")


def _segments(command: str) -> List[List[str]]:
    """Split on command separators, then tokenize each part. Raises ValueError on bad quoting."""
    out = []
    for part in _SEPARATORS.split(command.replace("\\\n", " ")):
        part = part.strip()
        if part:
            out.append(shlex.split(part, posix=True))
    return out


def _strip_env(tokens: List[str]) -> List[str]:
    i = 0
    while i < len(tokens) and re.match(r"^[A-Za-z_][A-Za-z0-9_]*=", tokens[i]):
        i += 1
    rest = tokens[i:]
    if rest and rest[0] in ("env", "command", "builtin", "exec", "nohup", "time", "sudo"):
        return _strip_env(rest[1:])
    return rest


def git_decision(tokens: List[str], policy: Policy, branch: str) -> Optional[Decision]:
    """D4: history-destroying and default-branch effects are refused; the rest is the forge's job."""
    if not tokens or tokens[0].rsplit("/", 1)[-1] != "git":
        return None
    args = tokens[1:]
    # Skip global options such as `-C path`, `-c key=value`, `--git-dir=...`.
    while args and args[0].startswith("-"):
        args = args[2:] if args[0] in ("-C", "-c") else args[1:]
    if not args:
        return None
    verb, rest = args[0], args[1:]
    default = policy.default_branch
    if "--no-verify" in rest or (verb == "commit" and "-n" in rest):
        return Decision(DENY, "git-no-verify", "skipping repository hooks is refused; fix what the hook reports")
    if verb == "push":
        if any(a in ("-f", "--force", "--force-with-lease", "--mirror", "--delete", "-d", "--prune") or
               a.startswith("--force") for a in rest):
            return Decision(DENY, "git-force-push", "force, mirror, delete and prune pushes are refused")
        refs = [a for a in rest if not a.startswith("-")]
        for ref in refs[1:]:
            if ref.startswith("+") or ref.startswith(":"):
                return Decision(DENY, "git-force-push", "forced or deleting refspecs are refused")
            dest = ref.split(":", 1)[-1]
            if dest in (default, "refs/heads/" + default):
                return Decision(DENY, "git-default-branch", "push to the default branch is refused; open a pull request")
        if len(refs) <= 1 and branch == default:
            return Decision(DENY, "git-default-branch", "push from the default branch is refused; open a pull request")
    if verb in ("filter-branch", "filter-repo"):
        return Decision(DENY, "git-history-rewrite", "history rewriting is refused")
    if verb == "update-ref" and "-d" in rest:
        return Decision(DENY, "git-history-rewrite", "deleting refs is refused")
    return None


def shell_decision(command: str, policy: Policy, branch: str) -> Decision:
    try:
        segments = _segments(command)
    except ValueError:
        return Decision(DENY, "invalid-shell", "the command could not be tokenized, so it cannot be classified")
    flat = " ".join(" ".join(seg) for seg in segments)
    for pattern, code, reason in policy.deny_commands:
        if re.search(pattern, flat, re.S):
            return Decision(DENY, code, reason)
    protected = protected_patterns(policy)
    for seg in segments:
        tokens = _strip_env(seg)
        if not tokens:
            continue
        git = git_decision(tokens, policy, branch)
        if git:
            return git
        text = " ".join(tokens)
        if _READ_ONLY.match(text) and not re.search(r"[<>`$]", text):
            continue
        for token in tokens[1:]:
            candidate = token.lstrip("./").rstrip("/")
            if not candidate:
                continue
            for pattern in protected:
                plain = pattern[:-3] if pattern.endswith("/**") else pattern
                if _match(candidate, pattern) or candidate.casefold() == plain.casefold():
                    return Decision(DENY, "protected-shell-target", "the command names a protected path: " + plain)
                if tokens[0] in ("rm", "mv", "rmdir") and (plain.casefold().startswith(candidate.casefold() + "/")):
                    return Decision(DENY, "protected-ancestor", "removing or moving an ancestor of a protected path")
    return Decision(DEFER, "shell-host-boundary", "unclassified shell effects stay with the host sandbox and permissions")


def evaluate(op: Operation, policy: Policy) -> Decision:
    if op.kind == "write":
        for target in op.targets:
            reason = protected_reason(policy, target)
            if reason:
                return Decision(DENY, "protected-target", f"{target.path}: {reason}")
        return Decision(DEFER, "local-edit", "an ordinary edit stays with host permissions")
    if op.kind == "shell":
        return shell_decision(op.command, policy, op.branch)
    if op.kind == "mcp":
        if op.tool in policy.mcp_reads:
            return Decision(DEFER, "reviewed-mcp-read", "a reviewed read operation stays with host permissions")
        return Decision(DENY, "unreviewed-mcp", "this MCP operation is not on the cell's reviewed list (policy.mcp_reads)")
    cls_name = policy.tools.get(op.tool)
    if cls_name in DEFER_CLASSES:
        return Decision(DEFER, "tool-" + cls_name, "a " + cls_name + " tool stays with host permissions")
    if cls_name in ("write", "shell"):
        return Decision(DENY, "unparsed-" + cls_name, "this tool's targets could not be read, so it cannot be checked")
    return Decision(DENY, "unknown-tool", f"{op.tool} is not in the tool registry; classify it in the kit or the cell")


def summarize(decisions: Sequence[Decision]) -> Dict[str, int]:
    counts: Dict[str, int] = {}
    for d in decisions:
        counts[d.code] = counts.get(d.code, 0) + 1
    return counts
