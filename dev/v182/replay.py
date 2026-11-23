#!/usr/bin/env python3
"""v1.8.2 paired report-only replays. All mutation stays under this repository."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys

REPO=Path(__file__).resolve().parents[2]
OUT=REPO/'target/v182'
CANDIDATE_ONLY=False
sys.path.insert(0,str(REPO/'scripts'))
from v15_replay import cases,command_line


def run(args,log,cwd=REPO,env=None):
    log.parent.mkdir(parents=True,exist_ok=True)
    with log.open('w') as f:
        f.write('$ '+' '.join(map(str,args))+'\n');f.flush()
        p=subprocess.run(list(map(str,args)),cwd=cwd,env={**os.environ,**(env or {})},stdout=f,stderr=subprocess.STDOUT)
    assert p.returncode==0, f'replay failed ({p.returncode}): {log}'


def bertie():
    root = OUT/'bertie'
    original = (REPO/'target/v182/baseline-bertie.toml').read_text()
    for mode in ['separate', 'runtime']:
        for label in ['before', 'after']:
            config = original
            if label == 'after':
                config += '\n[fstar.extraction]\ninactive_features=["hax-pv"]\nreason="hax-driver.py cargo_hax_into enables hax-pv only for ProverIf, not extract-fstar"\n'
            (root/'formal-coverage.toml').write_text(config)
            name = f'bertie-{mode}-{label}'
            tool = OUT/('baseline' if label == 'before' else 'candidate')
            run([tool, 'report', '.', '--fstar-log', 'target/ci-logs/fstar/fstar.log',
                 '--ghost-lines', mode, '--json', OUT/f'{name}.json', '--markdown', OUT/f'{name}.md'],
                REPO/f'logs/v1.8.2/{name}.log', cwd=root)
            print(name+': PASS', flush=True)


def verus():
    for name,case in cases(['scratch-spec-lemmas','vest'],REPO/'fixtures/v1.5/v1.4.2-scratch-server.toml'):
        for mode in ['separate','runtime']:
            for label in (['after'] if CANDIDATE_ONLY else ['before','after']):
                output=OUT/f'{name}-{mode}-{label}';output.mkdir(parents=True,exist_ok=True)
                args,env=command_line(OUT/('baseline' if label=='before' else 'candidate'),case,output,['--size-mode','whole-crate','--ghost-lines',mode])
                env['TMPDIR']=str(OUT/'tmp');Path(env['TMPDIR']).mkdir(exist_ok=True)
                run(args,output/'report.log',env=env)
                print(f'{name} {mode} {label}: PASS',flush=True)


def fixtures():
    for name in ['classification','doc_comments','evidence','ghost','hax','inventory']:
        root=OUT/'fixtures'/name
        if not CANDIDATE_ONLY:
            if root.exists(): shutil.rmtree(root)
            shutil.copytree(REPO/'fixtures'/name,root,ignore=shutil.ignore_patterns('target'))
        for mode in ['separate','runtime']:
            for label in (['after'] if CANDIDATE_ONLY else ['before','after']):
                output=OUT/f'fixture-{name}-{mode}-{label}';output.mkdir(parents=True,exist_ok=True)
                run([OUT/('baseline' if label=='before' else 'candidate'),'report',root,'--size-mode','whole-crate','--ghost-lines',mode,
                     '--json',output/'report.json','--markdown',output/'report.md'],output/'report.log')
                print(f'fixture {name} {mode} {label}: PASS',flush=True)


def main():
    global CANDIDATE_ONLY
    p=argparse.ArgumentParser();p.add_argument('group',choices=['bertie','verus','fixtures']);p.add_argument('--candidate-only',action='store_true')
    args=p.parse_args();CANDIDATE_ONLY=args.candidate_only;globals()[args.group]()
if __name__=='__main__': main()
