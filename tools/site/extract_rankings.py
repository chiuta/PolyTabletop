#!/usr/bin/env python3 -I
"""Extrage topurile din fișierul tabelar (openpyxl) în tools/site/data/rankings.json și le mapează
la catalogul PolyTabletop (polytabletop-games/src/catalog.rs, generat de tools/gen_catalog.py).

Rulare (din rădăcina depozitului):
  python3 -I tools/site/extract_rankings.py [/calea/către/topuri_tabletop.xlsx]
Singurul pas care are nevoie de openpyxl; restul scripturilor folosesc doar biblioteca standard.
Nu importă și nu rulează tools/gen_catalog.py (acela scrie catalog.rs)."""
import re, sys, json, os
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
xlsx = sys.argv[1] if len(sys.argv) > 1 else '/home/claude/out/topuri_tabletop.xlsx'
import openpyxl

def key(n):
    return re.sub(r"\(.*?\)", "", n).strip().lower()

gc = open(f'{root}/tools/gen_catalog.py', encoding='utf8').read()
TRAD = set(re.search(r'TRAD = set\("""(.*?)"""', gc, re.S).group(1).split('|'))
cat = {}
for m in re.finditer(r'E\s*\{\s*name:\s*"((?:[^"\\]|\\.)*)",\s*category:\s*"([^"]*)",\s*origin:\s*"([^"]*)",\s*kind:\s*Kind::(\w+),\s*impl_id:\s*"([^"]*)",?\s*\}', open(f'{root}/polytabletop-games/src/catalog.rs', encoding='utf8').read()):
    cat[key(m.group(1).replace('\\"', '"'))] = (m.group(4), m.group(5))
sys.path.insert(0, f'{root}/tools/data')
import data_cat100 as c100, data_cont as dcont
TRAD |= set(c100.TRAD_NEW) | {key(n) for n, _, _ in dcont.NEW}   # noile jocuri tradiționale, când vor fi implementate => status `play`
for a, cn in c100.ALIAS.items():   # dubluri unificate în catalog: numele vechi din topurile pe regiuni se mapează la intrarea păstrată
    cat[a] = cat[cn]
src = open(f'{root}/polytabletop-games/src/lib.rs', encoding='utf8').read()
src = src[src.index('pub fn playable()'):]
playable = set(re.findall(r'id: "([^"]+)",\s*title:', src))

CATK = {'Clasic': 'classic', 'Abstract': 'abstract', 'Familie': 'family', 'Strategie': 'strategy', 'Party': 'party', 'Cooperativ': 'coop',
        'Cărți': 'cards', 'Rol (RPG)': 'rpg', 'Miniaturi': 'minis', 'TCG': 'tcg', 'Copii': 'kids'}
# status: play = joc tradițional jucabil; var = variantă proprie (titlu propriu) pentru un nume de marcă/modern;
#         kit = toolkit RPG/miniaturi/TCG generic; brand = neimplementat din motive de mărci; plan = tradițional planificat
wb = openpyxl.load_workbook(xlsx, read_only=True)
lists, unmapped = {}, []
for sheet, lid in (('Lume', 'world'), ('Europa', 'europe'), ('Africa', 'africa'), ('Asia', 'asia'), ('America de Nord', 'namerica'),
                   ('America de Sud', 'samerica'), ('Oceania', 'oceania'), ('Alte continente', 'other'), ('România', 'romania')):
    rows = [r for r in wb[sheet].iter_rows(min_row=2, values_only=True) if r[0] is not None]
    out = []
    for loc, joc, c, extra in rows:
        k = key(joc)
        e = cat.get(k)
        if e is None:
            unmapped.append((lid, joc)); st, gid = 'brand', ''
        else:
            kind, gid = e
            if kind == 'Implemented':
                st = 'play' if k in TRAD else 'var'
                assert gid in playable, (joc, gid)
            elif kind == 'Toolkit': st, gid = 'kit', ''
            elif kind == 'Traditional': st, gid = 'plan', ''
            else: st, gid = 'brand', ''
        out.append({'r': int(loc), 'n': joc, 'c': CATK[c], 'x': extra or '', 's': st, 'g': gid})
    assert len(out) == 100, (sheet, len(out))
    assert len({x['n'] for x in out}) == 100, sheet
    gs = [x['g'] for x in out if x['g']]
    if len(gs) != len(set(gs)):
        print('atenție: același joc jucabil de două ori în', lid, sorted({g for g in gs if gs.count(g) > 1}))
    lists[lid] = out
# topuri pe categorii (foile „Top <categorie>”, vezi build_category_sheets.py): exact 100 de intrări distincte pe categorie
CATL = ['Abstract', 'Clasic', 'Familie', 'Strategie', 'Party', 'Cooperativ', 'Cărți', 'Rol (RPG)', 'Miniaturi', 'TCG', 'Copii', 'Zaruri']
CATK['Zaruri'] = 'dice'
catcount = {}
for m in re.finditer(r'category:\s*"([^"]*)"', open(f'{root}/polytabletop-games/src/catalog.rs', encoding='utf8').read()):
    catcount[m.group(1)] = catcount.get(m.group(1), 0) + 1
cat_lists = {}
for cn in CATL:
    out = []
    for loc, joc, c, extra in [r for r in wb['Top ' + cn].iter_rows(min_row=2, values_only=True) if r[0] is not None]:
        k = key(joc)
        kind, gid = cat[k]
        if kind == 'Implemented':
            st = 'play' if k in TRAD else 'var'
            assert gid in playable, (joc, gid)
        elif kind == 'Toolkit': st, gid = 'kit', ''
        elif kind == 'Traditional': st, gid = 'plan', ''
        else: st, gid = 'brand', ''
        assert c == cn, (joc, c, cn)
        out.append({'r': int(loc), 'n': joc, 'c': CATK[cn], 'x': extra or '', 's': st, 'g': gid})
    assert len(out) == 100 == catcount[cn], (cn, len(out), catcount[cn])
    assert [x['r'] for x in out] == list(range(1, len(out) + 1)) and len({x['n'] for x in out}) == len(out), cn
    cat_lists[CATK[cn]] = out
notes = [r[0] for r in wb['Note'].iter_rows(values_only=True) if r[0]]
data = {'lists': lists, 'cat_lists': cat_lists, 'notes_ro': notes}
os.makedirs(f'{root}/tools/site/data', exist_ok=True)
json.dump(data, open(f'{root}/tools/site/data/rankings.json', 'w', encoding='utf8'), ensure_ascii=False, indent=0)
print('unmapped:', unmapped)
for lid, L in lists.items():
    import collections
    print(lid, dict(collections.Counter(x['s'] for x in L)))
print({k: len(v) for k, v in cat_lists.items()})
used = {x['g'] for L in lists.values() for x in L if x['g']}
print('jocuri jucabile referite în topuri:', len(used), 'din', len(playable))
