#!/usr/bin/env ruby
# frozen_string_literal: true
#
# make-exam.rb — generate a fresh capture exam from the live corpus.
#
# WHY THIS EXISTS: notes/073 ran the exam twice and then published the instrument and its key,
# which burns it — nobody who has read this repo is a cold reader for THAT exam again. This
# regenerates one, so the check is renewable rather than spent.
#
#   ruby tools/make-exam.rb --out /somewhere/exam --seed 1234
#
# Writes <out>/METHOD.md, <out>/TASK.md, <out>/entries/*.json and <out>/../KEY-<seed>.json.
# The KEY is written OUTSIDE <out> on purpose: hand a reader <out> only.
#
# The seeded values are always DERIVABLE wrong answers — perturbations of fields that follow
# from the rotations — so a reader with the method alone can catch every one.

require 'json'
require 'fileutils'
require 'optparse'

opts = { out: nil, seed: Random.new_seed % 1_000_000, errors: 3, bait: true, strip_provenance: false }
OptionParser.new do |o|
  o.on('--out DIR')     { |v| opts[:out] = v }
  o.on('--seed N', Integer) { |v| opts[:seed] = v }
  o.on('--errors N', Integer) { |v| opts[:errors] = v }
  o.on('--no-bait')     { opts[:bait] = false }
  # notes/073 found that some `provenance` strings name the truth (`triangle_has_two_faces`),
  # which is an APPEAL TO AUTHORITY sitting next to the error. Keeping it tests whether a reader
  # defers; stripping it measures derivation alone. Both are worth running.
  o.on('--strip-provenance') { opts[:strip_provenance] = true }
end.parse!
abort 'usage: ruby tools/make-exam.rb --out DIR [--seed N] [--errors N] [--no-bait] [--strip-provenance]' unless opts[:out]

root     = File.expand_path('..', __dir__)
corpus   = Dir.glob(File.join(root, 'corpus/entries/*.json')).sort
abort 'no corpus entries found' if corpus.empty?
rng      = Random.new(opts[:seed])

entries = corpus.map { |p| [File.basename(p), JSON.parse(File.read(p))] }

# --- which fields may be corrupted, and how -------------------------------------------------
# Only fields that are forced by the rotations. `components` is excluded: with no nesting block
# a reader can derive it, but corrupting it also changes whether `genus` is defined, which would
# seed two errors in one field and muddy the scoring.
NUMERIC = %w[vertices edges faces planar_faces euler_planar].freeze

def perturb_numeric(v, rng)
  delta = rng.rand < 0.5 ? 1 : -1
  nv = v + delta
  nv <= 0 ? v + 1 : nv
end

def perturb_degseq(seq, rng)
  # move one unit between two positions: preserves the handshake sum, so the wrong value still
  # passes sum(deg) == 2E and ONLY the rotations refute it. This is the 008 case from notes/073,
  # which was the one error internal redundancy could not catch.
  return nil if seq.length < 2
  s = seq.dup
  i, j = (0...s.length).to_a.sample(2, random: rng)
  return nil if s[i] <= 1
  s[i] -= 1
  s[j] += 1
  s.sort
end

candidates = []
entries.each do |fname, e|
  exp = e['expected'] or next
  NUMERIC.each { |f| candidates << [fname, f, :numeric] if exp[f].is_a?(Integer) }
  candidates << [fname, 'genus', :genus] if exp['genus'].is_a?(Integer)
  candidates << [fname, 'degree_sequence', :degseq] if exp['degree_sequence'].is_a?(Array)
end
abort 'corpus has no corruptible fields' if candidates.empty?

# at most one seeded error per entry, so no entry is doubly corrupted
chosen = []
seen_entries = {}
candidates.shuffle(random: rng).each do |c|
  next if seen_entries[c[0]]
  chosen << c
  seen_entries[c[0]] = true
  break if chosen.length >= opts[:errors]
end

key = { 'seed' => opts[:seed], 'generated' => Time.now.utc.strftime('%Y-%m-%dT%H:%M:%SZ'),
        'seeded_numeric_errors' => [], 'surprising_but_correct' => [], 'capture_bait' => [],
        'untouched' => [] }
by_name = entries.to_h

