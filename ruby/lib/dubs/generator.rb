# frozen_string_literal: true

module Dubs
  class Generator
    DEFAULTS = {
      theme: nil,
      category: nil,
      pattern: :default,
      token: :numeric,
      token_length: 4,
      token_case: :lower,
      separator: "-",
      adjective_group: :general,
      seed: nil
    }.freeze

    def initialize(**options)
      @options = DEFAULTS.merge(options)
    end

    def generate
      theme = resolve_theme
      adjective = pick_adjective
      noun = pick_noun(theme)
      token_value = generate_token

      template = Pattern.resolve(@options[:pattern])
      Pattern.interpolate(template, adjective: adjective, noun: noun, token: token_value, separator: @options[:separator])
    end

    private

    def resolve_theme
      name = @options[:theme]
      name ? Data.theme(name) : Data.random_theme
    end

    def pick_adjective
      adjectives = Data.adjectives(@options[:adjective_group])
      if @options[:seed]
        adjectives[@options[:seed] % adjectives.length]
      else
        adjectives.sample
      end
    end

    def pick_noun(theme)
      nouns = theme.nouns(@options[:category])
      if @options[:seed]
        adj_count = Data.adjectives(@options[:adjective_group]).length
        nouns[(@options[:seed] / adj_count) % nouns.length]
      else
        nouns.sample
      end
    end

    def generate_token
      Token.generate(
        type: @options[:token],
        length: @options[:token_length],
        token_case: @options[:token_case],
        seed: @options[:seed]
      )
    end
  end
end
