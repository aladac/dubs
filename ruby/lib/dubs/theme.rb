# frozen_string_literal: true

module Dubs
  class Theme
    attr_reader :name, :display_name, :default_category

    def initialize(data)
      @name = data["name"]
      @display_name = data["display_name"]
      @categories_data = data["categories"]
      @default_category = data["default_category"]
    end

    def categories
      @categories_data.keys.map(&:to_sym)
    end

    def nouns(category = nil)
      cat = (category || default_category).to_s
      @categories_data.fetch(cat) do
        raise CategoryNotFound, "Unknown category: #{cat} for theme #{name}. Available: #{categories.join(", ")}"
      end
    end

    def random_noun(category = nil)
      nouns(category).sample
    end

    def to_sym
      name.tr("-", "_").to_sym
    end

    def inspect
      "#<Dubs::Theme #{name} categories=#{categories}>"
    end
  end
end
