#!/usr/bin/env ruby
# encoding: utf-8
# check-claims.rb — enforces the claim registry's tier discipline.
#
# WHY THIS EXISTS
#   This repo's named historical failure mode is "conventions without enforcement": rigor labels
#   nobody checked. claims.yaml declared its tier requirements in a comment block from the day it
#   was seeded and nothing verified them. This is that verification.
#
# CHECKS
#   1. IDs unique
#   2. Tier is one of the five known values
#   3. VERIFIED / ARGUED    -> evidence present and the path exists
#   4. CONJECTURED          -> falsified_by present and non-empty
#   5. DESIGN-CHOICE        -> rationale + alternatives_considered present
#   6. RETIRED              -> superseded_by, formalization, revival_condition, refutation_tier
#   7. DESIGN-CHOICE alternatives are not HOLLOW (F-018): bare numerals or version history are not
#      alternatives. Every genuine entry states an option and why it lost.
#   8. Referenced claim ids (blocks / blocked_by / supersedes / bounded_by) resolve
#   9. WARN: priority-0 claims whose own notes admit tier-5 evidence (S1 violation)
#
# USAGE
#   ruby tools/check-claims.rb            # report
#   ruby tools/check-claims.rb --strict   # exit 1 on any error (CI mode)

require 'yaml'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

TIERS = %w[VERIFIED ARGUED CONJECTURED DESIGN-CHOICE RETIRED].freeze
REQUIRED = {
  'VERIFIED'      => %w[evidence],
  'ARGUED'        => %w[evidence],
  'CONJECTURED'   => %w[falsified_by],
  'DESIGN-CHOICE' => %w[rationale alternatives_considered],
  'RETIRED'       => %w[superseded_by formalization revival_condition refutation_tier],
}.freeze

# An "alternative" that is only a number or a version label records no decision (F-018).
HOLLOW = /\A[\s\d.,()\-]*\z|\A\d+\s*\((?:pre-)?\d{4}|current\)\z/

errors = []

external = []
warnings = []
reach_of  = {}   # id -> :in_tree | :external | :absent | :none   (COMPUTED, never authored)
earned_of = {}   # id -> earned tier                              (COMPUTED, never authored)

doc = YAML.load_file('claims.yaml')
claims = doc['claims'] or abort 'claims.yaml has no `claims` key'

# Duplicate keys inside a claim block are SILENTLY DISCARDED by the YAML parser — the last wins
# and the earlier value vanishes without error. That is invisible data loss in the registry, so it
# is checked textually rather than through the parsed structure.
File.read('claims.yaml', encoding: 'UTF-8').split(/^  - id: /).drop(1).each do |block|
  cid = block.lines.first.strip
  seen = Hash.new(0)
  block.scan(/^    ([a-z_]+):/) { |k| seen[k.first] += 1 }
  seen.select { |_, n| n > 1 }.each_key do |k|
    errors << "#{cid}: duplicate key `#{k}` — YAML keeps only the last, silently discarding the rest"
  end
end

ids = claims.map { |c| c['id'] }
ids.group_by { |i| i }.select { |_, v| v.size > 1 }.each_key { |dup| errors << "duplicate id: #{dup}" }

