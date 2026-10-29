#!/usr/bin/env python3
"""Rerun the pinned iovec harness with original flags and network disabled."""
import sys,json,subprocess,shlex,time
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]));import verify_runs as v
out=v.RUNS/'firecracker';m=json.loads((out/'manifest.json').read_text());image=m['image_id'];assert image=='sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0'
m=v.initialize('firecracker')
assert not subprocess.check_output(['docker','ps','--filter','name=fc-verify','--format','{{.Names}}'],text=True).strip()
h=next(h for h in json.loads((out/'harnesses.json').read_text())['harnesses'] if h['id'].endswith('::verify_write_to_iovec_0'))
original=next(s for s in m['steps'] if s['log']==h['log']);script=original['script'];label='release-iovec-rerun';target=Path(m['export']);name='fc-verify-firecracker-release-iovec'
cmd=['docker','run','--rm','--name',name,'--platform','linux/arm64','--network','none','--mount',f'type=bind,source={target},target={target},readonly','--mount',f'type=bind,source={out},target=/out','--workdir',str(target),'--env','CARGO_HOME=/out/cargo-home','--env','CARGO_TARGET_DIR=/out/target','--env','TMPDIR=/out/tmp','--env','CARGO_BUILD_JOBS=2',image,'bash','-c','mkdir -p /out/tmp; '+script]
start=time.monotonic()
with (out/f'{label}.log').open('w') as log:
 try:code=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT,timeout=2700).returncode
 except subprocess.TimeoutExpired:
  subprocess.run(['docker','stop','--time','10',name],stdout=log,stderr=subprocess.STDOUT);code=124
step=dict(label=label,command=cmd,script=script,image_id=image,exit_code=code,seconds=time.monotonic()-start,timeout_seconds=2700,log=f'{label}.log',log_sha256=v.sha((out/f'{label}.log').read_bytes()))
v.write(out/'release-rerun-manifest.json',dict(date='2026-10-29',previous_outcome=h,steps=[step]))
r=json.loads((out/(h['log']+'.evidence.json')).read_text());r.update(exit_code=code,log_sha256=step['log_sha256'],harnesses=[r['command'][r['command'].index('--harness')+1]])
v.write(out/f'{label}.log.evidence.json',r)
print(code,step['seconds'])
