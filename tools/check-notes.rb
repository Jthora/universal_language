#!/usr/bin/env ruby
# encoding: utf-8
# check-notes.rb — the notes convention, enforced.
#
# WHY THIS EXISTS (notes/042)
#   The convention in research/notes/README.md was written and never applied. A sequential read
#   found two violations that every claim-level checker was blind to, because they all operate on
#   claims.yaml and the notes are a different representation:
#
#     1. TEN notes had two Status lines disagreeing — header "open", body "closed" — because
#        completing a note inserted a new line instead of editing the header.
#     2. ZERO notes carried a "Superseded by:" line, despite nine corrections naming what they
#        correct. Supersession was recorded FORWARD ONLY, so reading a corrected note in isolation
#        gave a superseded position with no signal.
#
#   Same failure class as F-028 and the claims propagation gap: a change recorded in one place,
#   with the places that depend on it left unaware.
#
# WHAT IT CHECKS
#   - exactly one Status line per note, with a recognised value
#   - a note whose Thread says it CORRECTS another must be matched by a Superseded-by on the target
#
# USAGE
#   ruby tools/check-notes.rb            # report
#   ruby tools/check-notes.rb --strict   # exit 1 (CI mode)

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

VALID = /\A(open|closed)\b/
errors = []
notes  = Dir.glob('research/notes/0*/README.md').sort

# id -> notes that claim to correct it
corrects = Hash.new { |h, k| h[k] = [] }

notes.each do |path|
  id   = File.basename(File.dirname(path))[0, 3]
  text = File.read(path, encoding: 'UTF-8')

  status = text.scan(/^\*\*Status:\*\* *(.+)$/).flatten
  if status.empty?
    errors << "#{id}: no Status line"
  elsif status.size > 1
    errors << "#{id}: #{status.size} Status lines — one source of truth, or they diverge silently"
  elsif status.first !~ VALID
    errors << "#{id}: Status #{status.first.inspect} is not open/closed"
  end

  thread = text[/^\*\*Thread:\*\* *(.+(?:\n(?!\*\*).+)*)/, 1].to_s
  # only targets governed by a "corrects" verb — "follows" is not supersession
  thread.scan(/correct\w*[^.;]*/).each do |seg|
    seg.scan(/`(\d{3})`/).flatten.each { |t| corrects[t] << id }
  end
end

# F-033: a cycle left open while later notes close around it is the deferral pattern that let
# the adversary front slide three times. Standing programs (type: decision) may stay open;
# investigative notes (cycle/correction) may not be lapped by more than LAP later notes.
LAP = 3
ids = notes.map { |p| File.basename(File.dirname(p))[0, 3].to_i }
max_id = ids.max || 0
notes.each do |path|
  id3 = File.basename(File.dirname(path))[0, 3]
  text = File.read(path, encoding: 'UTF-8')
  type   = text[/^\*\*Type:\*\* *(\w+)/, 1].to_s
  status = text[/^\*\*Status:\*\* *(\w+)/, 1].to_s
  next unless status == 'open' && %w[cycle correction].include?(type)
  lag = max_id - id3.to_i
  if lag > LAP
    errors << "#{id3}: open #{type} lapped by #{lag} later notes — danger-first queue (F-033): run it or close it"
  end
end

corrects.each do |target, by|
  path = notes.find { |p| File.basename(File.dirname(p)).start_with?(target) }
  next unless path
  text = File.read(path, encoding: 'UTF-8')
  sup  = text[/^\*\*Superseded by:\*\* *(.+)$/, 1].to_s
  by.uniq.each do |b|
    next if sup.include?(b)
    errors << "#{target}: corrected by #{b}, but carries no matching `Superseded by:` line"
  end
end

puts "notes checked: #{notes.size}   supersession links: #{corrects.values.flatten.uniq.size}"

if errors.empty?
  puts 'OK — status is single-valued and every correction is recorded in both directions.'
  exit 0
end

puts "\nVIOLATIONS (#{errors.size}):\n\n"
errors.each { |e| puts "  x #{e}" }
puts
puts 'FIX: one Status line per note (edit the header, do not add a second).'
puts '     A corrected note gets a `Superseded by:` line — the one permitted edit to a closed note.'
exit(ARGV.include?('--strict') ? 1 : 0)
