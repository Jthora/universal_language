#!/usr/bin/env ruby
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
warnings = []

doc = YAML.load_file('claims.yaml')
claims = doc['claims'] or abort 'claims.yaml has no `claims` key'

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

  # evidence / rationale paths must resolve when they look like paths
  %w[evidence rationale superseded_by].each do |field|
    v = c[field]
    next unless v.is_a?(String) && v.match?(/\.(md|rs|json|ya?ml)\b/)
    path = v.split('#').first.strip
    errors << "#{id}: `#{field}` path does not exist: #{path}" unless File.exist?(path)
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

by_tier = Hash[claims.group_by { |c| c['tier'] }.map { |k, v| [k, v.size] }]
puts "claims: #{claims.size}  |  " + TIERS.map { |t| "#{t}=#{by_tier.fetch(t, 0)}" }.join('  ')

unless warnings.empty?
  puts "\nWARNINGS (#{warnings.size}):"
  warnings.each { |w| puts "  ! #{w}" }
end

if errors.empty?
  puts "\nOK — tier discipline holds."
  exit 0
end

puts "\nERRORS (#{errors.size}):"
errors.each { |e| puts "  x #{e}" }
exit(ARGV.include?('--strict') ? 1 : 0)
