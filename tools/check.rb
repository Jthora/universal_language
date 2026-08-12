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
