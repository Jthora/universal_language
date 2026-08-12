#!/usr/bin/env ruby
# encoding: utf-8
# check-propagation.rb — when a claim is scoped, its dependents must be reviewed.
#
# WHY THIS EXISTS (notes/039)
#   FIXED-POINT-IS-COMBINATORIAL-MAP was scoped three times — connected-configurations-only,
#   UWS-not-UL, and its UP role. Five claims reference it. ZERO carried any of the scopings.
#   A correction lands on its target and nowhere else, so everything built on a hub keeps reading
#   the pre-correction version.
#
#   That produced a live error rather than a bookkeeping one: REGIONS-ARE-FACE-UNIONS, the Phase 4
#   decidability result, inherited a connectivity limitation it does not carry — leaving it
#   decidable and wrong for any notation with more than one stroke.
#
# WHY NOT KEYWORD MATCHING
#   The first attempt scanned dependents for terms from the scope text. It produced a FALSE POSITIVE
#   immediately: "connected" matched inside "connected components of the complement", an unrelated
#   use. The scan for lost context lost context. So acknowledgment is EXPLICIT, not inferred.
#
# THE CONVENTION
#   A claim that references a scoped claim must record:
#
#       scope_reviewed: [HUB-ID, ...]
#
#   meaning: the hub's scope was read and either applies and is accounted for, or does not apply.
#   Recording it is cheap; the point is that it cannot happen by accident.
#
# USAGE
#   ruby tools/check-propagation.rb            # report
#   ruby tools/check-propagation.rb --strict   # exit 1 on unreviewed dependencies (CI mode)

require 'yaml'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

claims = YAML.load_file('claims.yaml')['claims']
by_id  = claims.each_with_object({}) { |c, h| h[c['id']] = c }

# Hubs are claims carrying a scope condition — the thing dependents can silently miss.
hubs = claims.select { |c| c['scope'].to_s.strip.length > 40 }.map { |c| c['id'] }

pending = []
reviewed = 0

claims.each do |c|
  body = c.values.map(&:to_s).join(' ')
  acknowledged = Array(c['scope_reviewed']).map(&:to_s)

  hubs.each do |hub|
    next if hub == c['id']
    next unless body.include?(hub)
    if acknowledged.include?(hub)
      reviewed += 1
    else
      pending << [c['id'], hub]
    end
  end
end

puts "scoped hubs: #{hubs.size}   dependency edges into them: #{pending.size + reviewed}"
puts "reviewed: #{reviewed}   pending: #{pending.size}"

if pending.empty?
  puts "\nOK — every dependent of a scoped claim has reviewed that scope."
  exit 0
end

puts "\nUNREVIEWED DEPENDENCIES (#{pending.size}):\n\n"
pending.group_by(&:last).sort_by { |_, v| -v.size }.each do |hub, rows|
  puts "  #{hub}"
  puts "      scope: #{by_id[hub]['scope'].to_s.gsub(/\s+/, ' ')[0, 110]}..."
  rows.each { |dep, _| puts "      <- #{dep}" }
  puts
end

puts 'FIX: read the hub\'s scope, then record `scope_reviewed: [HUB-ID]` on the dependent —'
puts '     after confirming the scope either is accounted for, or does not apply.'

exit(ARGV.include?('--strict') ? 1 : 0)
