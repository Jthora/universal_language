#!/usr/bin/env ruby
# check-retired-content.rb — flags live documents that assert retired content without a status banner.
#
# WHY THIS EXISTS (FAILURES.md F-018):
#   Retiring a claim in claims.yaml does not retire the documents that instantiate it. Sigma_UL was
#   retired; its numbers (5 primitives / 4 sorts / 13 operations / 23 theorems) kept propagating
#   through ~70 unbannered files, and were repeatedly restated as current by readers and agents.
#   A retirement that leaves its artifacts standing has not happened.
#
# WHAT IT CHECKS
#   Retired counts must be ABSENT from the working tree, not disclaimed within it. Negating a
#   number still puts it in front of the reader, and in-repo archives were read as current for
#   months. Git history is the archive. The only files permitted to name retired counts are the
#   ones whose job is to record the retirement (allowlist below).
#
# USAGE
#   ruby tools/check-retired-content.rb          # report
#   ruby tools/check-retired-content.rb --strict # exit 1 on violations (CI mode)

require 'yaml'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

# Patterns that indicate retired Sigma_UL-era content stated as substance.
RETIRED_PATTERNS = [
  /\b5 primitives\b/i, /\bfive primitives\b/i,
  /\b4 sorts\b/i, /\bfour sorts\b/i,
  /\b13 operations\b/i, /\b13 composition operations\b/i,
  /\b23 theorems\b/i,
  /\bunique up to isomorphism\b/i,
  /\bUnique Grounding Theorem\b/i,
]

# A banner must appear within this many lines of the top.
BANNER_WINDOW = 15
BANNER = /retired|superseded|historical|do not cite|archive|status note/i

# Files whose PURPOSE is to discuss the retirement. Not violations.
ALLOWLIST = %w[
  FAILURES.md GLOSSARY.md claims.yaml CONTRIBUTING.md
  RESEARCH-PROTOCOL.md
  research/legacy-findings.md
  research/postmortem-and-rebuild-2026-08.md
  research/wiki-comparison-2026-08.md
  research/reassessment-2026-08-purpose-anchored.md
  research/deep-critique-2026-08-wiki-and-implementation.md
  research/emergence-investigation/keep-retire-inventory.md
]

violations = []
checked = 0

Dir.glob('**/*.md').sort.each do |path|
  next if path.start_with?('archive/')
  next if ALLOWLIST.include?(path)

  text = File.read(path, encoding: 'UTF-8', invalid: :replace, undef: :replace)
  hits = RETIRED_PATTERNS.select { |p| text =~ p }
  next if hits.empty?

  checked += 1
  violations << [path, hits.size]
end

if violations.empty?
  puts "OK — no retired counts present in the working tree."
  exit 0
end

puts "RETIRED-CONTENT VIOLATIONS: #{violations.size} file(s) name retired counts.\n\n"

by_area = violations.group_by { |p, _| p.split('/').first }
by_area.sort_by { |_, v| -v.size }.each do |area, files|
  puts "  #{area}/  (#{files.size})"
  files.sort.each { |p, n| puts "    #{p}  [#{n} pattern#{'s' if n > 1}]" }
  puts
end

puts "FIX: remove the count. Do NOT disclaim it — negating a number still restates it."
puts "  Inventories are properties of a presentation, not of the notation."
puts "  Point at the source instead: ul-forge/crates/ul-core/src/, claims.yaml#IMPL-*."
puts "  If the document as a whole is superseded, delete it — git history is the archive."
puts

exit(ARGV.include?('--strict') ? 1 : 0)
