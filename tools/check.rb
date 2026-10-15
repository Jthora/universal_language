#!/usr/bin/env ruby
# check.rb — runs every repository check. This is what CI invokes.
#
#   ruby tools/check.rb            # report everything, exit 0
#   ruby tools/check.rb --strict   # exit 1 if any check fails (CI mode)

strict = ARGV.include?('--strict')
checks = [
  ['claim registry tier discipline', 'check-claims.rb'],
  ['internal references resolve',    'check-links.rb'],
  ['no retired counts in the tree',  'check-retired-content.rb'],
  ['caveats travel with citations',  'check-caveats.rb'],
  ['scope corrections propagate',    'check-propagation.rb'],
  ['notes convention holds',         'check-notes.rb'],
  ['seed index matches the tree',    'check-index.rb'],
  ['derivation premises propagate', 'check-derivation-premises.rb'],
  ['stated counts are generated', 'check-counts.rb'],
  # A MEASUREMENT, not a gate: it reports how much of the corpus checks itself and never fails
  # the build (note 074). It runs here so the number is in front of every reader who verifies,
  # rather than needing to be remembered.
  ['corpus self-checking (measurement)', 'check-redundancy.rb'],
  # A GATE, and the first check that reads a VENDORED import against ITSELF rather than reading
  # this repo against itself (note 076). It fails if the derivation of the handedness column stops
  # holding, or if the two sheets that currently contradict each other on orbits 40 and 53 stop
  # being adjudicated by the third. The repo had good apparatus for its own tree and none for an
  # import's internal consistency, which is how a 374-file register sat here for weeks with a
  # four-cell hole in it.
  ['imported handedness column derives', 'check-chirality.rb'],
]

failed = []
checks.each do |name, script|
  puts "\n=== #{name} " + '=' * [0, 60 - name.length].max
  # always run children in strict mode so we get an accurate exit code;
  # this script decides what to do with the result.
  ok = system(RbConfig.ruby, File.join(__dir__, script), '--strict')
  failed << name unless ok
end

puts "\n" + '=' * 62
if failed.empty?
  puts 'ALL CHECKS PASS'
  exit 0
end
puts "FAILED: #{failed.join(', ')}"
exit(strict ? 1 : 0)
