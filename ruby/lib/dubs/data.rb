# frozen_string_literal: true

require "json"

module Dubs
  module Data
    MUTEX = Mutex.new

    class << self
      def adjectives(group = :general)
        key = group.to_s
        adjective_data.fetch(key) { raise Error, "Unknown adjective group: #{group}" }
      end

      def adjective_groups
        adjective_data.keys.map(&:to_sym)
      end

      def patterns
        pattern_data["patterns"]
      end

      def pattern(name)
        key = name.to_s
        patterns.fetch(key) { raise PatternNotFound, "Unknown pattern: #{name}" }
      end

      def pattern_names
        patterns.keys.map(&:to_sym)
      end

      def theme(name)
        key = name.to_s.tr("_", "-")
        themes.fetch(key) { raise ThemeNotFound, "Unknown theme: #{name}. Available: #{theme_names.join(", ")}" }
      end

      def theme_names
        themes.keys.map { |k| k.tr("-", "_").to_sym }
      end

      def register_theme(name:, display_name: nil, categories:, default_category:)
        key = name.to_s.tr("_", "-")
        data = {
          "name" => key,
          "display_name" => display_name || name.to_s,
          "categories" => categories.transform_keys(&:to_s).transform_values { |v| v.map(&:to_s) },
          "default_category" => default_category.to_s
        }
        MUTEX.synchronize { themes[key] = Theme.new(data) }
      end

      def register_theme_file(path)
        data = JSON.parse(File.read(path))
        MUTEX.synchronize { themes[data["name"]] = Theme.new(data) }
      end

      def random_theme
        themes.values.sample
      end

      private

      def themes
        @themes ||= load_themes
      end

      def adjective_data
        @adjective_data ||= load_json("adjectives.json")
      end

      def pattern_data
        @pattern_data ||= load_json("patterns.json")
      end

      def load_themes
        dir = File.join(data_dir, "themes")
        Dir[File.join(dir, "*.json")].each_with_object({}) do |path, hash|
          data = JSON.parse(File.read(path))
          hash[data["name"]] = Theme.new(data)
        end
      end

      def load_json(filename)
        JSON.parse(File.read(File.join(data_dir, filename)))
      end

      def data_dir
        @data_dir ||= begin
          gem_data = File.expand_path("../../data", __dir__)
          repo_data = File.expand_path("../../../data", __dir__)
          if Dir.exist?(gem_data) && Dir[File.join(gem_data, "*.json")].any?
            gem_data
          elsif Dir.exist?(repo_data)
            repo_data
          else
            raise Error, "Cannot find dubs data directory"
          end
        end
      end
    end
  end
end
