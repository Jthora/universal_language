#!/usr/bin/env ruby
# encoding: utf-8
# stats.rb — the single source of truth for every count this repository states.
#
# WHY THIS EXISTS (claims.yaml#COUNTS-ARE-GENERATED)
#   Derived quantities were stored in prose, so they went stale within hours of any work. In one
#   session alone: "354 tests" (was 370), "31 entries" (became 36), "seven checkers" (became eight),
#   and three different claim totals — each hand-patched across four files, each wrong again by the
#   next commit. Note 060 had already found the fix for corpus/README.md ("prose duplicating the
#   tree goes stale, so the tree is now named authoritative") and nothing executed that lesson
#   anywhere else.
#
#   A count belongs in exactly one place: computed from the thing it counts.
#
# USAGE
#   ruby tools/stats.rb           # human-readable
#   ruby tools/stats.rb --json    # machine-readable (consumed by tools/check-counts.rb)
#
# WHAT IS DELIBERATELY ABSENT
#   The test count. 370 tests pass, but a static scan finds 360 `#[test]` attributes — doc-tests and
#   generated cases make up the difference, so the true number cannot be known without building.
#   Rather than publish a number that is wrong by ten, no document states a test count: they say
#   `cargo test`. A count that cannot be computed must not be asserted.

require 'yaml'
require 'json'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

claims = YAML.load_file('claims.yaml')['claims']
tiers  = claims.group_by { |c| c['tier'] }.transform_values(&:size)

register_path = 'uws/register/derivation-register.json'
register = File.exist?(register_path) ? JSON.parse(File.read(register_path, encoding: 'UTF-8')) : nil

stats = {
  'claims_total'     => claims.size,
  'claims_verified'  => tiers.fetch('VERIFIED', 0),
  'claims_argued'    => tiers.fetch('ARGUED', 0),
  'claims_conjectured' => tiers.fetch('CONJECTURED', 0),
  'claims_design'    => tiers.fetch('DESIGN-CHOICE', 0),
  'claims_retired'   => tiers.fetch('RETIRED', 0),
  'claims_priority0' => claims.count { |c| c['priority'].to_s == '0' },
  'failures'         => File.read('FAILURES.md', encoding: 'UTF-8').scan(/^## F-\d+ /).size,
  'notes'            => Dir.glob('research/notes/[0-9]*/').size,
  'checkers'         => Dir.glob('tools/check-*.rb').size,
  'corpus_entries'   => Dir.glob('corpus/entries/*.json').size,
  'register_entries' => register ? register['claims'].size : 0,
}

# D1 of PROGRAM-SUCCESS-CONDITION has TWO numbers and they are not the same thing.
#   atlas_recorded  — orbits with an ATTESTED meaning in the imported Codex register. Theirs.
#   derived_orbits  — orbits this repo has given machine-checked structural content, counted
#                     from the `orbit` field of corpus entries. Ours, and the one that moves
#                     when work is done here (note 071).
# An attested label is not a derivation and a derivation is not an attested label, so the two
# are reported separately and never summed.
if register && register['combination_space']
  cs = register['combination_space']
  stats['atlas_orbits']       = cs['orbits']
  stats['atlas_recorded']     = cs['recorded']
  stats['atlas_undetermined'] = cs['undetermined']
  stats['atlas_percent']      = ((100.0 * cs['recorded']) / cs['orbits']).round(1)

  derived = Dir.glob('corpus/entries/*.json').filter_map do |f|
    JSON.parse(File.read(f, encoding: 'UTF-8'))['orbit']
  end.uniq
  stats['derived_orbits']  = derived.size
  stats['derived_percent'] = ((100.0 * derived.size) / cs['orbits']).round(1)
end

# --- survivability (note 074) ------------------------------------------------------------------
#   pinned_*  — how much of the corpus checks ITSELF, with no oracle and no source to trust
#   blast_*   — how much of the registry falls if a source does
# Both are generated here rather than written anywhere, per COUNTS-ARE-GENERATED.
begin
  red = JSON.parse(`ruby tools/check-redundancy.rb --json`)
  stats['pinned_fields']    = red['pinned']
  stats['checkable_fields'] = red['checkable_fields']
  stats['pinned_percent']   = red['pinned_percent']
rescue StandardError
  nil # measurement, never a gate
end

begin
  require 'set'
  dep = Hash.new { |h, k| h[k] = [] }
  claims.each { |c| Array(c['scope_reviewed']).each { |h| dep[h.to_s] << c['id'].to_s } }
  reach = lambda do |root|
    seen = Set.new
    q = [root]
    until q.empty?
      cur = q.shift
      dep[cur].each { |d| next if seen.include?(d) || d == root; seen << d; q << d }
    end
    seen
  end
  ids  = claims.map { |c| c['id'].to_s }
  conj = claims.select { |c| c['tier'] == 'CONJECTURED' }.map { |c| c['id'].to_s }
  union = conj.reduce(Set.new) { |acc, c| acc | reach.call(c) } - conj.to_set
  stats['claims_load_bearing']     = ids.count { |i| reach.call(i).any? }
  stats['claims_under_conjecture'] = union.size
end

if ARGV.include?('--json')
  puts JSON.pretty_generate(stats)
else
  w = stats.keys.map(&:length).max
  stats.each { |k, v| puts format("%-#{w}s  %d", k, v) }
  puts
  puts "tiers: " + tiers.sort_by { |_, n| -n }.map { |t, n| "#{t}=#{n}" }.join('  ')
  puts
  puts 'No test count here, on purpose — see the header. Run `cargo test` instead.'
end
