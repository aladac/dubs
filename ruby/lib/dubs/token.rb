# frozen_string_literal: true

require "securerandom"

module Dubs
  module Token
    HEX_CHARS = ("0".."9").to_a + ("a".."f").to_a
    ALPHA_CHARS = ("a".."z").to_a + ("0".."9").to_a

    module_function

    def generate(type: :numeric, length: 4, token_case: :lower, seed: nil)
      return "" if type.to_sym == :none || length <= 0

      raw = case type.to_sym
      when :hex
        if seed
          seeded_chars(HEX_CHARS, length, seed)
        else
          SecureRandom.hex(length)[0, length]
        end
      when :numeric
        if seed
          seeded_numeric(length, seed)
        else
          SecureRandom.random_number(10**length).to_s.rjust(length, "0")
        end
      when :alpha
        if seed
          seeded_chars(ALPHA_CHARS, length, seed)
        else
          Array.new(length) { ALPHA_CHARS.sample }.join
        end
      else
        raise Error, "Unknown token type: #{type}. Use :hex, :numeric, :alpha, or :none"
      end

      token_case.to_sym == :upper ? raw.upcase : raw.downcase
    end

    def seeded_chars(chars, length, seed)
      Array.new(length) { |i| chars[(seed + i * 7) % chars.length] }.join
    end

    def seeded_numeric(length, seed)
      (seed % (10**length)).to_s.rjust(length, "0")
    end

    private_class_method :seeded_chars, :seeded_numeric
  end
end
