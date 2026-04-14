# frozen_string_literal: true

RSpec.describe Dubs do
  describe ".generate" do
    it "generates a name with default settings" do
      name = Dubs.generate
      expect(name).to be_a(String)
      expect(name).not_to be_empty
      expect(name).to match(/^[a-z0-9]+-[a-z0-9-]+-[a-z0-9]+$/)
    end

    it "generates a name with a specific theme" do
      name = Dubs.generate(theme: :gundam)
      expect(name).to be_a(String)
      expect(name).not_to be_empty
    end

    it "accepts theme as string" do
      name = Dubs.generate(theme: "star_trek")
      expect(name).to be_a(String)
    end

    it "generates a hex token" do
      name = Dubs.generate(theme: :gundam, token: :hex, token_length: 6)
      token = name.split("-").last
      expect(token).to match(/^[0-9a-f]{6}$/)
    end

    it "generates an uppercase hex token" do
      name = Dubs.generate(theme: :gundam, token: :hex, token_length: 6, token_case: :upper)
      token = name.split("-").last
      expect(token).to match(/^[0-9A-F]{6}$/)
    end

    it "generates with no token" do
      name = Dubs.generate(theme: :gundam, token: :none)
      parts = name.split("-")
      # codename pattern has 2 parts, but default pattern with no token also produces 2
      expect(parts.length).to be >= 2
    end

    it "uses custom separator" do
      name = Dubs.generate(theme: :gundam, separator: ".", token: :none, pattern: :codename)
      expect(name).to include(".")
      expect(name).not_to include("-")
    end

    it "generates deterministic output with seed" do
      a = Dubs.generate(theme: :gundam, seed: 42)
      b = Dubs.generate(theme: :gundam, seed: 42)
      expect(a).to eq(b)
    end

    it "generates different output with different seeds" do
      a = Dubs.generate(theme: :gundam, seed: 42)
      b = Dubs.generate(theme: :gundam, seed: 99)
      expect(a).not_to eq(b)
    end

    it "uses the designation pattern" do
      name = Dubs.generate(theme: :star_trek, pattern: :designation, token: :hex, token_length: 6, seed: 42)
      expect(name).to include("-class-")
    end

    it "uses the classified pattern" do
      name = Dubs.generate(theme: :gundam, pattern: :classified, seed: 42)
      # classified = {noun}-{adjective}-{token}, noun comes first
      parts = name.split("-")
      adjectives = Dubs::Data.adjectives(:general)
      # second part should be an adjective
      expect(adjectives).to include(parts[-2])
    end

    it "uses combat adjectives" do
      name = Dubs.generate(theme: :gundam, adjective_group: :combat, seed: 42)
      adjective = name.split("-").first
      expect(Dubs::Data.adjectives(:combat)).to include(adjective)
    end

    it "uses cosmic adjectives" do
      name = Dubs.generate(theme: :gundam, adjective_group: :cosmic, seed: 42)
      adjective = name.split("-").first
      expect(Dubs::Data.adjectives(:cosmic)).to include(adjective)
    end

    it "selects a specific category" do
      name = Dubs.generate(theme: :gundam, category: :characters, seed: 42)
      characters = Dubs.theme(:gundam).nouns(:characters)
      noun = name.split("-")[1]
      expect(characters).to include(noun)
    end

    it "raises on unknown theme" do
      expect { Dubs.generate(theme: :nonexistent) }.to raise_error(Dubs::ThemeNotFound)
    end

    it "raises on unknown category" do
      expect { Dubs.generate(theme: :gundam, category: :nonexistent) }.to raise_error(Dubs::CategoryNotFound)
    end
  end

  describe ".themes" do
    it "returns available theme names as symbols" do
      themes = Dubs.themes
      expect(themes).to include(:gundam)
      expect(themes).to include(:star_trek)
      expect(themes).to all(be_a(Symbol))
    end
  end

  describe ".theme" do
    it "returns a theme object" do
      theme = Dubs.theme(:gundam)
      expect(theme).to be_a(Dubs::Theme)
      expect(theme.name).to eq("gundam")
      expect(theme.display_name).to eq("Mobile Suit Gundam")
    end

    it "lists categories" do
      categories = Dubs.theme(:gundam).categories
      expect(categories).to include(:mobile_suits, :characters, :ships)
    end

    it "returns nouns for a category" do
      nouns = Dubs.theme(:gundam).nouns(:mobile_suits)
      expect(nouns).to include("sazabi", "zaku", "exia")
    end

    it "returns nouns for default category" do
      nouns = Dubs.theme(:gundam).nouns
      expect(nouns).to include("sazabi") # mobile_suits is default
    end

    it "raises on unknown theme" do
      expect { Dubs.theme(:nonexistent) }.to raise_error(Dubs::ThemeNotFound)
    end
  end

  describe ".register_theme" do
    after { Dubs::Data.instance_variable_set(:@themes, nil) }

    it "registers a custom theme" do
      Dubs.register_theme(
        name: :test_theme,
        display_name: "Test Theme",
        categories: {mechs: %w[atlas centurion]},
        default_category: :mechs
      )

      expect(Dubs.themes).to include(:test_theme)
      name = Dubs.generate(theme: :test_theme, seed: 0)
      expect(name).to be_a(String)
    end
  end

  describe ".register_theme_file" do
    after { Dubs::Data.instance_variable_set(:@themes, nil) }

    it "registers a theme from a JSON file" do
      Dubs.register_theme_file(File.expand_path("../../data/themes/gundam.json", __dir__))
      expect(Dubs.themes).to include(:gundam)
    end
  end