claims.each do |c|
  id   = c['id'] || '(missing id)'
  tier = c['tier']

  errors << "#{id}: unknown tier #{tier.inspect}" unless TIERS.include?(tier)
  next unless TIERS.include?(tier)

  REQUIRED.fetch(tier, []).each do |field|
    v = c[field]
    if v.nil? || (v.respond_to?(:empty?) && v.empty?)
      errors << "#{id}: tier #{tier} requires `#{field}`"
    end
  end

  # evidence / rationale paths must resolve when they look like paths.
  #
  # WHY THE SHAPE OF THIS CHECK MATTERS (notes 073/074, found by a cold reader)
  #   The original pattern was /\.(md|rs|json|ya?ml)\b/, which silently skipped every `.py` and
  #   `.rb` evidence path and every directory-valued one — 40 of 197 paths, including 7 VERIFIED
  #   claims whose evidence lives in a DIFFERENT repository that is not in this tree. A checker
  #   that quietly declines to check is worse than no checker, because the registry's header
  #   promises the check happened. So: widened to .py/.rb, directory paths now resolve, and a
  #   `repo:path` reference is reported as EXTERNAL rather than skipped or miscalled broken.
  %w[evidence rationale superseded_by].each do |field|
    v = c[field]
    next unless v.is_a?(String)
    raw = v.split('#').first.strip
    # A claim ID (ALL-CAPS-WITH-DASHES) is a cross-reference, not a path.
    next if raw.match?(/\A[A-Z][A-Z0-9-]*\z/)
    # `rationale` is free prose for DESIGN-CHOICE entries; only treat it as a path if it looks like one.
    looks_pathy = raw.match?(%r{\A[\w.]+[\w./-]*\.(md|rs|json|ya?ml|py|rb)\z}) ||
                  raw.match?(%r{\A[\w][\w./-]*/\z}) ||
                  raw.match?(%r{\A[a-z_]+:[\w][\w./-]*})
    next unless looks_pathy
    if (m = raw.match(/\A([a-z_]+):(.+)\z/))
      # declared external reference, e.g. `ul_agent:tests/foo.py` — cannot be checked from here
      external << "#{id}: `#{field}` is external (#{m[1]}): #{m[2]}"
      next
    end
    errors << "#{id}: `#{field}` path does not exist: #{raw}" unless File.exist?(raw)
  end

  # F-018: alternatives that are bare values record no decision
  if tier == 'DESIGN-CHOICE'
    alts = Array(c['alternatives_considered']).map(&:to_s)
    if alts.any? && alts.all? { |a| a.match?(HOLLOW) }
      errors << "#{id}: alternatives_considered is hollow #{alts.inspect} — " \
                'bare values or version history are not alternatives (F-018)'
    end
  end

  # cross-references between claims must resolve
  %w[blocks blocked_by supersedes bounded_by].each do |field|
    Array(c[field]).each do |ref|
      next unless ref.is_a?(String) && ref == ref.upcase && ref.match?(/\A[A-Z0-9-]+\z/)
      errors << "#{id}: `#{field}` references unknown claim #{ref}" unless ids.include?(ref)
    end
  end

  # S1: a priority-0 claim should not rest on a summary
  if c['priority'].to_i.zero? && c.key?('priority')
    notes = c['notes'].to_s
    if notes.match?(/single (paper|source|citation)|read via summary|tier S1-5/i)
      warnings << "#{id}: priority-0 and admits tier-5 evidence — read the primary source (S1)"
    end
  end
end

# ─────────────────────────────────────────────────────────────────────────────────────────────
# EARNED TIER — computed from facts, never read from the file (notes 075)
#
#   WHY. claims.yaml#COUNTS-ARE-GENERATED says no count is a claim: a written count goes stale
#   because nothing recomputes it. A hand-written `tier:` IS a written count. It is set once, by
#   an author, and then the world moves. F-039: seven VERIFIED claims cited evidence in another
#   repository that never contained it, and the label said VERIFIED for as long as nobody looked.
#
#   So `tier:` is kept as the author's INTENT and the checker computes what the artifact has
#   actually EARNED. Where they disagree, the gap is printed. VERIFIED on unreachable evidence is
#   an error, because that is the one combination that cannot be true.
#
#   REACHABILITY is the axis that had no field at all, which is why its failure was invisible:
#     in_tree  — evidence resolves in this checkout; a reader can re-run it
#     external — a declared `repo:path` reference; cannot be checked from here
#     absent   — names a path that does not resolve
# ─────────────────────────────────────────────────────────────────────────────────────────────
EVIDENCE_FIELD = { 'VERIFIED' => 'evidence', 'ARGUED' => 'evidence', 'CONJECTURED' => 'evidence' }.freeze
RUNNABLE = /\.(py|rb|rs)\z/

claims.each do |c|
  id = c['id'].to_s
  field = EVIDENCE_FIELD[c['tier']]
  raw = field ? c[field].to_s.split('#').first.to_s.strip : ''
  reach_of[id] =
    if raw.empty?                     then :none
    elsif raw.match?(/\A[a-z_]+:/)    then :external
    elsif File.exist?(raw)            then :in_tree
    else :absent
    end
