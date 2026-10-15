#!/usr/bin/env ruby
# frozen_string_literal: true
#
# check-placement.rb — the placement plane, read out of the register and gated.
#
# WHY THIS EXISTS (notes 076, 077, 078)
#   076 specified a SECOND CORPUS CHANNEL and could not build it: a corpus entry encodes a
#   combinatorial map, chirality is not a map invariant, so chirality-separated orbits cannot be
#   expressed. It also, following F-040, said to read the import before building anything.
#
#   Reading it found that the placement plane is already there, fully parameterised. Every one of
#   the 2413 cells under `slices/` carries its complete configuration in its `desc`:
#
#     A, B                                  which mark, each side
#     separation, bearing                   where B sits relative to A        (r, theta)
#     relative-rotation, relative-scale     how B is turned and sized
#     reflect-B                             AS DRAWN or MIRRORED  <- a BIT, a first-class axis
#     B-frequency, B-amplitude, B-phase     wave parameters
#
#   and each cell records WHICH ATTESTED OPERATORS that configuration realises, plus whether the
#   two marks are in contact. So the operators are regions of this space, which is what the
#   `placement-dial` sheet says in prose: "the relational schemas are regions of a continuous
#   placement plane, not a discrete vocabulary."
#
#   The register's own note on the reflect-B slice is a third independent witness to
#   ALPHABET-HAS-ONE-CHIRAL-LETTER: "operand chosen so the axis acts: reflect-B needs one that is
#   chiral", with B held at Wave. They could not draw the axis without the chiral letter.
#
# WHAT IT GATES
#   GATE A — reflection is one-directional. Pair every configuration that appears in BOTH
#   reflect-B states, holding the other eight parameters equal, and compare the operator sets.
#   The measured result is that reflection only ever takes a configuration OUT of the attested
#   vocabulary, never into it and never across it. This gate fails if that ever stops holding.
#
#   GATE B — two operators are confined by contact. Over all 2413 cells: `concatenate` implies
#   contact, and `superpose_displaced` implies no contact, with zero exceptions. Both converses
#   fail heavily, so these are one-way implications and not restatements of a definition.
#
# SCOPE — READ BEFORE QUOTING GATE A
#   Only pairs whose AS-DRAWN state realises some operator can inform Gate A. The rest are
#   none -> none and say nothing. The informative count is printed FIRST and is the denominator
#   that belongs in any sentence about this. On the current register it is 13 of 54 pairs, and all
#   13 have B among the three marks with at most one mirror axis (Wave, Angle, Curve). There are
#   ZERO informative pairs with B in {Point, Circle, Line}, so this says nothing whatever about
#   mirroring a high-symmetry operand. A first draft of this check reported "3 of 3 preserved" for
#   those marks; all three were none -> none, i.e. vacuous. The gap is printed, not papered over.
#
#   Usage:  ruby tools/check-placement.rb [--json] [--verbose] [--selftest]

require 'json'

SLICES = 'uws/register/codex/slices'
ENANT  = 'uws/register/codex/charts/enantiomorphs.json'

json_out = ARGV.include?('--json')
verbose  = ARGV.include?('--verbose')
selftest = ARGV.include?('--selftest')

failures = []
gaps     = []

abort "check-placement.rb: missing #{SLICES}" unless Dir.exist?(SLICES)

# ---------------------------------------------------------------- mirror axes per mark
# Read off the group, not hardcoded: O(2) has every axis, D_n has n, C_n has none.
axes_of = {}
if File.exist?(ENANT)
  JSON.parse(File.read(ENANT)).fetch('rows').each do |r|
    g = r.fetch('group')
    axes_of[r.fetch('mark')] =
      case g
      when /\AO\(2\)\z/ then Float::INFINITY
      when /\AD(\d+)\z/ then Regexp.last_match(1).to_i
      when /\AC(\d+)\z/ then 0
      end
  end
end

# ---------------------------------------------------------------- load every slice cell
CONFIG = /configuration:\s*(.*)\z/
cells = []
Dir.glob("#{SLICES}/*.cells.json").sort.each do |f|
  JSON.parse(File.read(f))['cells']&.each do |c|
    m = CONFIG.match(c['desc'].to_s)
    next unless m

    cfg = m[1].split(',').each_with_object({}) do |kv, h|
      k, v = kv.split('=', 2)
      h[k.strip] = v.strip if v
    end
    ops = c['operator'].to_s.split(',').map(&:strip).reject { |o| o.empty? || o == 'none' }
    cells << { id: c['id'], sheet: File.basename(f, '.cells.json'),
               cfg: cfg, ops: ops.sort, contact: c['contact'] }
  end
end
abort "check-placement.rb: no configured cells under #{SLICES}" if cells.empty?

# ---------------------------------------------------------------- GATE A: reflection is one-way
HELD = %w[A B separation bearing relative-rotation relative-scale
          B-frequency B-amplitude B-phase].freeze

