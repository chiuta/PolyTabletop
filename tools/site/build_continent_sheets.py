#!/usr/bin/env python3 -I
"""Adaugă (sau reface) în fișierul tabelar foile Africa, Asia, America de Nord, America de Sud, Oceania
(din tools/data/data_cont.py), în formatul foii „Alte continente”, și actualizează foaia „Note”.
Rulare: python3 -I tools/site/build_continent_sheets.py [/calea/către/topuri_tabletop.xlsx]
Apoi: python3 -I tools/site/extract_rankings.py"""
import re, sys, os
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
sys.path.insert(0, f'{root}/tools/data')
import data_cont as dc
import openpyxl
xlsx = sys.argv[1] if len(sys.argv) > 1 else '/home/claude/out/topuri_tabletop.xlsx'

def key(n):
    return re.sub(r"\(.*?\)", "", n).strip().lower()

cat = {}
for m in re.finditer(r'name:\s*"((?:[^"\\]|\\.)*)",\s*category:\s*"([^"]*)"', open(f'{root}/polytabletop-games/src/catalog.rs', encoding='utf8').read()):
    cat[key(m.group(1).replace('\\"', '"'))] = m.group(2)
wb = openpyxl.load_workbook(xlsx)
SHEETS = [s for s, _, _ in dc.LISTS]
for s in SHEETS:
    if s in wb.sheetnames:
        del wb[s]
tpl = wb['Alte continente']
for i, (sheet, lid, L) in enumerate(dc.LISTS):
    ws = wb.copy_worksheet(tpl)
    ws.title = sheet
    ws.freeze_panes = 'A2'
    for r in ws.iter_rows(min_row=2):
        for c in r:
            c.value = None
    seen = set()
    for n, line in enumerate(L, 1):
        joc, extra = [x.strip() for x in line.split('|')]
        k = key(joc)
        assert k in cat and k not in seen, (sheet, joc)
        seen.add(k)
        ws.cell(n + 1, 1, n); ws.cell(n + 1, 2, joc); ws.cell(n + 1, 3, cat[k]); ws.cell(n + 1, 4, extra)
    assert len(L) == 100, (sheet, len(L))
# ordine: Lume, Europa, continentele noi, Alte continente, România, Note
order = ['Lume', 'Europa'] + SHEETS + ['Alte continente', 'România'] + [s for s in wb.sheetnames if s.startswith('Top ')] + ['Note']
wb._sheets = [wb[s] for s in order]
# foaia Note
nt = wb['Note']
NEW_NOTES = [
    'Topurile pe continente (Africa, Asia, America de Nord, America de Sud, Oceania) sunt estimări editoriale, ca toate celelalte: combină tradiția locală de joc, popularitatea jocurilor internaționale din regiune și, unde există, date de piață. Fiecare are 100 de locuri distincte, mapate la catalogul PolyTabletop.',
    'Împărțirea pe continente: Turcia și Orientul Mijlociu sunt la Asia; Egiptul și Maghrebul la Africa; Mexicul, America Centrală și Caraibele la America de Nord; Australia, Noua Zeelandă, Hawaii și insulele Pacificului la Oceania; Rusia apare la Europa.',
    'Topurile pentru Africa, America de Sud și Oceania sunt, ca cel pentru România, mai puțin sigure decât celelalte: există puține statistici publice despre jocurile de masă din aceste regiuni, iar ultimele locuri sunt jocuri internaționale ordonate după prezența lor pe piețele regionale (Africa de Sud, Brazilia, Australia).',
    'Jocurile tradiționale regionale care nu sunt încă implementate (de exemplu Oware, Bao, Escoba, Burako, Two-up) au fost adăugate în catalog cu starea „planificat”; au reguli publice și nu au nume de marcă.',
    'Antarctica nu are top: nu are populație permanentă și nici o tradiție proprie de jocuri; stațiile de cercetare joacă jocurile aduse din țările de origine, care apar deja în celelalte topuri.',
]
rows = [r[0].value for r in nt.iter_rows() if r[0].value]
rows = [r for r in rows if r not in NEW_NOTES and not r.startswith('Toate ') and not r.startswith('Topurile pe categorii')]
cat_notes = [r for r in [r[0].value for r in nt.iter_rows() if r[0].value] if r.startswith('Topurile pe categorii')]
rows.append('Toate topurile (pe regiuni și pe categorii) sunt estimări editoriale, nu o clasificare oficială.')
# păstrează titlul + notele vechi; adaugă cele noi
final = rows[:-1] + NEW_NOTES + cat_notes + rows[-1:]
for r in nt.iter_rows():
    for c in r:
        c.value = None
for i, t in enumerate(final, 1):
    nt.cell(i, 1, t)
wb.save(xlsx)
print('foi:', wb.sheetnames, 'note:', len(final))
