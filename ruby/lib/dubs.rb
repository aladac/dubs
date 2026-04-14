# frozen_string_literal: true

require_relative "dubs/version"
require_relative "dubs/data"
require_relative "dubs/theme"
require_relative "dubs/token"
require_relative "dubs/pattern"
require_relative "dubs/generator"

module Dubs
  class Error < StandardError; end
  class ThemeNotFound < Error; end
  class CategoryNotFound < Error; end
  class PatternNotFound < Error; end

  module_function

  # Generate a random name.
  #
  #   Dubs.generate
  #   # => "vicious-sazabi-4271"
  #
  #   Dubs.generate(theme: :gundam, token: :hex, token_length: 6)
  #   # => "crimson-exia-b7e2a1"
  #
  def generate(**options)
    Generator.new(**options).generate
  end

  # List available theme names.
  #
  #   Dubs.themes
  #   # => [:gundam, :star_trek]
  #
  def themes
    Data.theme_names
  end

  # Get a theme by name.
  #
  #   Dubs.theme(:gundam).categories
  #   # => [:mobile_suits, :characters, :ships]
  #
  def theme(name)
    Data.theme(name)
  end

  # Register a custom theme from a hash.
  #
  #   Dubs.register_theme(name: :battletech, display_name: "BattleTech", categories: { ... }, default_category: "mechs")
  #
  def register_theme(**data)
    Data.register_theme(**data)
  end

  # Register a custom theme from a JSON file.
  #
  #   Dubs.register_theme_file("/path/to/battletech.json")
  #
  def register_theme_file(path)
    Data.register_theme_file(path)
  end
end