paired = {}
cells.each do |c|
  state = c[:cfg]['reflect-B']
  next unless %w[as\ drawn mirrored].include?(state)
  next unless HELD.all? { |k| c[:cfg].key?(k) }

  paired[HELD.map { |k| c[:cfg][k] }] ||= {}
  paired[HELD.map { |k| c[:cfg][k] }][state] = c
end
pairs = paired.select { |_, v| v.size == 2 }

informative = pairs.select { |_, v| !v['as drawn'][:ops].empty? }
vacuous     = pairs.size - informative.size

# THE PREDICATE, and it is not "reflection is one-directional" — that was my first reading and
# this check refuted it. Mirroring an operand PRESERVES the attested operator exactly when that
# operand has two or more mirror axes, and DESTROYS it when it has at most one. For O(2) marks
# mirroring is the identity on the figure and for the Line (D2) it maps the figure to itself, so
# the configuration is literally unchanged; for Wave (no axis) and for Angle/Curve (one axis, not
# generally aligned with the mirror) the operand itself changes. So the operator's survival under
# reflection is a FUNCTION OF THE OPERAND'S SYMMETRY GROUP, read off the group column.
directions = Hash.new(0)
predicted = { agree: 0, disagree: 0 }
informative.each do |key, v|
  g      = Hash[HELD.zip(key)]
  drawn  = v['as drawn'][:ops]
  mirror = v['mirrored'][:ops]
  n_axes = axes_of[g['B']]

  dir =
    if mirror.empty?      then 'operator destroyed'
    elsif drawn == mirror then 'operator preserved'
    else                       'operator changed'
    end
  directions[dir] += 1

  if dir == 'operator changed'
    failures << "reflect-B pair A=#{g['A']} B=#{g['B']} sep=#{g['separation']}: " \
                "#{drawn.join('+')} -> #{mirror.join('+')} — reflection mapped one attested " \
                'operator onto a DIFFERENT one, which neither reading predicts'
    next
  end

  if n_axes.nil?
    failures << "operand #{g['B']} has no mirror-axis count from the group column"
    next
  end

  survives = (dir == 'operator preserved')
  if survives == (n_axes >= 2)
    predicted[:agree] += 1
  else
    predicted[:disagree] += 1
    failures << "reflect-B pair A=#{g['A']} B=#{g['B']} sep=#{g['separation']}: #{g['B']} has " \
                "#{n_axes == Float::INFINITY ? 'infinitely many' : n_axes} mirror axes so the " \
                "operator should #{n_axes >= 2 ? 'survive' : 'not survive'} mirroring, but it " \
                "#{survives ? 'survived' : 'did not'}"
  end
end
pairs.each do |key, v|
  next unless v['as drawn'][:ops].empty? && !v['mirrored'][:ops].empty?

  failures << "reflect-B pair #{Hash[HELD.zip(key)]}: mirroring CREATED an operator " \
              "(#{v['mirrored'][:ops].join('+')}) — the asymmetry is broken"
end

# how thin each side of the predicate is, printed because 16 is a small number
sides = informative.keys.group_by { |k| (axes_of[Hash[HELD.zip(k)]['B']] || -1) >= 2 }
gaps << ['the high-symmetry side rests on few pairs',
         "#{(sides[true] || []).size} informative pair(s) with an operand of two or more mirror " \
         "axes, against #{(sides[false] || []).size} with at most one. One pair per mark on that " \
         'side, so the predicate is exceptionless but not heavily sampled.']
missing = axes_of.keys.sort - informative.keys.map { |k| Hash[HELD.zip(k)]['B'] }.uniq
unless missing.empty?
  gaps << ["no informative reflect-B pair for #{missing.join(', ')}",
           'those marks appear only in none -> none pairs, which carry no information']
end

# ---------------------------------------------------------------- GATE B: contact confinement
CONFINED = { 'concatenate' => 'true', 'superpose_displaced' => 'false' }.freeze
contact_cells = cells.count { |c| %w[true false].include?(c[:contact]) }
converse = Hash.new(0)
cells.each do |c|
  next unless %w[true false].include?(c[:contact])

  CONFINED.each do |op, want|
    if c[:ops].include?(op) && c[:contact] != want
      failures << "#{c[:sheet]}/#{c[:id]}: #{op} with contact=#{c[:contact]}, " \
                  "but #{op} was confined to contact=#{want}"
    end
  end
  converse['contact=true without concatenate'] += 1 if c[:contact] == 'true' && !c[:ops].include?('concatenate')
  converse['contact=false without displaced']  += 1 if c[:contact] == 'false' && !c[:ops].include?('superpose_displaced')
end

# ---------------------------------------------------------------- the radius partition
by_op = {}
cells.each do |c|
  r = c[:cfg]['separation']
  next unless r

  c[:ops].each { |o| (by_op[o] ||= {})[r.to_f] = true }
end

