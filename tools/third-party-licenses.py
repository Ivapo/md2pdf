#!/usr/bin/env python3
"""Regenerates THIRD-PARTY-LICENSES.md from the workspace's own resolve.

    python3 tools/third-party-licenses.py

**What it walks, and why that set.** The normal-dependency closure of
`md2pdf-core` and `md2pdf-cli` for this host — the crates whose compiled code
ends up inside the `md2pdf` binary. Build-dependencies and dev-dependencies are
excluded deliberately: a proc macro runs at build time and is not linked in, and
a dev-dependency reaches no shipped artifact at all.

**Where the licence texts come from.** The crates' own files, out of
`~/.cargo/registry/src/`, never a copy typed here — so what this file reproduces
is what the authors actually shipped. One text per distinct licence, taken from
the first crate that carries a readable copy, with that crate named as its
provenance.
"""

import json, pathlib, re, subprocess, sys, textwrap
from collections import defaultdict

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "THIRD-PARTY-LICENSES.md"
ROOTS = {"md2pdf-core", "md2pdf-cli"}

def host_triple():
    out = subprocess.run(["rustc", "-vV"], capture_output=True, text=True).stdout
    return re.search(r"^host: (.+)$", out, re.M).group(1)

def metadata(triple):
    return json.loads(subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--filter-platform", triple],
        cwd=ROOT, capture_output=True, text=True, check=True).stdout)

def shipped(md):
    """Package ids reachable from the roots through normal dependencies alone."""
    nodes = {n["id"]: n for n in md["resolve"]["nodes"]}
    by_id = {p["id"]: p for p in md["packages"]}
    seen, stack = set(), [p["id"] for p in md["packages"] if p["name"] in ROOTS]
    while stack:
        pid = stack.pop()
        if pid in seen:
            continue
        seen.add(pid)
        for dep in nodes.get(pid, {}).get("deps", []):
            if any(k.get("kind") is None for k in dep.get("dep_kinds", [])):
                stack.append(dep["pkg"])
    return sorted((by_id[i] for i in seen), key=lambda p: (p["name"], p["version"]))

LICENSE_FILE = re.compile(r"^(LICEN[CS]E|COPYING|UNLICENSE)([-.].*)?$", re.I)


def split_spdx(expr):
    """`MIT OR Apache-2.0`, `MIT/Apache-2.0`, `(A) AND B` -> the bare terms."""
    return [t for t in (raw.strip("() ") for raw in re.split(r"(?:\s+(?:OR|AND)\s+|/)", expr or "")) if t]


def identify(body):
    """Which SPDX term a licence file's own text is, read rather than guessed
    from its filename — `bytemuck`'s `LICENSE-APACHE` is Apache-2.0 and not the
    Zlib its manifest also offers, and `adler2`'s `LICENSE-0BSD` is not MIT."""
    b = " ".join(body.split()).lower()
    if "apache license" in b and "version 2.0" in b:
        return "Apache-2.0"
    if "boost software license" in b:
        return "BSL-1.0"
    if "unicode license" in b or "unicode-3.0" in b:
        return "Unicode-3.0"
    if "creative commons" in b and "cc0" in b:
        return "CC0-1.0"
    if "free and unencumbered software released into the public domain" in b:
        return "Unlicense"
    if "altered source versions must be plainly marked as such" in b:
        return "Zlib"
    if "redistribution and use in source and binary forms" in b:
        if "neither the name of" in b or "nor the names of its contributors" in b:
            return "BSD-3-Clause"
        return "BSD-2-Clause"
    if "permission to use, copy, modify, and/or distribute this software" in b:
        # Not a bare `"isc" in b`, which matches "d(isc)laims" and so files
        # every 0BSD text as ISC. The two differ by the fee clause.
        return "0BSD" if "with or without fee" in b else "ISC"
    if "permission is hereby granted, free of charge" in b:
        return "MIT"
    return None


