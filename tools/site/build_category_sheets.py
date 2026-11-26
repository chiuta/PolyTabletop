#!/usr/bin/env python3 -I
"""Reface în fișierul tabelar foile „Top <categorie>”: câte un top mondial de exact 100 de jocuri pentru fiecare dintre cele 12 categorii
din catalog (polytabletop-games/src/catalog.rs). Ordinea (și rangul) vine din cercetarea din tools/data/cat100/*.json, îmbinată de
tools/data/build_cat100.py în tools/data/data_cat100.py (RANKING): locurile rămân în ordinea cercetării, dublurile dintre categorii sunt
păstrate într-una singură, iar locurile eliberate sunt completate la coadă cu următoarele jocuri reale. Coloanele sunt cele ale foii
„Alte continente”; „Extra” = originea din catalog. Actualizează și foaia „Note”. Topurile pe regiuni/continente rămân neschimbate.
Rulare: python3 -I tools/site/build_category_sheets.py [/calea/către/topuri_tabletop.xlsx]
Apoi: python3 -I tools/site/extract_rankings.py"""
import re, sys, os
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
import openpyxl
xlsx = sys.argv[1] if len(sys.argv) > 1 else '/home/claude/out/topuri_tabletop.xlsx'
CATS = ['Abstract', 'Clasic', 'Familie', 'Strategie', 'Party', 'Cooperativ', 'Cărți', 'Rol (RPG)', 'Miniaturi', 'TCG', 'Copii', 'Zaruri']
PREFIX = 'Top '

def key(n):
    return re.sub(r"\(.*?\)", "", n).strip().lower()

cat = []
for m in re.finditer(r'name:\s*"((?:[^"\\]|\\.)*)",\s*category:\s*"([^"]*)",\s*origin:\s*"([^"]*)"', open(f'{root}/polytabletop-games/src/catalog.rs', encoding='utf8').read()):
    cat.append((m.group(1).replace('\\"', '"'), m.group(2), m.group(3)))
assert {c for _, c, _ in cat} == set(CATS), {c for _, c, _ in cat} ^ set(CATS)
wb = openpyxl.load_workbook(xlsx)
for s in [s for s in wb.sheetnames if s.startswith(PREFIX)]:
    del wb[s]
sys.path.insert(0, f'{root}/tools/data')
import data_cat100 as c100
catinfo = {key(n): (n, c, o) for n, c, o in cat}
assert len(catinfo) == len(cat) == 100 * len(CATS), (len(catinfo), len(cat))
tpl = wb['Alte continente']
counts = {}
for c in CATS:
    L = c100.RANKING[c]
    assert len(L) == 100 and [e['r'] for e in L] == list(range(1, 101)), c
    ws = wb.copy_worksheet(tpl); ws.title = PREFIX + c; ws.freeze_panes = 'A2'
    for r in ws.iter_rows(min_row=2):
        for x in r: x.value = None
    for p, e in enumerate(L, 1):
        n, cc, o = catinfo[key(e['name'])]
        assert n == e['name'] and cc == c, (c, e['name'], n, cc)
        ws.cell(p + 1, 1, p); ws.cell(p + 1, 2, n); ws.cell(p + 1, 3, c); ws.cell(p + 1, 4, o)
    counts[c] = len(L)
order = [s for s in wb.sheetnames if s not in ('Note',) and not s.startswith(PREFIX)] + [PREFIX + c for c in CATS] + ['Note']
wb._sheets = [wb[s] for s in order]
NOTE = ('Topurile pe categorii (câte unul pentru fiecare categorie din catalog: Abstract, Clasic, Familie, Strategie, Party, Cooperativ, Cărți, Rol (RPG), Miniaturi, TCG, Copii, Zaruri) sunt topuri mondiale de câte 100 de jocuri. '
        'Ordinea este cea din cercetare (estimare editorială după popularitate și răspândire); un joc apare într-o singură categorie, iar locurile rămase libere după eliminarea dublurilor au fost completate la coadă cu următoarele jocuri reale. '
        'Jocurile cu reguli publice sunt marcate „planificat” în catalog; denumirile de marcă primesc în program titluri proprii.')
FINAL = 'Toate topurile (pe regiuni și pe categorii) sunt estimări editoriale, nu o clasificare oficială.'
nt = wb['Note']
rows = [r[0].value for r in nt.iter_rows() if r[0].value]
rows = [r for r in rows if not r.startswith('Toate ') and not r.startswith('Topurile pe categorii')]
final = rows + [NOTE, FINAL]
for r in nt.iter_rows():
    for c in r: c.value = None
for i, t in enumerate(final, 1): nt.cell(i, 1, t)
wb.save(xlsx)
print('foi:', wb.sheetnames); print(counts)
