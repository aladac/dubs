"""Tests for dubs."""

import re
import pytest
import dubs


class TestGenerate:
    def test_default(self):
        name = dubs.generate()
        assert isinstance(name, str)
        assert len(name) > 0
        assert "-" in name

    def test_with_theme(self):
        name = dubs.generate(theme="gundam")
        assert isinstance(name, str)
        assert len(name) > 0

    def test_underscore_theme(self):
        name = dubs.generate(theme="star_trek")
        assert isinstance(name, str)

    def test_hex_token(self):
        name = dubs.generate(theme="gundam", token="hex", token_length=6)
        token = name.split("-")[-1]
        assert re.match(r"^[0-9a-f]{6}$", token)

    def test_uppercase_hex(self):
        name = dubs.generate(theme="gundam", token="hex", token_length=6, token_case="upper")
        token = name.split("-")[-1]
        assert re.match(r"^[0-9A-F]{6}$", token)

    def test_no_token(self):
        name = dubs.generate(theme="gundam", token="none", pattern="codename")
        parts = name.split("-")
        assert len(parts) == 2

    def test_custom_separator(self):
        name = dubs.generate(theme="gundam", separator=".", token="none", pattern="codename")
        assert "." in name
        assert "-" not in name

    def test_seeded_determinism(self):
        a = dubs.generate(theme="gundam", seed=42)
        b = dubs.generate(theme="gundam", seed=42)
        assert a == b

    def test_different_seeds(self):
        a = dubs.generate(theme="gundam", seed=42)
        b = dubs.generate(theme="gundam", seed=99)
        assert a != b

    def test_designation_pattern(self):
        name = dubs.generate(
            theme="star_trek", pattern="designation", token="hex", token_length=6, seed=42
        )
        assert "-class-" in name

    def test_combat_adjectives(self):
        name = dubs.generate(theme="gundam", adjective_group="combat", seed=42)
        assert len(name) > 0

    def test_specific_category(self):
        name = dubs.generate(theme="gundam", category="characters", seed=42)
        assert len(name) > 0

    def test_unknown_theme(self):
        with pytest.raises(dubs.generator.ThemeNotFound):
            dubs.generate(theme="nonexistent")

    def test_unknown_category(self):
        with pytest.raises(dubs.generator.CategoryNotFound):
            dubs.generate(theme="gundam", category="nonexistent")


class TestThemes:
    def test_list_themes(self):
        t = dubs.themes()
        assert "gundam" in t
        assert "star-trek" in t

    def test_get_theme(self):
        t = dubs.theme("gundam")
        assert t["name"] == "gundam"
        assert t["display_name"] == "Mobile Suit Gundam"
        assert "mobile_suits" in t["categories"]

    def test_unknown_theme(self):
        with pytest.raises(dubs.generator.ThemeNotFound):
            dubs.theme("nonexistent")


class TestRegisterTheme:
    def test_register_custom(self):
        dubs.register_theme(
            name="test_theme",
            display_name="Test Theme",
            categories={"mechs": ["atlas", "centurion"]},
            default_category="mechs",
        )
        assert "test-theme" in dubs.themes()
        name = dubs.generate(theme="test_theme", seed=0)
        assert isinstance(name, str)