end

RSpec.describe Dubs::Token do
  describe ".generate" do
    it "generates a numeric token" do
      token = Dubs::Token.generate(type: :numeric, length: 4)
      expect(token).to match(/^\d{4}$/)
    end

    it "generates a hex token" do
      token = Dubs::Token.generate(type: :hex, length: 6)
      expect(token).to match(/^[0-9a-f]{6}$/)
    end

    it "generates an alpha token" do
      token = Dubs::Token.generate(type: :alpha, length: 4)
      expect(token).to match(/^[a-z0-9]{4}$/)
    end

    it "returns empty string for :none" do
      expect(Dubs::Token.generate(type: :none, length: 4)).to eq("")
    end

    it "respects uppercase" do
      token = Dubs::Token.generate(type: :hex, length: 4, token_case: :upper)
      expect(token).to match(/^[0-9A-F]{4}$/)
    end

    it "is deterministic with seed" do
      a = Dubs::Token.generate(type: :hex, length: 4, seed: 42)
      b = Dubs::Token.generate(type: :hex, length: 4, seed: 42)
      expect(a).to eq(b)
    end
  end
end

RSpec.describe Dubs::Pattern do
  describe ".interpolate" do
    it "interpolates default pattern" do
      result = Dubs::Pattern.interpolate(
        "{adjective}-{noun}-{token}",
        adjective: "vicious", noun: "sazabi", token: "a3f1"
      )
      expect(result).to eq("vicious-sazabi-a3f1")
    end

    it "interpolates designation pattern" do
      result = Dubs::Pattern.interpolate(
        "{adjective}-{noun}-class-{token}",
        adjective: "burning", noun: "akira", token: "EFFB1A"
      )
      expect(result).to eq("burning-akira-class-EFFB1A")
    end

    it "handles empty token in codename pattern" do
      result = Dubs::Pattern.interpolate(
        "{adjective}-{noun}",
        adjective: "dark", noun: "zaku", token: ""
      )
      expect(result).to eq("dark-zaku")
    end

    it "uses custom separator" do
      result = Dubs::Pattern.interpolate(
        "{adjective}-{noun}-{token}",
        adjective: "iron", noun: "dom", token: "42", separator: "."
      )
      expect(result).to eq("iron.dom.42")
    end
  end
end

RSpec.describe Dubs::Data do
  describe ".adjectives" do
    it "returns general adjectives" do
      adj = Dubs::Data.adjectives(:general)
      expect(adj).to be_an(Array)
      expect(adj.length).to be > 10
    end

    it "returns combat adjectives" do
      adj = Dubs::Data.adjectives(:combat)
      expect(adj).to include("tactical", "stealth")
    end

    it "raises on unknown group" do
      expect { Dubs::Data.adjectives(:nonexistent) }.to raise_error(Dubs::Error)
    end
  end

  describe ".pattern_names" do
    it "returns available patterns" do
      names = Dubs::Data.pattern_names
      expect(names).to include(:default, :classified, :designation, :codename, :serial)
    end
  end

  describe ".adjective_groups" do
    it "returns available groups" do
      groups = Dubs::Data.adjective_groups
      expect(groups).to include(:general, :combat, :cosmic)
    end
  end
end
