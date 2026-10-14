#!/usr/bin/env ruby
# frozen_string_literal: true
#
# blast-radius.rb — if this falls, what falls with it?
#
# WHY THIS EXISTS (note 074)
#   The repo already executes "a derivation routed through an undetermined premise is itself
#   undetermined" (check-derivation-premises.rb), but only as a GATE: it reports violations. It
#   never answers the question an anchor collapse actually asks, which is not "is anything
#   violated?" but "what do I still know, now that this has fallen?"
#
#   That question is the whole content of an anchor collapse. When something load-bearing is
#   discredited, the damage is not the thing itself — it is everything that silently routed
#   through it and now has nothing underneath. A reader who cannot COMPUTE that set has to treat
#   the whole corpus as suspect, which is the void. A reader who can compute it loses exactly the
#   affected subtree and keeps the rest.
#
#   So the blast radius of a node is the price of having trusted it, and the ledger of open
#   conventions is a map of where that price can be charged (072: every open ledger line is a seat
#   of power; 074: it is also a blast radius).
#
#   Usage:
#     ruby tools/blast-radius.rb CLAIM-ID [--json]   # what falls if CLAIM-ID falls
#     ruby tools/blast-radius.rb --census            # every claim, ranked by blast radius

require 'yaml'
require 'json'
require 'set'

claims = YAML.load_file('claims.yaml')['claims']
by_id  = claims.to_h { |c| [c['id'].to_s, c] }

# Edge direction: a claim recording `scope_reviewed: [HUB]` has READ the hub's scope and tied its
# own stated limits to it. HUB -> DEPENDENT is the direction damage travels.
#
# WHAT AN EDGE DOES AND DOES NOT MEAN — this bounds every number below.
#   `scope_reviewed` is a SCOPE ACKNOWLEDGEMENT, not a logical premise. If a hub falls, its
#   dependents do NOT automatically become false. What they lose is their BOUNDARY: the scope they
#   declared was computed against a hub that no longer holds, so the claim may survive while the
#   statement of where it applies does not.
#   That is the subtle half of an anchor collapse and the half people miss. The loud failure is
#   losing beliefs. The quiet one is keeping them and losing the ability to say where they are
#   valid — which is how a collapsed anchor goes on doing damage through people who feel fine.
dependents = Hash.new { |h, k| h[k] = [] }
claims.each do |c|
  Array(c['scope_reviewed']).each { |hub| dependents[hub.to_s] << c['id'].to_s }
end

def fallout(root, dependents)
  seen = Set.new
  queue = [root]
  depth = { root => 0 }
  until queue.empty?
    cur = queue.shift
    dependents[cur].each do |d|
      next if seen.include?(d) || d == root
      seen << d
      depth[d] = depth[cur] + 1
      queue << d
    end
  end
  [seen, depth]
end

if ARGV.include?('--census')
  rows = by_id.keys.map do |id|
    set, = fallout(id, dependents)
    { 'id' => id, 'tier' => by_id[id]['tier'], 'blast' => set.size }
  end.sort_by { |r| [-r['blast'], r['id']] }

  carriers = rows.count { |r| r['blast'].positive? }
  puts '=== blast-radius census ========================================='
  puts "claims: #{rows.length}   load-bearing (blast > 0): #{carriers}   " \
       "isolated: #{rows.length - carriers}"
  puts
  puts 'A claim with blast 0 can be withdrawn and nothing recorded here falls with it.'
  puts 'A high number is not a defect — it means the claim is doing work. It is the price of'
  puts 'trusting it, and the number to look at FIRST when the claim is challenged.'
  puts
  top = rows.select { |r| r['blast'].positive? }.first(15)
  puts 'MOST LOAD-BEARING:'
  top.each { |r| puts format('  %-3d  %-9s %s', r['blast'], r['tier'], r['id']) }
  puts
  puts 'CAVEAT — this walks `scope_reviewed` edges only. A claim used in prose but never recorded'
  puts 'as a scope dependency is INVISIBLE here, so these numbers are lower bounds, not totals.'
  exit 0
end

root = ARGV.find { |a| !a.start_with?('--') }
abort "usage: ruby tools/blast-radius.rb CLAIM-ID [--json] | --census" unless root
abort "no such claim: #{root}" unless by_id.key?(root)

set, depth = fallout(root, dependents)
ordered = set.to_a.sort_by { |d| [depth[d], d] }

if ARGV.include?('--json')
  puts JSON.pretty_generate('root' => root, 'blast_radius' => set.size,
                            'falls' => ordered.map { |d| { 'id' => d, 'hops' => depth[d],
                                                           'tier' => by_id[d]['tier'] } })
  exit 0
end

c = by_id[root]
puts "=== if #{root} falls ============================"
puts "tier: #{c['tier']}"
puts
if set.empty?
  puts 'BLAST RADIUS: 0 — nothing recorded depends on it.'
  puts 'Withdrawing it costs this registry nothing. That is not praise: an isolated claim is'
  puts 'either genuinely independent or never wired in.'
else
  puts "BLAST RADIUS: #{set.size} claim(s) lose a scope they acknowledged, by hops:"
  ordered.each { |d| puts format('  %d hop%s  %-9s %s', depth[d], depth[d] == 1 ? ' ' : 's', by_id[d]['tier'], d) }
  puts
  puts "SURVIVES: #{claims.length - set.size - 1} of #{claims.length - 1} other claims are untouched."
  puts 'That subtraction is the point. A reader who can compute this set loses exactly it and'
  puts 'keeps the rest; a reader who cannot has to suspect everything, which is the void.'
end
puts
puts 'CAVEAT — `scope_reviewed` edges only. Prose dependencies are invisible, so this is a'
puts 'lower bound on what actually falls.'
