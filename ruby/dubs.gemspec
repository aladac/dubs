# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "dubs"
  spec.version = "0.1.4"
  spec.authors = ["Adam Ladachowski"]
  spec.email = ["adam@saiden.pl"]

  spec.summary = "Themed name generator — like haikunator, but with categories"
  spec.description = "Generate random names from themed word lists (Gundam, Star Trek, Star Wars, Transformers, and more) with configurable patterns and token formats."
  spec.homepage = "https://github.com/aladac/dubs"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = "#{spec.homepage}/blob/main/CHANGELOG.md"

  spec.files = Dir["lib/**/*.rb", "data/**/*.json", "LICENSE", "README.md"]
  spec.require_paths = ["lib"]
end
