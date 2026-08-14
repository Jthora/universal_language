#!/usr/bin/env ruby
# check-links.rb — verifies that internal file references resolve.
#
# WHY THIS EXISTS
#   Two prunes and a restructure moved or deleted ~180 files. Every pass left dangling references
#   behind, and each was found by a different ad-hoc grep. This makes the check repeatable.
#
# WHAT IT CHECKS
#   - markdown links:   [text](path/to/file.md)
#   - backtick paths:   `research/framework/foo.md`
#   - claims.yaml evidence / rationale / superseded_by paths
#
# DELIBERATE EXCEPTIONS
#   - FAILURES.md is append-only and immutable. Its references record what existed when each failure
#     was logged; they are accurate history, not rot.
#   - A path explicitly marked as gone — "since removed", "removed —", "(retired" — is intentional
#     historical reference in a note. Notes are the trajectory record and legitimately name deleted
#     files.
#
# USAGE
#   ruby tools/check-links.rb            # report
#   ruby tools/check-links.rb --strict   # exit 1 on failures (CI mode)

require 'yaml'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

SKIP_DIRS  = %w[.git/ ul-forge/target/ ul-forge/web/node_modules/ node_modules/].freeze
SKIP_FILES = %w[FAILURES.md].freeze
GONE = /since removed|removed —|removed --|\(retired|no longer exists|git history/i

def skip?(path)
  SKIP_DIRS.any? { |d| path.start_with?(d) } || SKIP_FILES.include?(path)
end

# Returns true if the reference is annotated as deliberately-gone in its surrounding context.
def annotated_gone?(line)
  line =~ GONE
end

failures = []
checked  = 0

Dir.glob('**/*.md').sort.each do |path|
  next if skip?(path)
  File.readlines(path, encoding: 'UTF-8').each_with_index do |line, i|
    refs = line.scan(/\]\(([^)#][^)]*\.md)[^)]*\)/).flatten
    refs += line.scan(/`((?:research|uws|ul-forge|tools|spec)\/[\w.\/-]+\.(?:md|rs|json|rb))`/).flatten
    refs.each do |ref|
      next if ref.start_with?('http', 'mailto')
      checked += 1
      target = ref.split('#').first
      resolved = File.exist?(File.join(File.dirname(path), target)) || File.exist?(target)
      next if resolved
      next if annotated_gone?(line)
      failures << "#{path}:#{i + 1}  ->  #{ref}"
    end
  end
end

# claims.yaml evidence paths must resolve unconditionally — no historical exemption.
if File.exist?('claims.yaml')
  YAML.load_file('claims.yaml')['claims'].each do |c|
    %w[evidence rationale superseded_by].each do |field|
      v = c[field]
      next unless v.is_a?(String) && v.include?('.md')
      checked += 1
      failures << "claims.yaml##{c['id']}  ->  #{v}  (#{field})" unless File.exist?(v.split('#').first)
    end
  end
end

if failures.empty?
  puts "OK — #{checked} internal references resolve."
  exit 0
end

puts "BROKEN REFERENCES: #{failures.size} of #{checked} checked\n\n"
failures.each { |f| puts "  #{f}" }
puts
puts 'FIX: repoint it, or — if the file is deliberately gone and the reference is historical —'
puts '     annotate the line ("since removed", "git history") so the intent is explicit.'

exit(ARGV.include?('--strict') ? 1 : 0)