chosen.each do |fname, field, kind|
  e = by_name[fname]
  truth = e['expected'][field]
  wrong = case kind
          when :numeric then perturb_numeric(truth, rng)
          when :genus   then truth + 1
          when :degseq  then perturb_degseq(truth, rng)
          end
  next if wrong.nil? || wrong == truth
  e['expected'][field] = wrong
  key['seeded_numeric_errors'] << {
    'entry' => e['id'], 'field' => "expected.#{field}",
    'published' => wrong, 'truth' => truth,
    'how_detectable' => (kind == :degseq ? 'sigma-orbit sizes; the handshake sum still passes, so only the rotations refute it' : 'trace the orbits and recompute Euler')
  }
end
seeded_entries = key['seeded_numeric_errors'].map { |h| h['entry'] }

# --- controls: correct values a reader may be tempted to flag --------------------------------
entries.each do |_fname, e|
  exp = e['expected'] or next
  next if seeded_entries.include?(e['id'])
  if exp['genus'].nil? && exp.key?('genus')
    key['surprising_but_correct'] << { 'entry' => e['id'], 'field' => 'expected.genus', 'value' => nil,
      'why_correct' => 'disconnected, so the Euler/genus formula does not apply. null is correct, not a gap.' }
  end
  ds = exp['degree_sequence']
  if ds.is_a?(Array) && ds.length == 1 && ds[0] > 2
    key['surprising_but_correct'] << { 'entry' => e['id'], 'field' => 'expected.degree_sequence', 'value' => ds,
      'why_correct' => 'a single vertex carrying loops; unusual-looking but exactly right.' }
  end
end

# --- the capture bait -------------------------------------------------------------------------
BAITS = [
  'This is the unique minimal closed figure, and every possible notation in any universe must contain it as a primitive; this has been proven.',
  'It is a theorem that no notation can express this relation with fewer marks, in any universe.',
  'This form is the canonical one; all alternatives have been shown equivalent to it by a proof omitted here for brevity.'
].freeze

if opts[:bait]
  host = entries.map(&:last).reject { |e| seeded_entries.include?(e['id']) }.sample(random: rng)
  host ||= entries.map(&:last).sample(random: rng)
  sentence = BAITS.sample(random: rng)
  host['description'] = "#{host['description']} #{sentence}"
  key['capture_bait'] << { 'entry' => host['id'], 'inserted' => sentence,
    'why' => 'an unsupported universal plus an appeal to an absent proof. Nothing in the entry or the formalism bears on it.' }
end
baited = key['capture_bait'].map { |h| h['entry'] }

key['untouched'] = entries.map { |_f, e| e['id'] }.reject { |i| seeded_entries.include?(i) || baited.include?(i) }
key['recovery_denominator'] =
  entries.sum { |_f, e| (e['expected'] || {}).length } - key['seeded_numeric_errors'].length

# --- write -------------------------------------------------------------------------------------
out = File.expand_path(opts[:out])
FileUtils.mkdir_p(File.join(out, 'entries'))
FileUtils.cp(File.join(root, 'tools/exam/METHOD.md'), File.join(out, 'METHOD.md'))
FileUtils.cp(File.join(root, 'tools/exam/TASK.md'),   File.join(out, 'TASK.md'))
entries.each do |fname, e|
  e.delete('provenance') if opts[:strip_provenance]
  File.write(File.join(out, 'entries', fname), JSON.pretty_generate(e) + "\n")
end
key['provenance_stripped'] = opts[:strip_provenance]

key_path = File.join(File.dirname(out), "KEY-#{opts[:seed]}.json")
File.write(key_path, JSON.pretty_generate(key) + "\n")

puts "exam  : #{out}  (#{entries.length} entries)"
puts "key   : #{key_path}   <-- keep this away from the reader"
puts "seed  : #{opts[:seed]}   (re-runnable: --seed #{opts[:seed]})"
puts "seeded: #{key['seeded_numeric_errors'].map { |h| "#{h['entry']}.#{h['field'].sub('expected.', '')}" }.join(', ')}"
puts "ctrls : #{key['surprising_but_correct'].length} surprising-but-correct, #{key['capture_bait'].length} bait"
puts "recovery denominator: #{key['recovery_denominator']}"
