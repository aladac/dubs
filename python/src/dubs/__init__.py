"""dubs — Themed name generator. Like haikunator, but with categories."""

__version__ = "0.1.1"

from dubs.generator import generate, themes, theme, register_theme, register_theme_file

__all__ = [
    "__version__",
    "generate",
    "themes",
    "theme",
    "register_theme",
    "register_theme_file",
]
