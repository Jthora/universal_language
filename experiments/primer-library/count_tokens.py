import tiktoken
enc = tiktoken.get_encoding('cl100k_base')

files = [
    'original/primer.txt',
    'ablations/V1_ABL-PROSE.txt', 'ablations/V2_ABL-SYMBOL.txt', 'ablations/V3_ABL-STANDARD.txt',
    'ablations/V4_ABL-BRIDGE.txt', 'ablations/V5_ABL-LINEAR.txt', 'ablations/V6_ABL-NODAMP.txt',
    'ablations/V7_ABL-REORDER.txt',
    'controls/CT-1_dense-physics.txt', 'controls/CT-2_cross-domain-prose.txt',
    'controls/CT-3_scrambled-primer.txt', 'controls/CT-4_nonsense-math.txt',
    'negative-controls/NC-1_single-domain-math.txt', 'negative-controls/NC-2_primer-as-prose.txt',
    'negative-controls/NC-3_standard-physics.txt', 'negative-controls/NC-4_reversed-dissipation.txt',
    'negative-controls/NC-5_pseudo-esoteric.txt'
]

primer_tokens = None
for f in files:
    with open(f) as fh:
        text = fh.read()
    tokens = len(enc.encode(text))
    if primer_tokens is None:
        primer_tokens = tokens
    pct = ((tokens - primer_tokens) / primer_tokens * 100)
    flag = ' *** FIX' if abs(pct) > 5 else ' OK'
    print(f'{f:55s} tokens:{tokens:5d}  ({pct:+6.1f}%){flag}')

print(f'\nPrimer token count: {primer_tokens}')
print(f'Allowed range (±5%): {int(primer_tokens*0.95)} - {int(primer_tokens*1.05)}')
