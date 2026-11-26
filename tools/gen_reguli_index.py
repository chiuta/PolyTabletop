#!/usr/bin/env python3 -I
"""Regenerează docs/reguli/INDEX.md (jocurile 1–120) și INDEX2.md (121–…) din `polytabletop list`, HOTSEAT_SAFE și paginile de reguli existente.
Folosire:  python3 -I tools/gen_reguli_index.py [--bin CALE_POLYTABLETOP]
Paginile care încă nu există apar fără link, marcate „(în curs)”."""
import os, re, subprocess, sys
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
a = sys.argv
BIN = a[a.index('--bin') + 1] if '--bin' in a else os.environ.get('POLYTABLETOP_BIN', os.path.join(ROOT, 'target/release/polytabletop'))
SPLIT = 120
lst = subprocess.run([BIN, 'list'], capture_output=True, text=True, timeout=120).stdout
games = []
for l in lst.splitlines():
    if l.strip() and not l.startswith(' '):
        i, _, t = l.partition(' ')
        games.append((i, t.strip()))
lib = open(f'{ROOT}/polytabletop-games/src/lib.rs', encoding='utf8').read()
cat = dict((i, c) for i, c in re.findall(r'id: "([^"]+)",\s*title: "[^"]*",\s*category: "([^"]*)"', lib[lib.index('pub fn playable()'):]))
hot = set(re.findall(r'"([^"]+)"', re.search(r'pub const HOTSEAT_SAFE: &\[&str\] = &\[(.*?)\];', lib, re.S).group(1)))
def ai(i):
    f = f'{ROOT}/docs/reguli/{i}.md'
    if not os.path.exists(f): return '—'
    m = re.search(r'^Stil AI în cod: (.*)$', open(f, encoding='utf8').read(), re.M)
    s = m.group(1) if m else ''
    return 'alpha-beta cu timp' if 'buget de timp' in s else 'alpha-beta' if 'alpha-beta' in s else 'MCTS' if 'MCTS' in s else '—'
def row(n, i, t):
    p = f'{ROOT}/docs/reguli/{i}.md'
    link = f'[`{i}`]({i}.md)' if os.path.exists(p) else f'`{i}` (în curs)'
    return f'| {n} | {link} | {t} | {cat.get(i, "")} | {"da" if i in hot else "nu"} | {ai(i)} |'
N = len(games)
COMMON = """* Mutările se scriu exact ca în lista afișată de `?`; majusculele nu contează, spațiile se ignoră. `hint` cere sugestia AI-ului, `q` iese.
* Exemplele au fost produse rulând `polytabletop play <id> --seed 1` și introducând mutări legale; acolo unde AI-ul folosește un buget de timp sau simulări, răspunsul calculat poate diferi de la o rulare la alta.
* Jocurile cu informație ascunsă separă pașii care ar dezvălui informația (tragere, zaruri, revelare, verdict) în mutări forțate; AI-ul nu citește informația privată a adversarului.
* Coloana `HOTSEAT_SAFE` indică jocurile al căror afișaj respectă modul „la masă” (`--you both`): informația privată se arată doar celui la mutare. Jocurile cu „nu” fie nu au informație privată, fie nu ascund informația Jucătorului 1 în acest mod (vezi nota din fiecare pagină).
* „(în curs)” marchează jocurile a căror pagină de reguli încă nu a fost scrisă.
"""
HEAD = '| Nr. | Id | Titlu | Categorie | `HOTSEAT_SAFE` | AI |\n|---|---|---|---|---|---|\n'
def write(name, title, intro, lo, hi):
    body = '\n'.join(row(n, i, t) for n, (i, t) in enumerate(games[lo - 1:hi], lo))
    open(f'{ROOT}/docs/reguli/{name}', 'w', encoding='utf8').write(f'# {title}\n\n{intro}\n\nObservații comune:\n\n{COMMON}\n{HEAD}{body}\n')
write('INDEX.md', f'Reguli detaliate: jocurile 1–{SPLIT} (prima parte din `polytabletop list`)',
      'Fiecare pagină descrie, pentru implementarea din acest depozit: scopul, pregătirea, structura turei, sintaxa mutărilor (cu exemple rulate efectiv în CLI cu `--seed 1`), regulile speciale, punctajul, sfârșitul jocului, notele despre AI și informația ascunsă și diferențele față de regulile tradiționale.\n\n'
      f'Ordinea este cea din `polytabletop list`; partea a doua (jocurile {SPLIT + 1}–{N}) este în [INDEX2.md](INDEX2.md). Regulile scurte și sintaxa generală sunt și în `docs/JOCURI.md`.', 1, SPLIT)
write('INDEX2.md', f'Reguli detaliate: jocurile {SPLIT + 1}–{N} (partea a doua din `polytabletop list`)',
      f'Pagini de reguli în română pentru jocurile {SPLIT + 1}–{N}, în ordinea din `polytabletop list`. Primele {SPLIT} de jocuri sunt în [INDEX.md](INDEX.md).', SPLIT + 1, N)
print('jocuri', N, 'pagini lipsă', sum(1 for i, _ in games if not os.path.exists(f'{ROOT}/docs/reguli/{i}.md')))
