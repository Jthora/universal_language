#!/usr/bin/env ruby
# frozen_string_literal: true
#
# check-redundancy.rb — how much of the corpus checks itself?
#
# WHY THIS EXISTS (notes 073, 074)
#   073 seeded three errors and found they were NOT equally detectable. Two were caught by the
#   entry contradicting ITSELF: `faces: 3` on a triangle forces chi = 3, hence genus -0.5, against
#   the same block's published `genus: 0`. The third was not: [1,1,2,2] preserves sum(deg) = 2E and
#   its own length, so it agrees with every other number the entry publishes, and only recomputing
#   from the rotations refutes it.
#
#   That was an anecdote over three fields. This measures it over the whole corpus.
#
# THE QUESTION IT ANSWERS
#   For each published field: can it be corrupted CONSISTENTLY with every other published field?
#     - no  -> PINNED. The artifact detects this corruption with no oracle and no source to trust.
#     - yes -> FREE.   Detecting it requires recomputing from the primary data (rotations/n_darts).
#
#   The pinned fraction is the share of the published surface that is self-checking. It is a
#   property of the ARTIFACT, not of the reader (073 CORPUS-REDUNDANCY-CATCHES-TWO-OF-THREE), and
#   it is what survives when the source of an artifact is discredited: a pinned field does not
#   depend on anyone's word, so it does not fall with them (074).
#
#   PINNING IS REDUNDANCY, AND REDUNDANCY COSTS CAPACITY (064 CAPACITY-RADIUS-TRADEOFF). The
#   pinned fraction is this corpus's code rate seen from the detection side, so a high number is
#   not free and not automatically desirable.
#
# SCOPE — READ BEFORE QUOTING THE NUMBER
#   "Pinned" is always pinned AGAINST A PERTURBATION CLASS, never absolute. The class is printed
#   with the result and is deliberately small and local (see PERTURBATIONS below). A field pinned
#   here can still be corrupted by a large or coordinated edit that this check does not search.
#   This measures single-field corruption only: every identity is re-checked with exactly one
#   field changed.
#
#   Usage:  ruby tools/check-redundancy.rb [--json] [--verbose]

require 'json'

verbose = ARGV.include?('--verbose')
as_json = ARGV.include?('--json')

# --- the identities available from PUBLISHED FIELDS ALONE -------------------------------------
# No rotations, no n_darts: that is the point. A reader holding only the `expected` block can
# check exactly these.
def violations(e)
  v = []
  ds, vert, edg, fac = e['degree_sequence'], e['vertices'], e['edges'], e['faces']
  comp, gen, pf, ep   = e['components'], e['genus'], e['planar_faces'], e['euler_planar']

  # domain
  v << 'vertices>=1'  if vert && vert < 1
  v << 'edges>=1'     if edg  && edg  < 1
  v << 'faces>=1'     if fac  && fac  < 1
  v << 'components>=1' if comp && comp < 1
  v << 'genus>=0'     if gen && gen.negative?
  v << 'planar_faces>=1' if pf && pf < 1
  if ds.is_a?(Array)
    v << 'degrees>=1' if ds.any? { |d| !d.is_a?(Integer) || d < 1 }
    v << 'handshake: sum(deg)==2E'  if edg  && ds.sum != 2 * edg
    v << 'len(deg)==V'              if vert && ds.length != vert
  end
  # Euler, connected case only: chi = V-E+F = 2-2g
  if comp == 1 && gen.is_a?(Integer) && vert && edg && fac
    v << 'euler: V-E+F==2-2g' if (vert - edg + fac) != (2 - 2 * gen)
  end
  # genus is defined exactly when connected (this corpus's convention)
  v << 'genus defined iff connected' if comp && e.key?('genus') && (comp == 1) != !gen.nil?
  # planar reading: V-E+F_planar == 1+c, and euler_planar is that quantity
  if pf && vert && edg
    v << 'euler_planar==V-E+planar_faces' if ep && ep != (vert - edg + pf)
    v << 'planar euler: V-E+F_p==1+c'     if comp && (vert - edg + pf) != (1 + comp)
  end
  v
end

# --- the perturbation class, stated explicitly ------------------------------------------------
PERTURBATION_CLASS = [
  'integer fields: +/-1, +/-2',
  'genus: +/-1, +/-2, and null<->0',
  'degree_sequence: move one unit between two positions (sum- and length-preserving); ' \
  '+/-1 on one position; drop one element'
].freeze