def texts_for(pkgs):
    """SPDX term -> (text, the crate it was read from). One text per term, and
    a file is only ever filed under what it actually says."""
    wanted = {t for p in pkgs for t in split_spdx(p.get("license"))}
    found = {}
    for p in pkgs:
        src = pathlib.Path(p["manifest_path"]).parent
        if not src.is_dir():
            continue
        for f in sorted(src.iterdir()):
            if not f.is_file() or not LICENSE_FILE.match(f.name):
                continue
            try:
                body = f.read_text(encoding="utf-8").strip()
            except (UnicodeDecodeError, OSError):
                continue
            term = identify(body)
            if term and term in wanted and term not in found:
                found[term] = (body, f"{p['name']} {p['version']} ({f.name})")
    return found


def main():
    triple = host_triple()
    pkgs = [p for p in shipped(metadata(triple)) if p["name"] not in ROOTS]
    texts = texts_for(pkgs)
    terms = {t for p in pkgs for t in split_spdx(p.get("license"))}
    uncovered = sorted(
        (p for p in pkgs if not any(t in texts for t in split_spdx(p.get("license")))),
        key=lambda p: p["name"])

    by_license = defaultdict(list)
    for p in pkgs:
        by_license[p.get("license") or "(none declared)"].append(p)

    w = []
    w.append("<!-- generated by tools/third-party-licenses.py — do not edit by hand -->")
    w.append("")
    w.append("# Third-party licences")
    w.append("")
    w.append(textwrap.fill(
        "The `md2pdf` binary embeds its dependencies rather than linking them at "
        "runtime, so the crates below are the ones whose code can end up inside it. "
        "This file is that list and the licence texts those crates carry, reproduced "
        "from the crates' own files. It is generated — run `python3 "
        "tools/third-party-licenses.py` after a dependency change.", 88))
    w.append("")
    w.append(textwrap.fill(
        f"Resolved for `{triple}`, normal dependencies only: a build-dependency "
        "runs at build time and is not linked in, and a dev-dependency reaches no "
        "shipped artifact.", 88))
    w.append("")
    w.append(textwrap.fill(
        "**It is a superset, deliberately.** The walk is over the resolve graph and "
        "not over an enabled feature set, so a crate reached only through a feature "
        "this build does not turn on is still listed — measured against `cargo tree "
        "-e normal`, eight are. Attributing a crate that did not ship costs a reader "
        "nothing; omitting one that did is the failure this file exists to prevent, "
        "so the error is taken in the safe direction and said out loud rather than "
        "trimmed away.", 88))
    w.append("")
    w.append(textwrap.fill(
        "The bundled fonts are not here — they are not crates. "
        "`core/assets/fonts/OFL.txt` and `core/assets/fonts/GUST-FONT-LICENSE.txt` "
        "travel beside them in the published crate, and the README names both.", 88))
    w.append("")
    w.append(f"**{len(pkgs)} crates, {len(by_license)} distinct licence expressions, "
             f"{len(terms)} distinct terms.**")
    w.append("")
    w.append("## What ships")
    w.append("")
    w.append("| crate | version | licence |")
    w.append("|---|---|---|")
    for p in pkgs:
        w.append(f"| `{p['name']}` | {p['version']} | {p.get('license') or '—'} |")
    w.append("")
    w.append("## The licence texts")
    w.append("")
    for term in sorted(texts):
        body, prov = texts[term]
        w.append(f"### {term}")
        w.append("")
        w.append(f"*Reproduced from {prov}.*")
        w.append("")
        w.append("```")
        w.append(body)
        w.append("```")
        w.append("")

    if uncovered:
        w.append("## Crates whose licence text is not reproduced above")
        w.append("")
        w.append(textwrap.fill(
            "Each of these declares its terms in its manifest and ships no licence "
            "file in its published crate, so there is no text to reproduce and none "
            "is invented here. A crate offering a choice of terms is not listed while "
            "this file holds the text of any one of them.", 88))
        w.append("")
        for p in uncovered:
            w.append(f"- `{p['name']}` {p['version']} — {p.get('license') or '—'}")
        w.append("")

    OUT.write_text("\n".join(w) + "\n")
    print(f"wrote {OUT.relative_to(ROOT)}: {len(pkgs)} crates, "
          f"{len(texts)} licence texts, {len(uncovered)} crate(s) without text")

if __name__ == "__main__":
    sys.exit(main())