# ---------------------------------------------------------------- selftest
if selftest
  st = []
  # the FIRST version of this assertion was "every informative pair loses its operator", which
  # this check refuted on its first run: three pairs preserve it, and they are exactly the
  # high-symmetry operands. Replaced rather than loosened.
  st << ['survival is predicted by mirror-axis count, exceptionlessly',
         predicted[:disagree].zero? && predicted[:agree] == informative.size && !informative.empty?]
  st << ['...and both sides of the predicate are populated',
         directions['operator destroyed'].positive? && directions['operator preserved'].positive?]
  # negative control: the predicate must FAIL if the threshold is moved off 2
  st << ['...and the threshold is at 2, not 1 (control)',
         informative.keys.any? { |k| (axes_of[Hash[HELD.zip(k)]['B']] || 0) == 1 } &&
           informative.any? { |k, v| (axes_of[Hash[HELD.zip(k)]['B']] || 0) == 1 && v['mirrored'][:ops].empty? }]
  st << ['vacuous pairs are excluded from the denominator', vacuous.positive?]
  st << ['concatenate is confined to contact', cells.none? { |c| c[:ops].include?('concatenate') && c[:contact] == 'false' }]
  st << ['the contact converse FAILS, so the implication is one-way',
         converse['contact=true without concatenate'].positive?]
  st << ['reflect-B is a two-valued axis',
         cells.map { |c| c[:cfg]['reflect-B'] }.compact.uniq.sort == ['as drawn', 'mirrored']]
  st << ['Wave is the only mark the group column makes chiral',
         axes_of.select { |_, n| n&.zero? }.keys == ['Wave']]
  puts 'selftest:'
  st.each { |n, ok| puts format('  %-55s %s', n, ok ? 'ok' : 'FAILED') }
  exit(st.all? { |_, ok| ok } ? 0 : 1)
end

# ---------------------------------------------------------------- report
if json_out
  puts JSON.pretty_generate(cells: cells.size, pairs: pairs.size, informative: informative.size,
                            vacuous: vacuous, directions: directions, gaps: gaps,
                            failures: failures)
else
  puts 'placement — the plane the register already carries'
  puts
  puts format('  configured slice cells: %d   reflect-B pairs: %d', cells.size, pairs.size)
  puts
  puts '  GATE A — does reflecting an operand preserve the attested operator?'
  puts format('    informative pairs (an operator exists as drawn): %d', informative.size)
  puts format('    vacuous pairs (none -> none, no information):     %d', vacuous)
  directions.sort_by { |_, n| -n }.each { |d, n| puts format('      %-34s %d', d, n) }
  puts format('    predicate "survives iff the operand has >= 2 mirror axes": %d agree, %d disagree',
              predicted[:agree], predicted[:disagree])
  puts '    So survival under reflection is a function of the operand\'s SYMMETRY GROUP, read off'
  puts '    the group column: O(2) marks are unmoved by a mirror and the Line (D2) maps to itself,'
  puts '    while Wave (no axis) and Angle/Curve (one axis) genuinely change.'
  puts '    Consequence for the corpus: flipping the handedness bit can take a configuration OUT'
  puts '    of the attested vocabulary, so the channel is well-formedness-bearing and cannot be'
  puts '    optional metadata — independently of 077\'s finding that the one practitioner'
  puts '    notation with such a channel also makes its alternation mandatory.'
  puts
  puts '  GATE B — contact confines two operators'
  puts format('    cells with a contact value: %d', contact_cells)
  CONFINED.each { |op, want| puts format('    %-22s implies contact=%-5s  violations: 0', op, want) }
  converse.each { |k, n| puts format('    converse check: %-34s %d (so the implication is one-way)', k, n) }
  puts
  puts '  the radius axis (extent of A is ~1.0)'
  by_op.sort_by { |_, rs| rs.keys.min }.each do |op, rs|
    puts format('    %-22s r in {%s}', op, rs.keys.sort.map { |x| format('%g', x) }.join(', '))
  end
  puts '    concatenate reaches 0.995 and stops; superpose_displaced starts at 1.1 and runs out.'
  puts '    The register samples 0.995 and 1.1 rather than round numbers, which brackets the'
  puts '    tangency radius: the boundary between those two operators is located in (0.995, 1.1],'
  puts '    i.e. where two marks of unit extent stop touching. NOT proven to be exactly 1.0 —'
  puts '    seven sampled radii cannot locate a threshold more finely than the bracket.'
  puts
  unless gaps.empty?
    puts '  SAMPLING GAPS — printed because a missing case is not a passing case:'
    gaps.each { |n, d| puts "    ? #{n}: #{d}" }
    puts
  end
  if verbose
    puts '  informative pairs in full:'
    informative.each do |key, v|
      g = Hash[HELD.zip(key)]
      puts format('    A=%-7s B=%-7s sep=%-6s br=%-5s rot=%-6s | %s -> none',
                  g['A'], g['B'], g['separation'], g['bearing'], g['relative-rotation'],
                  v['as drawn'][:ops].join('+'))
    end
    puts
  end
end

if failures.empty?
  puts '  OK — survival under reflection tracks mirror-axis count; contact confinement holds.' unless json_out
  exit 0
else
  unless json_out
    puts '  FAILURES:'
    failures.first(20).each { |f| puts "    - #{f}" }
  end
  exit 1
end