end

# Earned tier from evidence alone, before premises.
RANK = { 'CONJECTURED' => 1, 'ARGUED' => 2, 'VERIFIED' => 3 }.freeze
UNRANK = RANK.invert.freeze

claims.each do |c|
  id = c['id'].to_s
  declared = c['tier']
  unless RANK.key?(declared)
    earned_of[id] = declared   # DESIGN-CHOICE and RETIRED are a KIND and a LIFECYCLE, not
    next                       # points on the confidence scale. They earn nothing and lose nothing.
  end
  raw = c['evidence'].to_s.split('#').first.to_s.strip
  # "Runnable" means a reader can execute it and see it pass. That includes a script, a directory
  # of scripts, AND a corpus entry: corpus/entries/*.json is asserted field-by-field against the
  # real implementation by ul-forge/crates/ul-core/tests/corpus_tests.rs, so it is machine-checked
  # evidence that merely happens not to be a .py file. Documentary evidence (a note, a spec) is
  # NOT runnable: it may well be the right kind of evidence for a prior-art attribution, which is
  # why that case is reported as a gap for review rather than raised as an error.
  harness_covered = raw.start_with?('corpus/entries/') && raw.end_with?('.json')
  cap =
    case reach_of[id]
    when :in_tree then (raw.match?(RUNNABLE) || File.directory?(raw) || harness_covered) ? 3 : 2
    when :external then 2   # a reader cannot re-run it, so it cannot earn the top tier here
    else 1                  # absent or missing evidence earns nothing above CONJECTURED
    end
  earned_of[id] = UNRANK[[RANK[declared], cap].min]
end

# PREMISE PROPAGATION — the imported rule, finally applied at home.
#
#   uws/register/ carries "a derivation routed through an undetermined premise is itself
#   undetermined", and tools/check-derivation-premises.rb enforces it — ON THE VENDORED REGISTER
#   ONLY. It could never run here because claims.yaml had NO premise field: `scope_reviewed` is an
#   ACKNOWLEDGEMENT ("I read your scope"), not a premise, and treating it as one produces numbers
#   that mean nothing (notes 075). `premises:` is that missing edge. Coverage is reported rather
#   than assumed, exactly as the register's own checker reports its 15%.
premise_edges = 0
claims.each { |c| premise_edges += Array(c['premises']).size }
if premise_edges.positive?
  by_id = claims.to_h { |c| [c['id'].to_s, c] }
  # Validate premise IDs FIRST. The original version checked them inside the propagation loop,
  # after `next unless RANK.key?(pe)` — so an unknown premise had no rank, was skipped, and passed
  # silently. A negative control caught it. That is the same disease as the regex this file was
  # just fixed for: a check that declines to check (F-039).
  claims.each do |c|
    Array(c['premises']).each do |pid|
      errors << "#{c['id']}: premise `#{pid}` is not a registered claim" unless by_id.key?(pid.to_s)
    end
  end
  changed = true
  rounds = 0
  while changed && rounds < 50
    changed = false
    rounds += 1
    claims.each do |c|
      id = c['id'].to_s
      next unless RANK.key?(earned_of[id])
      Array(c['premises']).each do |pid|
        pe = earned_of[pid.to_s]
        next unless RANK.key?(pe)
        if RANK[pe] < RANK[earned_of[id]]
          earned_of[id] = pe
          changed = true
        end
      end
    end
  end
end

# The one combination that cannot be true.
claims.each do |c|
  next unless c['tier'] == 'VERIFIED'
  case reach_of[c['id'].to_s]
  when :absent
    errors << "#{c['id']}: tier VERIFIED but its evidence path does not resolve — " \
              'a reader cannot re-derive it (F-039)'
  when :external
    errors << "#{c['id']}: tier VERIFIED but its evidence is in another repository — " \
              'not re-derivable from this checkout (F-039)'
  when :none
    errors << "#{c['id']}: tier VERIFIED with no evidence field at all"
  end
