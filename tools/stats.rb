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
  # `^## F-\d+ ` requires a space after the digits, which EXCLUDES the lettered correction
  # entries (F-012a, F-013a). That is intended — a correction to a failure is not a new failure —
  # but it was silent, and a count with an undocumented exclusion is the shape of F-039.
  'failures'         => File.read('FAILURES.md', encoding: 'UTF-8').scan(/^## F-\d+ /).size,
  'notes'            => Dir.glob('research/notes/[0-9]*/').size,
  'checkers'         => Dir.glob('tools/check-*.rb').size,
  'corpus_entries'   => Dir.glob('corpus/entries/*.json').size,
  'register_entries' => register ? register['claims'].size : 0,
}

# D1 of PROGRAM-SUCCESS-CONDITION has TWO numbers and they are not the same thing.
#   atlas_recorded  — orbits with an ATTESTED meaning in the imported Codex register. Theirs.
#   derived_orbits  — orbits this repo has given machine-checked structural content, counted
#                     from the `orbit` field of VERIFIED corpus entries. Ours, and the one that
#                     moves when work is done here (note 071).
#   derived_orbits_argued
#                   — orbits whose entry is machine-checked but whose content routes through a
#                     premise weaker than VERIFIED. Reported separately and NEVER summed into the
#                     line above, for the same reason atlas_recorded is never summed with it.
# An attested label is not a derivation and a derivation is not an attested label, so the two
# are reported separately and never summed.
if register && register['combination_space']
  cs = register['combination_space']
  stats['atlas_orbits']       = cs['orbits']
  stats['atlas_recorded']     = cs['recorded']
  stats['atlas_undetermined'] = cs['undetermined']
  stats['atlas_percent']      = ((100.0 * cs['recorded']) / cs['orbits']).round(1)

  # TIGHTENED 2026-10-15, note 078, and tightened BEFORE anything was added to it so the change
  # cannot be a ratchet (075 REQ-3 forbids adjusting a normalisation until the number comes out
  # right; this moves it the hard way). 071 counted every entry's orbit regardless of tier, but
  # the figure it headlines is "orbits this repo has given machine-checked structural content",
  # and an ARGUED entry's content rests on a fitted premise. So the headline counts VERIFIED
  # entries only and the weaker work is reported beside it rather than folded in or hidden.
  by_tier = Dir.glob('corpus/entries/*.json').filter_map do |f|
    e = JSON.parse(File.read(f, encoding: 'UTF-8'))
    e['orbit'] ? [e['tier'], e['orbit']] : nil
  end
  derived = by_tier.select { |t, _| t == 'VERIFIED' }.map(&:last).uniq
  argued  = by_tier.reject { |t, _| t == 'VERIFIED' }.map(&:last).uniq - derived
  stats['derived_orbits']        = derived.size
  stats['derived_orbits_argued'] = argued.size
  stats['derived_percent'] = ((100.0 * derived.size) / cs['orbits']).round(1)
end

# --- import-consistency measurements (notes 076, 078) -----------------------------------------
#   These are the numbers the two import checkers compute. They are here so that NOTHING has to
#   type them: before this block they lived as hand-written digits inside claims.yaml and inside
#   those checkers' own headers, and claims.yaml is not scanned by check-counts.rb at all. One of
#   them had already gone stale within the hour it was written (note 078 section 1).
#
#   Loud on failure, never silently zeroed — a bare `rescue` that swallows the error and emits
#   nothing is the same disease as a checker that declines to check (F-039, and stats.rb's own
#   earned-tier block, which had exactly this bug).
begin
  ch = JSON.parse(`ruby tools/check-chirality.rb --json 2>/dev/null`.force_encoding('UTF-8'))
  stats['chirality_cells']    = ch['cells']
  stats['chirality_agree']    = ch['agree']
  stats['chirality_disputed'] = ch['disputed'].size
rescue StandardError => e
  warn "stats.rb: chirality block failed (#{e.class}: #{e.message}) — keys omitted, not zeroed"
end

begin
  pl = JSON.parse(`ruby tools/check-placement.rb --json 2>/dev/null`.force_encoding('UTF-8'))
  stats['placement_cells']       = pl['cells']
  stats['placement_pairs']       = pl['pairs']
  stats['placement_informative'] = pl['informative']
  stats['placement_vacuous']     = pl['vacuous']
  # the two sides of the predicate, which is what a reader needs to judge the sample
  stats['placement_destroyed'] = pl['directions']['operator destroyed'] || 0
  stats['placement_preserved'] = pl['directions']['operator preserved'] || 0
rescue StandardError => e
  warn "stats.rb: placement block failed (#{e.class}: #{e.message}) — keys omitted, not zeroed"
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

# --- earned tiers (note 075) --------------------------------------------------------------------
# The declared tier is the author's intent; the EARNED tier is what the artifact supports. Both are
# reported because the gap is the honest edge of the registry, and because a hand-written tier is a
# written count and COUNTS-ARE-GENERATED says those go stale.
begin
  # force_encoding is load-bearing: the checker's output contains UTF-8 (em dashes), the default
  # external encoding here is US-ASCII, and `match` then raises invalid-byte-sequence. The first
  # version of this block had a bare `rescue StandardError` that SWALLOWED exactly that and
  # silently emitted no keys — a rescue that hides a bug is the same disease as a checker that
  # declines to check (note 075).
  out = `ruby tools/check-claims.rb 2>/dev/null`.force_encoding('UTF-8')
  if (m = out.match(/VERIFIED: (\d+) declared \s*\|\s*(\d+) EARNED/))
    stats['claims_verified_declared'] = m[1].to_i
    stats['claims_verified_earned']   = m[2].to_i
  end
  if (m = out.match(/confidence-scale claims: (\d+)/))
    # The well-formed denominator: DESIGN-CHOICE is a KIND and RETIRED a LIFECYCLE state, so
    # neither can ever be VERIFIED. Quoting VERIFIED over all claims understates it.
    stats['claims_confidence_scale'] = m[1].to_i
  end
  if (m = out.match(/reachability \(computed\): in-tree=(\d+)\s+external=(\d+)\s+absent=(\d+)/))
    stats['evidence_in_tree']  = m[1].to_i
    stats['evidence_external'] = m[2].to_i
    stats['evidence_absent']   = m[3].to_i
  end
rescue StandardError => e
  warn "stats.rb: earned-tier block failed (#{e.class}: #{e.message}) — keys omitted, not zeroed"
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
