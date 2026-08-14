#!/usr/bin/env ruby
# check-caveats.rb — a source with a recorded caveat may not be cited without it.
#
# WHY THIS EXISTS (FAILURES.md F-027)
#   research/surveys/research-register.md records fourteen source caveats — sources whose evidence
#   is narrower than it looks. They were written after each one was caught. Nothing enforced them,
#   so they sat in a survey document with no force, and note 034 cited AMR as evidence about
#   universal semantic structure despite D2-c recording that AMR is an "English-only corpus,
#   English predicate lexicon, English annotators" whose use as universality evidence "would
#   reproduce UNL's single most documented failure mode."
#
#   The caveat had already been written down, marked "near-miss caught", and was re-committed
#   anyway. Recording a limitation is not the same as being unable to forget it.
#
# WHAT IT CHECKS
#   If a document cites a caveated source in an evidential way, it must also carry the caveat —
#   either the register id (e.g. "D2-c") or the caveat's distinguishing term.
#
# USAGE
#   ruby tools/check-caveats.rb            # report
#   ruby tools/check-caveats.rb --strict   # exit 1 on violations (CI mode)

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

# source pattern => [register id, terms any one of which counts as carrying the caveat]
CAVEATS = {
  /\bAMR\b/                       => ['D2-c',  %w[English-only English\ corpus English\ predicate annotation\ convention]],
  /\bUNL\b/                       => ['D3-c',  ['confounded', 'outcompeted', 'statistical', 'neural MT']],
  /\bSHACL\b/                     => ['D5-c',  ['not the invariants', 'constraint language', 'shapes graph']],
  /\bDaikon\b/                    => ['E6-c',  ['candidate generator', 'likely invariants', 'not oracle', 'not an oracle']],
  /\bsheaf\b/i                    => ['E7-c',  ['cover', 'quantum measurement', 'base space']],
  /G[äa]rdenfors/                 => ['E9-c',  ['quality dimensions', 'relative to a choice', 'convexity is relative']],
  /\bChangizi\b/                  => ['F1-c',  ['legibility', 'detectability', 'not semantic transparency', 'human']],
  /\bShin\b/                      => ['F2-c',  ['monadic']],
  /\bpictogram/i                  => ['F3-c',  ['referential', 'not structural', 'culturally-embedded']],
  /\bCognitive Dimensions\b|\bCDN\b/ => ['F5-c', ['discussion vocabulary', 'not a measurement']],
  /\bdecipherment\b/i             => ['F6-c',  ['no spec', 'no teacher', 'bilingual']],
  /music.notation|staff notation/i => ['F7-c', ['displacing', 'entrenched', 'incumbent', '1,000 years']],
}.freeze

SKIP = %w[.git/ ul-forge/target/ node_modules/].freeze
# The register defines the caveats; FAILURES.md and the protocol discuss the mechanism itself.
ALLOW = ['research/surveys/research-register.md', 'FAILURES.md', 'RESEARCH-PROTOCOL.md',
         'tools/check-caveats.rb', 'research/method/negative-results.md'].freeze

# A mention is only "evidential" if the document leans on it. Bare name-drops in a list are fine.
EVIDENTIAL = /shows|show |demonstrat|evidence|proves|proved|establish|confirms|finds|found that|
              according to|per |uses|use[sd]? label|indicat/xi

# Closed notes are immutable by convention (research/notes/README.md) — the record of what was
# thought at the time is the data, and retro-editing it would destroy that. So notes are REPORTED,
# not enforced; live documents are enforced.
def historical?(path)
  path.start_with?('research/notes/')
end

violations = []
historical = []
checked = 0

Dir.glob('**/*.md').sort.each do |path|
  next if SKIP.any? { |d| path.start_with?(d) } || ALLOW.include?(path)
  text = File.read(path, encoding: 'UTF-8', invalid: :replace, undef: :replace)

  CAVEATS.each do |pattern, (id, terms)|
    next unless text =~ pattern
    checked += 1
    # Does the document carry the caveat at all?
    carried = text.include?(id) || terms.any? { |t| text =~ /#{t}/i }
    next if carried
    # Only flag if the source is actually being leaned on.
    next unless text =~ EVIDENTIAL
    (historical?(path) ? historical : violations) << [path, id, pattern.source[0, 30]]
  end
end

puts "caveated-source citations checked: #{checked}"

unless historical.empty?
  puts "\nHISTORICAL (#{historical.size}) — in closed notes, which are immutable by convention."
  puts "Reported so the pattern is visible; not enforced, because the record is the data."
  historical.group_by(&:first).each { |p, v| puts "  #{p}  (#{v.map { |x| x[1] }.join(', ')})" }
end

if violations.empty?
  puts "\nOK — every citation of a caveated source in a LIVE document carries its caveat."
  exit 0
end

puts "\nVIOLATIONS: #{violations.size} — a caveated source cited without its caveat\n\n"
violations.each { |p, id, src| puts "  #{p}\n      cites /#{src}/ — caveat #{id} not carried" }
puts
puts 'FIX: state the caveat where the source is used, or cite the register id.'
puts '     A recorded limitation only works if it travels with the citation.'

exit(ARGV.include?('--strict') ? 1 : 0)