end

by_tier = Hash[claims.group_by { |c| c['tier'] }.map { |k, v| [k, v.size] }]
puts "claims: #{claims.size}  |  " + TIERS.map { |t| "#{t}=#{by_tier.fetch(t, 0)}" }.join('  ')

# The denominator matters. DESIGN-CHOICE and RETIRED are a KIND and a LIFECYCLE state; neither is
# a point on the confidence scale, so neither can ever be VERIFIED. Quoting VERIFIED over ALL
# claims understates it, which is how "40 of 183 = 21.9%" got repeated when the well-formed figure
# is 40 of 138 (notes 075).
candidates = claims.count { |c| RANK.key?(c['tier']) }
puts format('confidence-scale claims: %d  (excludes %d DESIGN-CHOICE + %d RETIRED, which are a ' \
            'kind and a lifecycle state, not confidence levels)',
            candidates, by_tier.fetch('DESIGN-CHOICE', 0), by_tier.fetch('RETIRED', 0))

reach_counts = reach_of.values.tally
puts format('reachability (computed): in-tree=%d  external=%d  absent=%d  no-evidence-field=%d',
            reach_counts.fetch(:in_tree, 0), reach_counts.fetch(:external, 0),
            reach_counts.fetch(:absent, 0), reach_counts.fetch(:none, 0))

earned_v = earned_of.count { |_, v| v == 'VERIFIED' }
puts format('VERIFIED: %d declared  |  %d EARNED  (earned = evidence resolves in-tree AND is ' \
            'runnable; premises cannot be weaker)', by_tier.fetch('VERIFIED', 0), earned_v)

gaps = claims.select { |c| RANK.key?(c['tier']) && earned_of[c['id'].to_s] != c['tier'] }
unless gaps.empty?
  puts "\nDECLARED ABOVE EARNED (#{gaps.size}) — the author's intent, and what the artifact supports:"
  gaps.each do |c|
    id = c['id'].to_s
    puts format('  %-44s declared %-11s earned %-11s (%s)',
                id, c['tier'], earned_of[id], reach_of[id])
  end
  puts '  This list is the honest edge of the registry. It is printed, not hidden: a gap is a'
  puts '  claim whose label is ahead of its evidence, which is the failure mode of F-035/F-039.'
end

if premise_edges.zero?
  puts "\nPREMISE COVERAGE: 0 of #{candidates} claims declare `premises:`. The propagation rule"
  puts '  ("a derivation routed through an undetermined premise is itself undetermined") is'
  puts '  enforced on the VENDORED register and cannot yet bite here. Adding premise edges is'
  puts '  what turns it on; until then a pass here says nothing about premise soundness.'
else
  with = claims.count { |c| Array(c['premises']).any? }
  puts format("\nPREMISE COVERAGE: %d of %d claims declare premises (%d edges). Propagation is " \
              'live for those; silent for the rest.', with, candidates, premise_edges)
end

unless warnings.empty?
  puts "\nWARNINGS (#{warnings.size}):"
  warnings.each { |w| puts "  ! #{w}" }
end

unless external.empty?
  # Not errors: these are evidence paths in another repository, declared with a `repo:` prefix.
  # They are printed because they CANNOT be checked here, and a reader deciding how much of the
  # top tier is re-derivable from this checkout needs that number in front of them. Silently
  # skipping them is what the original regex did (see above).
  ext_verified = external.count { |e| claims.any? { |c| c['id'].to_s == e.split(':').first && c['tier'] == 'VERIFIED' } }
  puts "\nEXTERNAL EVIDENCE — not checkable from this checkout (#{external.size}):"
  external.each { |e| puts "  ~ #{e}" }
  puts "  #{ext_verified} of these are on VERIFIED claims. Their evidence is not in this tree,"
  puts "  so a reader cannot re-derive them here. Not counted as errors; counted in the open."
end

if errors.empty?
  puts "\nOK — tier discipline holds."
  exit 0
end

puts "\nERRORS (#{errors.size}):"
errors.each { |e| puts "  x #{e}" }
exit(ARGV.include?('--strict') ? 1 : 0)
