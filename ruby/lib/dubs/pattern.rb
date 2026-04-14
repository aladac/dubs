# frozen_string_literal: true

module Dubs
  module Pattern
    module_function

    # Interpolate a pattern template with values.
    #
    #   Pattern.interpolate("{adjective}-{noun}-{token}", adjective: "vicious", noun: "sazabi", token: "a3f1", separator: "-")
    #   # => "vicious-sazabi-a3f1"
    #
    def interpolate(template, adjective:, noun:, token:, separator: "-")
      result = template
        .gsub("{adjective}", adjective)
        .gsub("{noun}", noun)
        .gsub("{token}", token)

      # Replace literal hyphens in the template with the configured separator
      if separator != "-"
        result = result.gsub("-", separator)
      end

      # Clean up: remove trailing/leading separators if token is empty
      if token.empty?
        result = result
          .chomp(separator)
          .delete_prefix(separator)
          .gsub("#{separator}#{separator}", separator)
      end

      result
    end

    # Resolve a pattern name to its template string.
    def resolve(pattern)
      case pattern
      when Symbol, String
        Data.pattern(pattern)
      else
        pattern.to_s
      end
    end
  end
end
