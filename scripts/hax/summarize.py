#!/usr/bin/env python3
"""Summarize actual reports and separately labelled static upstream models."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys

out = Path(sys.argv[1])
manifest = json.loads((out/'manifest.json').read_text())
result = {'method': 'Docker attempts with adapted shipped lock; compiled-files reports; rejected receipts credit nothing',
          'independent_acceptance': 'pending Dudley', 'trees': {}}
for tree in ['upstream', 'shipped']:
    work = out/'work'/tree
    report = json.loads((work/'report.json').read_text())
    models = json.loads((out/f'{tree}-models.json').read_text())
    first = report['summary']['first_party']['security_relevant']
    library = [d for d in report['dependencies'] if d['name'].startswith('libcrux')]
    code = lambda name: int((work/f'{name}.exit').read_text())
    rows = {}
    for module, row in models['modules'].items():
        rows[module] = row
    result['trees'][tree] = {
        'commit': manifest['bertie'][tree],
        'proven_extracted_sloc': first['statuses']['proven-extracted'],
        'first_party_security_sloc': first['sloc'],
        'proven_extracted_percent': 100*first['statuses']['proven-extracted']/first['sloc'] if first['sloc'] else 0,
        'rust_runtime_inventory_trusted_items': report['summary']['trusted_items'],
        'rust_runtime_inventory_trusted_sloc': report['summary']['trusted_sloc'],
        'rust_runtime_inventory_escapes_including_dependencies': report['summary']['escape_sites'],
        'credited_contract_kinds': dict(Counter(d['contract_kind'] for d in report.get('fstar_definitions', []) if d['classification']=='proven-extracted' and d['file'].endswith('.fst'))),
        'libcrux_sloc': sum(d['sloc'] for d in library if d['size_known']),
        'libcrux_unknown_sizes': [f"{d['name']}@{d['version']}" for d in library if not d['size_known']],
        'libcrux': [{k:d[k] for k in ['name','version','sloc','size_known','size_mode','size_reason','whole_crate_sloc']} for d in library],
        'exits': {name:code(name) for name in ['metadata-locked','metadata','lock-audit','extract','patches','typecheck-extraction','typecheck-panic-free','report']},
        'adapted_shipped_lock_sha256': hashlib.sha256((work/'Cargo.lock').read_bytes()).hexdigest(),
        'receipts': report['evidence_runs'],
        'archived_model_inventory_uncredited': rows,
        'worked_examples_uncredited': models['worked_examples'],
        'upstream_protocol_claims_uncredited': [w for w in report['warnings'] if w.startswith('Upstream model (uncredited):')],
        'size_fallbacks': report['size_fallbacks'],
    }
    # Actual Rust copies must remain identical to the immutable export.
    source = Path(sys.argv[2])/tree
    changed = [str(p.relative_to(source)) for p in source.rglob('*.rs') if p.read_bytes() != (work/p.relative_to(source)).read_bytes()]
    assert not changed, changed
    result['trees'][tree]['rust_export_identity'] = 'all Rust files byte-identical'
    manifest.setdefault('adapted_shipped_locks', {})[tree] = result['trees'][tree]['adapted_shipped_lock_sha256']
manifest['attempts'][0]['status'] = 'hax built; Fstar first environment failed, compatibility environment recorded separately'
for name in ['rust','hax','fstar','z3']:
    path = out/f'{name}-version.log'
    manifest[f'{name}_version_output'] = path.read_text(errors='replace').strip()
if (out/'z3.commit').exists():
    manifest['z3_commit'] = (out/'z3.commit').read_text().strip()
(out/'summary.json').write_text(json.dumps(result, indent=2)+'\n')
(out/'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
for tree,row in result['trees'].items():
    print(tree, 'proven-extracted', row['proven_extracted_sloc'], row['proven_extracted_percent'],
          'trusted',row['rust_runtime_inventory_trusted_items'],'escapes',row['rust_runtime_inventory_escapes_including_dependencies'],
          'libcrux',row['libcrux_sloc'],'exits',row['exits'])
