"""Core generator for dubs."""

from __future__ import annotations

import json
import secrets
from pathlib import Path
from typing import Any

# Errors


class DubsError(Exception):
    pass


class ThemeNotFound(DubsError):
    pass


class CategoryNotFound(DubsError):
    pass


# Data loading

_data_dir: Path | None = None
_adjectives: dict[str, list[str]] | None = None
_patterns: dict[str, str] | None = None
_themes: dict[str, dict[str, Any]] | None = None


def _find_data_dir() -> Path:
    # Check relative to this file: ../../data (repo layout) or ../data (installed)
    here = Path(__file__).parent
    for candidate in [
        here / "data",
        here.parent / "data",
        here.parent.parent / "data",
        here.parent.parent.parent / "data",
    ]:
        if (candidate / "adjectives.json").exists():
            return candidate
    raise DubsError("Cannot find dubs data directory")


def _get_data_dir() -> Path:
    global _data_dir
    if _data_dir is None:
        _data_dir = _find_data_dir()
    return _data_dir


def _get_adjectives() -> dict[str, list[str]]:
    global _adjectives
    if _adjectives is None:
        _adjectives = json.loads((_get_data_dir() / "adjectives.json").read_text())
    return _adjectives


def _get_patterns() -> dict[str, str]:
    global _patterns
    if _patterns is None:
        data = json.loads((_get_data_dir() / "patterns.json").read_text())
        _patterns = data["patterns"]
    return _patterns


def _get_themes() -> dict[str, dict[str, Any]]:
    global _themes
    if _themes is None:
        _themes = {}
        themes_dir = _get_data_dir() / "themes"
        for path in themes_dir.glob("*.json"):
            data = json.loads(path.read_text())
            _themes[data["name"]] = data
    return _themes


# Token generation


def _generate_token(
    token_type: str = "numeric",
    length: int = 4,
    token_case: str = "lower",
    seed: int | None = None,
) -> str:
    if token_type == "none" or length <= 0:
        return ""

    hex_chars = "0123456789abcdef"
    alpha_chars = "abcdefghijklmnopqrstuvwxyz0123456789"

    if token_type == "hex":
        if seed is not None:
            raw = "".join(hex_chars[(seed + i * 7) % len(hex_chars)] for i in range(length))
        else:
            raw = secrets.token_hex(length)[:length]
    elif token_type == "numeric":
        if seed is not None:
            raw = str(seed % (10**length)).zfill(length)
        else:
            raw = str(secrets.randbelow(10**length)).zfill(length)
    elif token_type == "alpha":
        if seed is not None:
            raw = "".join(alpha_chars[(seed + i * 7) % len(alpha_chars)] for i in range(length))
        else:
            raw = "".join(secrets.choice(alpha_chars) for _ in range(length))
    else:
        raise DubsError(f"Unknown token type: {token_type}")

    return raw.upper() if token_case == "upper" else raw.lower()


# Pattern interpolation


def _interpolate(
    template: str, adjective: str, noun: str, token: str, separator: str = "-"
) -> str:
    result = template.replace("{adjective}", adjective).replace("{noun}", noun).replace("{token}", token)

    if separator != "-":
        result = result.replace("-", separator)

    if not token:
        result = result.strip(separator).replace(separator + separator, separator)

    return result


# Public API


def generate(
    *,
    theme: str | None = None,
    category: str | None = None,
    pattern: str = "default",
    token: str = "numeric",
    token_length: int = 4,
    token_case: str = "lower",
    separator: str = "-",
    adjective_group: str = "general",
    seed: int | None = None,
) -> str:
    """Generate a random themed name.

    >>> import dubs
    >>> dubs.generate(theme="gundam", seed=42)
    'shining-barbatos-0042'
    >>> dubs.generate(theme="star_trek", pattern="designation", token="hex", token_length=6, seed=42)
    'shining-akira-class-c50c9e'
    """
    # Resolve theme
    all_themes = _get_themes()
    if theme:
        key = theme.replace("_", "-")
        theme_data = all_themes.get(key)
        if not theme_data:
            available = ", ".join(all_themes.keys())
            raise ThemeNotFound(f"Theme not found: {theme}. Available: {available}")
    else:
        theme_data = secrets.choice(list(all_themes.values()))

    # Pick adjective
    adjs = _get_adjectives().get(adjective_group)
    if not adjs:
        raise DubsError(f"Unknown adjective group: {adjective_group}")
    adj = adjs[seed % len(adjs)] if seed is not None else secrets.choice(adjs)

    # Pick noun
    cat = category or theme_data["default_category"]
    nouns = theme_data["categories"].get(cat)
    if not nouns:
        available = ", ".join(theme_data["categories"].keys())
        raise CategoryNotFound(f"Category not found: {cat}. Available: {available}")
    if seed is not None:
        noun = nouns[(seed // len(adjs)) % len(nouns)]
    else:
        noun = secrets.choice(nouns)

    # Generate token
    tok = _generate_token(token, token_length, token_case, seed)

    # Resolve pattern
    template = _get_patterns().get(pattern, pattern)

    return _interpolate(template, adj, noun, tok, separator)


def themes() -> list[str]:
    """List available theme names."""
    return list(_get_themes().keys())


def theme(name: str) -> dict[str, Any]:
    """Get theme details."""
    key = name.replace("_", "-")
    data = _get_themes().get(key)
    if not data:
        raise ThemeNotFound(f"Theme not found: {name}")
    return {
        "name": data["name"],
        "display_name": data["display_name"],
        "categories": list(data["categories"].keys()),
        "default_category": data["default_category"],
    }


def register_theme(
    *,
    name: str,
    display_name: str | None = None,
    categories: dict[str, list[str]],
    default_category: str,
) -> None:
    """Register a custom theme."""
    key = name.replace("_", "-")
    _get_themes()[key] = {
        "name": key,
        "display_name": display_name or name,
        "categories": categories,
        "default_category": default_category,
    }


def register_theme_file(path: str | Path) -> None:
    """Register a custom theme from a JSON file."""
    data = json.loads(Path(path).read_text())
    _get_themes()[data["name"]] = data
