import { describe, it, expect } from "vitest";
import {
  generate,
  listThemes,
  getTheme,
  registerTheme,
} from "../src/index.js";

describe("generate", () => {
  it("generates a name with defaults", () => {
    const name = generate();
    expect(name).toBeTruthy();
    expect(name).toContain("-");
  });

  it("generates with a specific theme", () => {
    const name = generate({ theme: "gundam" });
    expect(name).toBeTruthy();
  });

  it("accepts underscore theme names", () => {
    const name = generate({ theme: "star_trek" });
    expect(name).toBeTruthy();
  });

  it("generates hex tokens", () => {
    const name = generate({ theme: "gundam", token: "hex", tokenLength: 6 });
    const token = name.split("-").pop()!;
    expect(token).toMatch(/^[0-9a-f]{6}$/);
  });

  it("generates uppercase hex tokens", () => {
    const name = generate({
      theme: "gundam",
      token: "hex",
      tokenLength: 6,
      tokenCase: "upper",
    });
    const token = name.split("-").pop()!;
    expect(token).toMatch(/^[0-9A-F]{6}$/);
  });

  it("generates with no token", () => {
    const name = generate({
      theme: "gundam",
      token: "none",
      pattern: "codename",
    });
    const parts = name.split("-");
    expect(parts.length).toBe(2);
  });

  it("uses custom separator", () => {
    const name = generate({
      theme: "gundam",
      separator: ".",
      token: "none",
      pattern: "codename",
    });
    expect(name).toContain(".");
    expect(name).not.toContain("-");
  });

  it("is deterministic with seed", () => {
    const a = generate({ theme: "gundam", seed: 42 });
    const b = generate({ theme: "gundam", seed: 42 });
    expect(a).toBe(b);
  });

  it("produces different output with different seeds", () => {
    const a = generate({ theme: "gundam", seed: 42 });
    const b = generate({ theme: "gundam", seed: 99 });
    expect(a).not.toBe(b);
  });

  it("uses designation pattern", () => {
    const name = generate({
      theme: "star_trek",
      pattern: "designation",
      token: "hex",
      tokenLength: 6,
      seed: 42,
    });
    expect(name).toContain("-class-");
  });

  it("uses combat adjectives", () => {
    const name = generate({
      theme: "gundam",
      adjectiveGroup: "combat",
      seed: 42,
    });
    expect(name).toBeTruthy();
  });

  it("selects a specific category", () => {
    const name = generate({
      theme: "gundam",
      category: "characters",
      seed: 42,
    });
    expect(name).toBeTruthy();
  });

  it("throws on unknown theme", () => {
    expect(() => generate({ theme: "nonexistent" })).toThrow("Theme not found");
  });

  it("throws on unknown category", () => {
    expect(() =>
      generate({ theme: "gundam", category: "nonexistent" })
    ).toThrow("Category not found");
  });
});

describe("listThemes", () => {
  it("returns available themes", () => {
    const themes = listThemes();
    expect(themes).toContain("gundam");
    expect(themes).toContain("star-trek");
  });
});

describe("getTheme", () => {
  it("returns theme details", () => {
    const theme = getTheme("gundam");
    expect(theme.name).toBe("gundam");
    expect(theme.displayName).toBe("Mobile Suit Gundam");
    expect(theme.categories).toContain("mobile_suits");
  });

  it("throws on unknown theme", () => {
    expect(() => getTheme("nonexistent")).toThrow("Theme not found");
  });
});

describe("registerTheme", () => {
  it("registers a custom theme", () => {
    registerTheme({
      name: "test-theme",
      display_name: "Test Theme",
      categories: { mechs: ["atlas", "centurion"] },
      default_category: "mechs",
    });

    expect(listThemes()).toContain("test-theme");
    const name = generate({ theme: "test-theme", seed: 0 });
    expect(name).toBeTruthy();
  });
});
