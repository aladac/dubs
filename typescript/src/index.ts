import { readFileSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

// Types
export type TokenType = "hex" | "numeric" | "alpha" | "none";
export type TokenCase = "lower" | "upper";
export type PatternName =
  | "default"
  | "classified"
  | "designation"
  | "codename"
  | "serial";
export type AdjectiveGroup = "general" | "combat" | "cosmic";

export interface GenerateOptions {
  theme?: string;
  category?: string;
  pattern?: PatternName | string;
  token?: TokenType;
  tokenLength?: number;
  tokenCase?: TokenCase;
  separator?: string;
  adjectiveGroup?: AdjectiveGroup;
  seed?: number;
}

interface ThemeData {
  name: string;
  display_name: string;
  categories: Record<string, string[]>;
  default_category: string;
}

interface AdjectiveData {
  general: string[];
  combat: string[];
  cosmic: string[];
  [key: string]: string[];
}

interface PatternData {
  patterns: Record<string, string>;
  default_pattern: string;
}

// Data loading
const __dirname = dirname(fileURLToPath(import.meta.url));

function dataDir(): string {
  // In dist: ../data (bundled), or ../../data (repo root)
  const bundled = join(__dirname, "..", "data");
  const repo = join(__dirname, "..", "..", "data");
  try {
    readFileSync(join(bundled, "adjectives.json"));
    return bundled;
  } catch {
    return repo;
  }
}

const DATA_DIR = dataDir();

function loadJSON<T>(filename: string): T {
  return JSON.parse(readFileSync(join(DATA_DIR, filename), "utf-8"));
}

const adjectives: AdjectiveData = loadJSON("adjectives.json");
const patternData: PatternData = loadJSON("patterns.json");

const themeRegistry = new Map<string, ThemeData>();

// Load built-in themes
for (const file of ["gundam.json", "star-trek.json"]) {
  const theme: ThemeData = loadJSON(join("themes", file));
  themeRegistry.set(theme.name, theme);
}

// Token generation
function generateToken(
  type: TokenType,
  length: number,
  tokenCase: TokenCase,
  seed?: number
): string {
  if (type === "none" || length <= 0) return "";

  const hexChars = "0123456789abcdef";
  const alphaChars = "abcdefghijklmnopqrstuvwxyz0123456789";

  let raw: string;

  switch (type) {
    case "hex":
      if (seed !== undefined) {
        raw = seededChars(hexChars, length, seed);
      } else {
        raw = Array.from({ length }, () =>
          hexChars[Math.floor(Math.random() * hexChars.length)]
        ).join("");
      }
      break;
    case "numeric":
      if (seed !== undefined) {
        raw = String(seed % Math.pow(10, length)).padStart(length, "0");
      } else {
        raw = String(Math.floor(Math.random() * Math.pow(10, length))).padStart(
          length,
          "0"
        );
      }
      break;
    case "alpha":
      if (seed !== undefined) {
        raw = seededChars(alphaChars, length, seed);
      } else {
        raw = Array.from({ length }, () =>
          alphaChars[Math.floor(Math.random() * alphaChars.length)]
        ).join("");
      }
      break;
  }

  return tokenCase === "upper" ? raw.toUpperCase() : raw.toLowerCase();
}

function seededChars(chars: string, length: number, seed: number): string {
  return Array.from(
    { length },
    (_, i) => chars[(seed + i * 7) % chars.length]
  ).join("");
}

// Pattern interpolation
function interpolate(
  template: string,
  adj: string,
  noun: string,
  token: string,
  separator: string
): string {
  let result = template
    .replace("{adjective}", adj)
    .replace("{noun}", noun)
    .replace("{token}", token);

  if (separator !== "-") {
    result = result.replaceAll("-", separator);
  }

  if (token === "") {
    result = result
      .replace(new RegExp(`^\\${separator}|\\${separator}$`, "g"), "")
      .replaceAll(separator + separator, separator);
  }

  return result;
}

// Public API

/**
 * Generate a random themed name.
 *
 * @example
 * generate()                                    // "vicious-sazabi-4271"
 * generate({ theme: "gundam", token: "hex" })   // "crimson-exia-b7e2"
 * generate({ theme: "star-trek", pattern: "designation", token: "hex", tokenLength: 6, tokenCase: "upper" })
 * // "burning-akira-class-EFFB1A"
 */
export function generate(options: GenerateOptions = {}): string {
  const {
    theme: themeName,
    category,
    pattern = "default",
    token = "numeric",
    tokenLength = 4,
    tokenCase = "lower",
    separator = "-",
    adjectiveGroup = "general",
    seed,
  } = options;

  // Resolve theme
  let themeData: ThemeData;
  if (themeName) {
    const key = themeName.replace(/_/g, "-");
    const found = themeRegistry.get(key);
    if (!found) {
      throw new Error(
        `Theme not found: ${themeName}. Available: ${[...themeRegistry.keys()].join(", ")}`
      );
    }
    themeData = found;
  } else {
    const keys = [...themeRegistry.keys()];
    themeData = themeRegistry.get(
      keys[Math.floor(Math.random() * keys.length)]
    )!;
  }

  // Pick adjective
  const adjs = adjectives[adjectiveGroup];
  if (!adjs) throw new Error(`Unknown adjective group: ${adjectiveGroup}`);
  const adj =
    seed !== undefined ? adjs[seed % adjs.length] : adjs[Math.floor(Math.random() * adjs.length)];

  // Pick noun
  const cat = category || themeData.default_category;
  const nouns = themeData.categories[cat];
  if (!nouns)
    throw new Error(
      `Category not found: ${cat}. Available: ${Object.keys(themeData.categories).join(", ")}`
    );
  const noun =
    seed !== undefined
      ? nouns[Math.floor(seed / adjs.length) % nouns.length]
      : nouns[Math.floor(Math.random() * nouns.length)];

  // Generate token
  const tok = generateToken(token, tokenLength, tokenCase, seed);

  // Resolve pattern
  const template = patternData.patterns[pattern] || pattern;

  return interpolate(template, adj, noun, tok, separator);
}

/** List available theme names. */
export function listThemes(): string[] {
  return [...themeRegistry.keys()];
}

/** Get theme details. */
export function getTheme(name: string): {
  name: string;
  displayName: string;
  categories: string[];
} {
  const key = name.replace(/_/g, "-");
  const theme = themeRegistry.get(key);
  if (!theme) throw new Error(`Theme not found: ${name}`);
  return {
    name: theme.name,
    displayName: theme.display_name,
    categories: Object.keys(theme.categories),
  };
}

/** Register a custom theme. */
export function registerTheme(theme: ThemeData): void {
  themeRegistry.set(theme.name, theme);
}
