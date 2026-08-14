#!/usr/bin/env ruby
# encoding: utf-8
# check-index.rb — seed/INDEX.yaml is the machine-readable root map, and this makes it
# unable to lie (notes/054, F-031: prose does not execute).
#
# WHAT IT CHECKS
#   - every path named anywhere in the index exists in the tree
#   - every claim ID it cites exists in claims.yaml
#   - every stack-layer status is from the allowed set
#   - every phase carries a non-empty acceptance criterion
#
# USAGE
#   ruby tools/check-index.rb            # report
#   ruby tools/check-index.rb --strict   # exit 1 (CI mode)

require 'yaml'

ROOT = File.expand_path('..', __dir__)
Dir.chdir(ROOT)

ALLOWED_STATUS = %w[strong partial defined in-progress not-started].freeze

errors = []
index  = YAML.load_file('seed/INDEX.yaml')

claim_ids = File.read('claims.yaml', encoding: 'UTF-8')
                .scan(/^\s+- id:\s*([A-Z0-9-]+)\s*$/).flatten

check_path = lambda do |p, ctx|
  errors << "#{ctx}: path does not exist: #{p}" unless File.exist?(p)
end
check_claims = lambda do |ids, ctx|
  Array(ids).each do |cid|
    errors << "#{ctx}: claim not in registry: #{cid}" unless claim_ids.include?(cid)
  end
end

# entry points + roots
(index['entry_points'] || {}).each { |k, p| check_path.call(p, "entry_points.#{k}") }
%w[claims_root failures_root].each { |k| check_path.call(index[k], k) if index[k] }
(index['program_notes'] || []).each { |p| check_path.call(p, 'program_notes') }
check_claims.call(index['contract'], 'contract')

# the stack
(index['stack'] || {}).each do |layer, spec|
  (spec['paths'] || []).each { |p| check_path.call(p, "stack.#{layer}") }
  st = spec['status']
  unless ALLOWED_STATUS.include?(st)
    errors << "stack.#{layer}: status #{st.inspect} not in #{ALLOWED_STATUS.join('|')}"
  end
  check_claims.call(spec['claims'], "stack.#{layer}")
end

# the phases
(index['phases'] || {}).each do |phase, spec|
  st = spec['status']
  unless ALLOWED_STATUS.include?(st)
    errors << "phases.#{phase}: status #{st.inspect} not in #{ALLOWED_STATUS.join('|')}"
  end
  if spec['acceptance'].to_s.strip.empty?
    errors << "phases.#{phase}: no acceptance criterion — a phase without one cannot close"
  end
end

layers = (index['stack'] || {}).size
phases = (index['phases'] || {}).size
if errors.empty?
  puts "index: #{layers} layers, #{phases} phases, #{claim_ids.size} registry claims visible"
  puts 'OK — the index matches the tree and the registry.'
  exit 0
else
  errors.each { |e| puts "  x #{e}" }
  puts "\nFIX: the index is the machine-readable truth; make it match the tree, or the tree match it."
  exit(ARGV.include?('--strict') ? 1 : 0)
end