def perturbations(field, val)
  out = []
  case val
  when Integer
    [-2, -1, 1, 2].each { |d| out << val + d }
    out << nil if field == 'genus'
  when nil
    out << 0 if field == 'genus'
    out += [1, 2] if field == 'genus'
  when Array
    return [] if val.length < 1
    # (a) move one unit between positions — preserves sum AND length. The 073 `008` class.
    val.each_index do |i|
      next if val[i] <= 1
      val.each_index do |j|
        next if i == j
        s = val.dup; s[i] -= 1; s[j] += 1
        out << s.sort
      end
    end
    # (b) +/-1 on one position
    val.each_index do |i|
      [-1, 1].each do |d|
        s = val.dup; s[i] += d
        out << s.sort if s[i] >= 1
      end
    end
    # (c) drop one element
    val.each_index { |i| out << (val[0...i] + val[i + 1..]).sort } if val.length > 1
  end
  out.uniq.reject { |p| p == val }
end

SKIP = %w[same_face different_face].freeze # relations, not counts: not checkable from counts alone

entries = Dir.glob('corpus/entries/*.json').sort.map { |f| JSON.parse(File.read(f)) }
abort 'no corpus entries found' if entries.empty?

rows = []
entries.each do |entry|
  exp = entry['expected'] or next
  baseline = violations(exp)
  exp.each_key do |field|
    next if SKIP.include?(field)
    perts = perturbations(field, exp[field])
    next if perts.empty?
    witness = perts.find do |p|
      trial = exp.merge(field => p)
      violations(trial).empty?
    end
    rows << { 'entry' => entry['id'], 'field' => field,
              'pinned' => witness.nil?, 'witness' => witness,
              'candidates' => perts.length,
              'entry_consistent' => baseline.empty? }
  end
end

pinned = rows.count { |r| r['pinned'] }
total  = rows.length
pct    = total.zero? ? 0.0 : (100.0 * pinned / total).round(1)

# --- self-test: this must reproduce 073's hand-seeded result, or one of them is wrong ---------
if ARGV.include?('--selftest')
  want = [
    ['001-triangle',  'faces',           true,  nil],
    ['002-hexagon',   'genus',           true,  nil],
    ['008-junction',  'degree_sequence', false, [1, 1, 2, 2]]
  ]
  bad = []
  want.each do |id, field, pinned, witness|
    r = rows.find { |x| x['entry'] == id && x['field'] == field }
    bad << "#{id}.#{field}: not measured" and next unless r
    bad << "#{id}.#{field}: pinned=#{r['pinned']}, 073 says #{pinned}" if r['pinned'] != pinned
    bad << "#{id}.#{field}: witness #{r['witness'].inspect}, 073 seeded #{witness.inspect}" if witness && r['witness'] != witness
  end
  if bad.empty?
    puts 'SELFTEST OK — reproduces notes/073 exactly: 001.faces and 002.genus pinned (caught by'
    puts '              internal redundancy), 008.degree_sequence FREE with witness [1, 1, 2, 2],'
    puts '              which is the value 073 seeded by hand. Two independent routes, one answer.'
    exit 0
  end
  puts 'SELFTEST FAILED:'
  bad.each { |b| puts "  x #{b}" }
  exit 1
end

if as_json
  puts JSON.pretty_generate('pinned' => pinned, 'checkable_fields' => total,
                            'pinned_percent' => pct,
                            'perturbation_class' => PERTURBATION_CLASS,
                            'fields' => rows)
  exit 0
end

puts '=== how much of the corpus checks itself ========================'
puts "checkable published fields: #{total}   PINNED: #{pinned}   FREE: #{total - pinned}   (#{pct}%)"
puts
puts 'PINNED = no single-field corruption in the class below is consistent with the rest of the'
puts '         entry, so the artifact detects it with no oracle and no source to trust.'
puts 'FREE   = a consistent corruption exists; only recomputation from rotations/n_darts refutes it.'
puts
puts 'perturbation class (pinning is always relative to this, never absolute):'
PERTURBATION_CLASS.each { |c| puts "  - #{c}" }
puts

free = rows.reject { |r| r['pinned'] }
unless free.empty?
  puts "FREE FIELDS (#{free.length}) — each needs the primary data, not the published block:"
  free.each do |r|
    puts format('  %-34s %-16s witness: %s', r['entry'], r['field'], r['witness'].inspect)
  end
  puts
end

bad = rows.select { |r| !r['entry_consistent'] }.map { |r| r['entry'] }.uniq
unless bad.empty?
  puts "WARNING: these entries are already internally inconsistent as published: #{bad.join(', ')}"
  puts
end
puts verbose ? rows.map { |r| format('  %-34s %-16s %s', r['entry'], r['field'], r['pinned'] ? 'pinned' : 'FREE') }.join("\n") : ''
puts 'OK — this is a measurement, not a gate. It never fails the build.'
