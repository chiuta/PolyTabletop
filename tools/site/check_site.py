#!/usr/bin/env python3 -I
"""Verificări pentru docs/: HTML parsabil, JSON-uri, chei i18n în 8 limbi, linkuri relative, 9x100 rânduri + 12 topuri pe categorii (exact 100 fiecare), o pagină pe joc jucabil.
Rulare din rădăcina depozitului: python3 -I tools/site/check_site.py"""
import os, re, json, sys, glob
from html.parser import HTMLParser
from urllib.parse import urlparse, unquote
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
docs = f'{root}/docs'
LANGS = ['en', 'ro', 'fr', 'it', 'es', 'pt', 'de', 'hu']
err, media_missing = [], set()
class P(HTMLParser):
    def __init__(s): super().__init__(); s.refs = []; s.ids = set(); s.stack = []
    def handle_starttag(s, tag, a):
        a = dict(a)
        if 'id' in a: s.ids.add(a['id'])
        for k in ('href', 'src', 'poster'):
            if k in a and a[k]: s.refs.append((tag, k, a[k]))
pages = sorted(glob.glob(f'{docs}/*.html') + glob.glob(f'{docs}/jocuri/*.html'))
ids = {}
for f in pages:
    t = open(f, encoding='utf8').read()
    p = P(); p.feed(t); p.close(); ids[f] = p.ids
    for tag, k, v in p.refs:
        if re.match(r'(https?:|mailto:|data:|#)', v) and not v.startswith('#'): continue
        path, _, frag = v.partition('#'); path = path.split('?')[0]
        tgt = f if not path else os.path.normpath(os.path.join(os.path.dirname(f), unquote(path)))
        if '/media/' in tgt.replace('\\', '/'):
            if not os.path.exists(tgt): media_missing.add(os.path.relpath(tgt, docs))
            continue
        if os.path.isdir(tgt): tgt = os.path.join(tgt, 'index.html')
        if not os.path.exists(tgt): err.append(f'{os.path.relpath(f, docs)}: {k}={v} lipsește'); continue
for f in glob.glob(f'{docs}/**/*.json', recursive=True) + glob.glob(f'{root}/tools/site/data/*.json') + [f'{docs}/manifest.webmanifest']:
    try: json.load(open(f, encoding='utf8'))
    except Exception as e: err.append(f'JSON {f}: {e}')
# JSON-LD
for f in glob.glob(f'{docs}/jocuri/*.html'):
    m = re.search(r'<script type="application/ld\+json" id="ld">(.*?)</script>', open(f, encoding='utf8').read(), re.S)
    if m: json.loads(m.group(1))
# chei i18n: fiecare data-i* din fiecare pagină există în T pentru toate limbile și nu e gol
for f in glob.glob(f'{docs}/jocuri/*.html') + [f'{docs}/index.html']:
    t = open(f, encoding='utf8').read()
    if f.endswith('index.html') and '/jocuri/' not in f:
        continue
    m = re.search(r'const TT=(\{.*?\});\nconst T=', t, re.S)
    TT = json.loads(m.group(1))
    T = {l: {k: v[i] for k, v in TT.items()} for i, l in enumerate(LANGS)}
    keys = set(re.findall(r'data-(?:i|ia|ialt)="(\w+)"', t))
    for l in LANGS:
        for k in keys:
            if not T.get(l, {}).get(k): err.append(f'{os.path.basename(f)}: cheia {k} lipsește în {l}')
    if len(set(len(T[l]) for l in LANGS)) != 1: err.append(f'{f}: număr de chei diferit între limbi')
# index: T + TX
t = open(f'{docs}/index.html', encoding='utf8').read()
rk = json.load(open(f'{root}/tools/site/data/rankings.json', encoding='utf8'))['lists']
rows = {k: len(v) for k, v in rk.items()}
print('rânduri clasamente', rows)
assert all(v == 100 for v in rows.values()) and len(rows) == 9
cl = json.load(open(f'{root}/tools/site/data/rankings.json', encoding='utf8'))['cat_lists']
cnt = {}
for m in re.finditer(r'category: "([^"]*)"', open(f'{root}/polytabletop-games/src/catalog.rs', encoding='utf8').read()):
    cnt[m.group(1)] = cnt.get(m.group(1), 0) + 1
print('topuri pe categorii', {k: len(v) for k, v in cl.items()})
allc = [x['n'] for v in cl.values() for x in v]
if len(cl) != 12 or sum(len(v) for v in cl.values()) != sum(cnt.values()) or any(len(v) != 100 for v in cl.values()):
    err.append('topurile pe categorii nu au exact 100 de intrări / nu acoperă catalogul')
if len(set(allc)) != len(allc) or set(cnt.values()) != {100}:
    err.append('dubluri între topurile pe categorii sau catalog cu categorii ≠ 100')
for k, v in cl.items():
    if [x['r'] for x in v] != list(range(1, 101)): err.append(f'rang neconsecutiv în topul {k}')
for k, v in cl.items():
    print('top', k, 'jucabile', sum(1 for x in v if x['s'] in ('play', 'var')), 'din', len(v))
n_pages = len(glob.glob(f'{docs}/jocuri/*.html')) - 1
print('pagini de joc', n_pages)
import re as _re
_n_play = len(_re.findall(r'id: "[^"]+",\s*title:', open(f'{root}/polytabletop-games/src/lib.rs', encoding='utf8').read().split('fn playable_base()', 1)[1] + ''.join(open(f'{root}/polytabletop-games/src/{r}.rs', encoding='utf8').read() for r in ('q6_reg', 'b1_reg'))))
if n_pages != _n_play or n_pages < 240: err.append(f'pagini {n_pages} != jocuri jucabile {_n_play}')
sizes = [os.path.getsize(f) for f in glob.glob(f'{docs}/jocuri/*.html') if not f.endswith('index.html')]
print('dimensiune index', os.path.getsize(f'{docs}/index.html'), 'pagini max', max(sizes))
if max(sizes) > 64000: err.append('pagină > 64 KB')
print('lipsă media (încă negenerată):', len(media_missing))
print('ERORI:' if err else 'OK: nicio eroare', *err[:40], sep='\n')
sys.exit(1 if err else 0)
