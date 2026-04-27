#!/usr/bin/env python3
"""Validate README local links and catalog official links."""

from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOCAL_LINK_RE = re.compile(r"\[([^\]]+)\]\((?!https?://|mailto:|#)([^)]+)\)")
URL_RE = re.compile(r"https?://[^\s>)]+")


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def catalog_urls() -> list[str]:
    urls: list[str] = []
    for path in (ROOT / "catalog").glob("*.json"):
        data = load_json(path)
        if isinstance(data, list):
            for item in data:
                urls.extend(item.get("official_docs", []))
                if item.get("official_project_url"):
                    urls.append(item["official_project_url"])
    return sorted(set(urls))


def markdown_urls() -> list[str]:
    urls: list[str] = []
    for path in [ROOT / "README.md", *ROOT.glob("mcp/**/*.md"), *ROOT.glob("docs/**/*.md")]:
        if path.exists():
            urls.extend(URL_RE.findall(path.read_text(encoding="utf-8", errors="ignore")))
    return sorted(set(urls))


def validate_local_readme_links(errors: list[str]) -> None:
    readme = ROOT / "README.md"
    text = readme.read_text(encoding="utf-8")
    for _, raw in LOCAL_LINK_RE.findall(text):
        target = raw.split("#", 1)[0]
        if not target:
            continue
        if not (ROOT / target).exists():
            errors.append(f"README local link does not exist: {raw}")


def check_url(url: str, errors: list[str]) -> None:
    req = urllib.request.Request(url, method="HEAD", headers={"User-Agent": "vanguard-frontier-agentic-link-check/1.0"})
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            if resp.status >= 400:
                errors.append(f"{url}: HTTP {resp.status}")
    except Exception:
        # Some sites reject HEAD. Retry GET with a small range.
        req = urllib.request.Request(url, method="GET", headers={"User-Agent": "vanguard-frontier-agentic-link-check/1.0", "Range": "bytes=0-1024"})
        try:
            with urllib.request.urlopen(req, timeout=15) as resp:
                if resp.status >= 400:
                    errors.append(f"{url}: HTTP {resp.status}")
        except Exception as exc:
            errors.append(f"{url}: {exc}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--offline", action="store_true", help="Only validate local links and URL syntax.")
    args = parser.parse_args()

    errors: list[str] = []
    validate_local_readme_links(errors)
    urls = sorted(set(catalog_urls() + markdown_urls()))
    for url in urls:
        if not URL_RE.match(url):
            errors.append(f"invalid URL syntax: {url}")
        elif not args.offline:
            check_url(url, errors)

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    mode = "offline" if args.offline else "online"
    print(f"OK: validated README links and {len(urls)} URLs ({mode})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
