#!/usr/bin/env ruby
# encoding: utf-8
# check-counts.rb — no count is stated in prose unless it is generated.
#
# WHY THIS EXISTS (claims.yaml#COUNTS-ARE-GENERATED)
#   Every count this repository stated in prose went stale, repeatedly, within hours. The owner's
#   verdict after watching it happen again: "those numbers should be dynamic, and not stored in
#   static files that simply get stale within a few iterations of effort."
#
#   Note 060 had already diagnosed it once, for one file, and the lesson did not execute anywhere
#   else. This is the executor. Two halves, and the second is the one that matters:
#
#     1. VERIFY   every generated marker matches tools/stats.rb
#     2. FORBID   any bare count in a risky phrasing, so a stale number cannot be REINTRODUCED
#
#   Half 1 alone would be useless: nothing stopped the next writer from typing "156 claims" beside
#   the marker. The repo's failure mode is not wrong numbers, it is hand-written numbers.
#
# MARKER SYNTAX
#   <!--s:claims_total-->156<!--/s-->
#   Keys are whatever tools/stats.rb emits. index.json is handled structurally instead, since JSON
#   has no comments: its `registry_state` block is overwritten from stats.
#
# USAGE
#   ruby tools/check-counts.rb            # report
#   ruby tools/check-counts.rb --strict   # exit 1 on any violation (checker mode)
#   ruby tools/check-counts.rb --fix      # rewrite markers + index.json from stats, then re-verify
#
# SCOPE
#   Live documents only. research/notes/** is immutable by convention and FAILURES.md is
#   append-only, so both record what was true when written and are never rewritten here.

require 'json'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

STATS = JSON.parse(`ruby #{File.join(__dir__, 'stats.rb')} --json`)
FIX    = ARGV.include?('--fix')
STRICT = ARGV.include?('--strict')

# Immutable or historical by convention — never rewritten, never scanned for bare counts.
EXEMPT = lambda do |path|
  path.start_with?('research/notes/') ||
    path == 'FAILURES.md' ||
    path.start_with?('ul-forge/target/', 'tools/') ||
    path.include?('/node_modules/') ||
    path == 'research/surveys/research-register.md'   # a record of past threads
end

LIVE = (Dir.glob('**/*.{md,txt}') + ['index.json']).uniq.reject { |p| EXEMPT.call(p) }.select { |p| File.file?(p) }

# A bare count is only flagged in phrasings that have actually gone stale here. Deliberately
# narrow: "191 constructions", "12/12", "Z/2" and "89-94" are results, not inventories.
# `(?<![-\w])` keeps "priority-0 claim" and "Priority-0 entries" out: those are labels, not
# inventories. The number must also be 2+ or multi-digit, since "0 entries" is never a count here.
N = /(?<![-\w])(\d{2,}|[2-9])/
FORBIDDEN = [
  [/#{N}\s+claims?\b/i,                        'claims_total'],
  [/#{N}\s+(?:of\s+\d+\s+)?VERIFIED\b/,       'claims_verified'],
  [/\bVERIFIED\s*[—-]\s*#{N}\b/,              'claims_verified'],
  [/#{N}\s+(?:of\s+)?\d*\s*checkers?\b/i,     'checkers'],
  [/\b(?:six|seven|eight|nine|ten)\s+checkers?\b/i, 'checkers'],
  # Only the two logs whose length actually changes with work. Lexicon/gallery tables state the
  # size of a static table in the same file, which is description rather than a derived count.
  [/#{N}\s+entries\b(?=.*(?:FAILURES|append-only|register|registry))|(?:FAILURES|append-only|register|registry).*?#{N}\s+entries\b/i,
   'failures / register_entries'],
  [/#{N}\s+(?:workspace\s+)?tests?\b/i,         'REMOVE — not computable, say `cargo test`'],
  [/#{N}\s+corpus entries\b/i,                 'corpus_entries'],
]

MARKER = /<!--s:([a-z0-9_]+)-->(.*?)<!--\/s-->/m

marker_errors = []
bare_errors   = []
fixed         = []
marker_count  = 0

LIVE.each do |path|
  next if path == 'index.json'
  text = File.read(path, encoding: 'UTF-8', invalid: :replace, undef: :replace)
  original = text.dup

  text = text.gsub(MARKER) do
    key, val = Regexp.last_match(1), Regexp.last_match(2)
    marker_count += 1
    unless STATS.key?(key)
      marker_errors << "#{path}: unknown stats key `#{key}`"
      next Regexp.last_match(0)
    end
    want = STATS[key].to_s
    if val != want
      if FIX
        fixed << "#{path}: #{key} #{val} -> #{want}"
      else
        marker_errors << "#{path}: #{key} says #{val}, stats says #{want}"
      end
    end
    "<!--s:#{key}-->#{want}<!--/s-->"
  end

  File.write(path, text) if FIX && text != original

  # Half 2: bare counts outside markers.
  stripped = text.gsub(MARKER, '')
  FORBIDDEN.each do |re, key|
    stripped.each_line.with_index do |line, i|
      next unless line =~ re
      bare_errors << "#{path}:#{i + 1}: #{line.strip[0, 96]}\n      -> use <!--s:#{key}--> or remove the count"
    end
  end
end

# index.json: no comments in JSON, so the whole block is generated.
if File.exist?('index.json')
  doc = JSON.parse(File.read('index.json', encoding: 'UTF-8'))
  want = {
    'generated_by' => 'ruby tools/stats.rb --json',
    'note' => 'Generated, not hand-written. Do not edit these numbers — they go stale. ' \
              'Regenerate with `ruby tools/check-counts.rb --fix`.',
  }.merge(STATS)
  have = doc['registry_state']
  if have != want
    if FIX
      doc['registry_state'] = want
      File.write('index.json', JSON.pretty_generate(doc) + "\n")
      fixed << 'index.json: registry_state regenerated'
    else
      marker_errors << 'index.json: `registry_state` does not match stats (run --fix)'
    end
  end
end

puts "live documents scanned: #{LIVE.size}   generated markers: #{marker_count}"
puts

unless fixed.empty?
  puts "FIXED (#{fixed.size}):"
  fixed.each { |f| puts "  + #{f}" }
  puts
end

if marker_errors.empty? && bare_errors.empty?
  puts 'OK — every stated count is generated and current; no bare counts in live prose.'
  exit 0
end

unless marker_errors.empty?
  puts "STALE GENERATED COUNTS (#{marker_errors.size}):"
  marker_errors.each { |e| puts "  x #{e}" }
  puts "\nFIX: ruby tools/check-counts.rb --fix\n\n"
end

unless bare_errors.empty?
  puts "HAND-WRITTEN COUNTS (#{bare_errors.size}) — these are what go stale:"
  bare_errors.each { |e| puts "  x #{e}" }
  puts
  puts 'FIX: wrap it in a marker so it regenerates, or delete the number and name the'
  puts '     authoritative source instead (note 060: prose duplicating the tree goes stale).'
  puts '     A count that cannot be computed — the test count — must not be stated at all.'
end

exit 1 if STRICT
