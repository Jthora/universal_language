#!/usr/bin/env ruby
# encoding: utf-8
# check-derivation-premises.rb — the undetermined-premise rule, executed.
#
# WHY THIS EXISTS (note 065)
#   The imported derivation register (uws/register/) carries one rule its own author calls
#   "the part that transfers to a corpus we did not write":
#
#       A derivation routed through an undetermined premise is itself undetermined.
#
#   This repo had no equivalent. `scope:` plus `scope_reviewed:` is a LOCAL, per-node
#   acknowledgement — each edge is judged on its own and nothing propagates uncertainty
#   forward through a chain. FAILURES.md F-035 records what a per-node check costs: a
#   perimeter can retreat one locally-defensible step at a time with nothing summing the
#   steps. Forward propagation is the shape of check that cannot be defeated that way.
#
#   So the rule is adopted, and this file is what executes it — because the register states
#   the rule in prose and prose does not execute (note 048). Running it on the register's own
#   data found no violations in the traceable portion and showed the traceable portion is
#   small: the premises are mostly free text. That is reported as coverage, not hidden.
#
# WHAT IT CHECKS
#   1. No assertable entry (derived / derived_given / measured / conventional) transitively
#      routes through a premise that is `undetermined` or `withdrawn`.
#   2. No cycles in the premise graph.
#   3. Reports COVERAGE — how much of the register the rule can actually see. A rule whose
#      coverage is unstated is a rule that can pass by being blind.
#
# USAGE
#   ruby tools/check-derivation-premises.rb            # report
#   ruby tools/check-derivation-premises.rb --strict   # exit 1 on violations (CI mode)

require 'json'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

REGISTER = 'uws/register/derivation-register.json'
unless File.exist?(REGISTER)
  puts "SKIP — #{REGISTER} not present."
  exit 0
end

doc    = JSON.parse(File.read(REGISTER, encoding: 'UTF-8'))
claims = doc['claims'] or abort "#{REGISTER}: no `claims` key"
ids    = claims.keys

ASSERTABLE = %w[derived derived_given measured conventional].freeze
BLOCKING   = %w[undetermined withdrawn].freeze

# Premises are referenced by id INSIDE free text, so the edge set is recovered by substring
# match. That under-reads (prose-only premises are invisible) and never over-reads.
premises = {}
ids.each do |id|
  c = claims[id]
  text = %w[given derivation].map { |f| c[f].to_s }.join(' ')
  premises[id] = ids.reject { |o| o == id }.select { |o| text.include?(o) }
end

def reach(id, premises, seen = nil)
  seen ||= {}
  out = []
  premises[id].each do |p|
    next if seen[p]
    seen[p] = true
    out << p
    out.concat(reach(p, premises, seen))
  end
  out.uniq
end

violations = []
ids.each do |id|
  st = claims[id]['status']
  next unless ASSERTABLE.include?(st)
  bad = reach(id, premises).select { |p| BLOCKING.include?(claims[p]['status']) }
  violations << [id, st, bad.sort] unless bad.empty?
end

cycles = ids.select { |id| reach(id, premises).include?(id) }

by_status = claims.values.group_by { |c| c['status'] }.transform_values(&:size)
linked    = premises.count { |_, v| !v.empty? }
edges     = premises.values.sum(&:size)
dg        = claims.values.count { |c| c['status'] == 'derived_given' }
dg_prose  = ids.count { |id| claims[id]['status'] == 'derived_given' && premises[id].empty? }

puts "register: #{ids.size} entries  " + by_status.sort_by { |_, n| -n }.map { |s, n| "#{s}=#{n}" }.join('  ')
puts "premise graph: #{linked} entries with id-linked premises, #{edges} edges"
puts

if violations.empty? && cycles.empty?
  puts 'OK — no assertable entry routes through an undetermined or withdrawn premise; no cycles.'
else
  unless violations.empty?
    puts "PREMISE VIOLATIONS: #{violations.size}\n\n"
    violations.each do |id, st, bad|
      puts "  x #{id}  [#{st}]"
      bad.each { |b| puts "      routes through #{b}  [#{claims[b]['status']}]" }
    end
    puts
    puts 'FIX: the rule is not negotiable — downgrade the entry to `undetermined`, or settle'
    puts '     the premise. An assertable claim standing on an unsettled one is the defect.'
    puts
  end
  unless cycles.empty?
    puts "PREMISE CYCLES: #{cycles.size}"
    cycles.each { |c| puts "  x #{c}" }
    puts
  end
end

# Coverage is part of the result. A rule that passes by being blind has not passed.
pct = ids.empty? ? 0 : (100.0 * linked / ids.size).round
puts "COVERAGE: the rule can see #{linked}/#{ids.size} entries (#{pct}%)."
puts "  #{dg_prose} of #{dg} `derived_given` entries state their premise as PROSE ONLY and are"
puts '  invisible to this check. Converting those premises to id references is what raises'
puts '  coverage; until then a pass here is a pass over the linked portion, nothing more.'

exit 1 if (!violations.empty? || !cycles.empty?) && ARGV.include?('--strict')
