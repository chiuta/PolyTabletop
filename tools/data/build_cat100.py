#!/usr/bin/env python3 -I
"""Validează și îmbină cele 12 topuri de 100 pe categorii (cercetare în tools/data/cat100/*.json) într-un fișier de date
tools/data/data_cat100.py, folosit de tools/gen_catalog.py (intrări noi) și de tools/site/{build_category_sheets,extract_rankings}.py.

Rulare (din rădăcina depozitului):  python3 -I tools/data/build_cat100.py          (idempotent: rescrie data_cat100.py)
                                     python3 -I tools/data/build_cat100.py --check   (doar raportează)

Intrări:
  tools/data/cat100/<categorie>.json   cercetarea brută ({category, ranking:[{r,name,existing,origin,year,ctx,own_title_ro,
                                       public_rules,notes_ro,sources}]}); nu se editează.
  tools/data/cat100/extra.json         jocuri reale adăugate manual ca înlocuitori, în ordinea priorității, pentru locurile
                                       rămase libere după dedublare (câte sunt necesare, restul sunt ignorate).
  jocurile deja existente în catalog se recunosc după tools/data/data_{world,rest,cont}.py (NU după catalog.rs, ca scriptul
  să rămână idempotent după regenerarea catalogului).

Reguli de dedublare (de mai jos): MERGE (aceeași intrare sub nume diferite -> una canonică + alias-uri), KEEP (același joc
în mai multe categorii -> rămâne într-una singură), RENAME (nume afișat; cheia fără paranteze rămâne aceeași).
Rangul din cercetare se păstrează ca ordine; locurile eliberate se închid, iar înlocuitorii se așază la coadă."""
import re, sys, json, os, glob, pprint, collections

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import data_world as dw, data_rest as dr, data_cont as dc

CATS = ['Abstract', 'Clasic', 'Familie', 'Strategie', 'Party', 'Cooperativ', 'Cărți', 'Rol (RPG)', 'Miniaturi', 'TCG', 'Copii', 'Zaruri']
FILES = {'Abstract': 'abstract', 'Clasic': 'clasic', 'Familie': 'familie', 'Strategie': 'strategie', 'Party': 'party', 'Cooperativ': 'cooperativ',
         'Cărți': 'carti', 'Rol (RPG)': 'rpg', 'Miniaturi': 'miniaturi', 'TCG': 'tcg', 'Copii': 'copii', 'Zaruri': 'zaruri'}
TOOLKIT = {'Rol (RPG)', 'Miniaturi', 'TCG'}


def key(n):
    return re.sub(r"\(.*?\)", "", n).strip().lower()


# ---- intrări deja existente în catalog (aceeași logică de dedublare ca în gen_catalog.py)
base = collections.OrderedDict()
rows = [(n, c, o) for L in (dw.LUME, dw.EUROPA, dr.ALTE) for n, c, o in L] + [(n, c, "") for n, c, _ in dr.ROMANIA] + list(dc.NEW)
for n, c, o in rows:
    base.setdefault(key(n), (n, c))

# ---- reguli de dedublare ------------------------------------------------------------------------------------------
# MERGE: (categorie canonică, nume canonic așa cum apare în JSON, nume afișat final, [(categorie, nume alias), ...])
MERGE = [
    ('Clasic', 'Battleship (Bătălia navală)', None, [('Clasic', 'Bătălia navală')]),
    ('Copii', 'Guess Who? (Cine e?)', None, [('Copii', 'Cine e? (Guess Who?)')]),
    ('Familie', 'Nu te supăra, frate (Ludo)', 'Nu te supăra, frate (Ludo, Mensch ärgere dich nicht)',
     [('Familie', 'Mensch ärgere dich nicht'), ('Familie', 'Mensch ärgere dich nicht / Ludo'), ('Familie', 'Ludo')]),
    ('Abstract', "Moara (Nine Men's Morris)", "Moara (Țintar, Nine Men's Morris)", [('Abstract', 'Țintar (Moara)')]),
    ('Cărți', 'Șase-zeci și șase (66)', 'Șase-zeci și șase (Santase, 66)', [('Cărți', 'Santase (66)')]),
    ('Cărți', 'Remi (Rummy)', None, [('Cărți', 'Rummy (13 cărți)')]),
    ('Party', 'Werewolf / Mafia', 'Werewolf / Mafia (Vârcolacii, Les Loups-Garous)',
     [('Party', 'Les Loups-Garous de Thiercelieux'), ('Party', 'Werewolf (langren sha)')]),
    ('Abstract', 'Tic-tac-toe (X și 0)', None, [('Clasic', 'Tic-tac-toe'), ('Copii', 'X și 0 (Tic-tac-toe)')]),
    ('Clasic', 'Chinese Checkers', 'Chinese Checkers (Table chinezească, Sternhalma)',
     [('Abstract', 'Table chinezească (Chinese Checkers)'), ('Familie', 'Chinese Checkers (Sternhalma)')]),
    ('Cărți', 'Solitaire (Klondike)', None, [('Copii', 'Klondike')]),
    ('Clasic', 'Marbles', None, [('Copii', 'Bile (Marbles)')]),
    ('Copii', 'Mikado (Pick-up sticks)', None, [('Clasic', 'Pick-up sticks')]),
    ('Copii', 'Spânzurătoarea (Hangman)', None, [('Clasic', 'Hangman')]),
    ('Party', 'Chinese Whispers (Telefonul fără fir)', None, [('Copii', 'Telefonul fără fir (Chinese whispers)')]),
    ('Copii', 'Scaunele muzicale', None, [('Party', 'Musical Chairs (Scaunele muzicale)')]),
    ('Copii', 'Simon zice (Simon says)', None, [('Party', 'Simon Says (Simon zice)')]),
    ('Familie', 'Pay Day', None, [('Clasic', 'Payday')]),
    ('Zaruri', 'Knucklebones (Tali)', None, [('Clasic', 'Knucklebones (Jacks)')]),
    ('Clasic', 'Domino', None, [('Clasic', 'Draw dominoes')]),
]
# KEEP: același joc (aceeași cheie) în mai multe categorii -> rămâne doar în categoria indicată
KEEP = {
    'halma': 'Clasic', 'hnefatafl': 'Abstract', 'alquerque': 'Abstract', 'blokus': 'Abstract', 'qwirkle': 'Familie', 'sorry!': 'Familie',
    'mouse trap': 'Copii', 'sequence': 'Familie', 'trouble': 'Copii', 'mexican train': 'Clasic', 'upwords': 'Familie', 'careers': 'Familie',
    # jocuri mutate la Zaruri (moved_from în zaruri.json); Two-up rămâne la Clasic (nu are zaruri: se aruncă monede)
    'sic bo': 'Zaruri', "liar's dice": 'Zaruri', 'generala': 'Zaruri', 'farkle': 'Zaruri', 'bunco': 'Zaruri', 'barbut': 'Zaruri',
    'crown and anchor': 'Zaruri', 'yahtzee': 'Zaruri', 'two-up': 'Clasic',
    'arkham horror: the card game': 'Cooperativ', 'the mind': 'Cooperativ', 'marvel champions: the card game': 'Cooperativ',
    'the lord of the rings: the card game': 'Cooperativ', 'legendary: a marvel deck building game': 'Cooperativ',
    'pandemic: the cure': 'Zaruri', 'elder sign': 'Cooperativ', 'escape: the curse of the temple': 'Cooperativ', 'concept': 'Party',
    'outfoxed!': 'Copii', 'zombie kidz evolution': 'Copii', 'operation': 'Copii', 'hungry hungry hippos': 'Copii', 'halli galli': 'Copii',
    'jungle speed': 'Party', 'boggle': 'Party', 'cranium': 'Party', 'king of tokyo': 'Zaruri', 'sagrada': 'Zaruri', 'qwixx': 'Zaruri',
    "can't stop": 'Zaruri', 'bananagrams': 'Familie', 'scattergories': 'Party', 'stone age': 'Strategie', 'tikal': 'Familie',
    'heckmeck am bratwurmeck': 'Zaruri', 'lords of waterdeep': 'Strategie', 'taj mahal': 'Strategie', 'ra': 'Strategie', 'yspahan': 'Zaruri',
    'railroad ink': 'Zaruri', 'pass the pigs': 'Zaruri', 'star realms': 'Strategie', 'troyes': 'Zaruri',
    'ascension: chronicle of the godslayer': 'Strategie',
}
# nume diferite pentru produse distincte care ar avea aceeași cheie (fără paranteze)
RENAME_RECORD = {('Rol (RPG)', 'Legend of the Five Rings'): 'Legend of the Five Rings RPG'}
# corecturi de fapte verificate pe Wikipedia (câmp în JSON-ul cercetat -> valoare corectă); cercetarea brută nu se editează
FIELD_FIX = {('Rol (RPG)', 'Mythras'): {'year': None}, ('Abstract', 'Onitama'): {'origin': 'Japonia'}}
# Kind: Traditional dacă regulile sunt publice și jocul e vechi (<= 1930) / fără an / fără titlu propriu diferit; altfel Branded
OLD_YEAR = 1930


# origini: normalizare (aceleași denumiri ca în tools/site/data/origins.json, ca să se poată traduce în 8 limbi)
ORIGIN_FULL = {'China / Avatar': 'SUA', 'Antichitate / global': 'Antic / Global'}
ORIGIN_TOK = {'Anglia': 'Marea Britanie', 'Țările de Jos': 'Olanda', 'Roma Antică': 'Roma antică', 'global': 'Global', 'China (Hong Kong)': 'Hong Kong',
              'China (Sichuan)': 'China', 'SUA (Texas)': 'SUA', 'SUA (Wisconsin)': 'SUA', 'SUA (din Hazard)': 'SUA', 'Marea Britanie (Royal Navy)': 'Marea Britanie',
              'Germania (Bavaria)': 'Germania', 'Spania (Țara Bascilor)': 'Spania', 'Aztecii (Mexic)': 'Mexic', 'India (Marea Britanie)': 'India',
              'Orient': 'Orientul Mijlociu', 'Antichitate': 'Antic', 'Noua Zeelandă': 'Noua Zeelandă', 'Persia': 'Persia'}


def norm_origin(o):
    o = (o or '').strip()
    if not o or o == '—':
        return '—'
    o = ORIGIN_FULL.get(o, o)
    toks = []
    for t in re.split(r'\s*/\s*', o):
        t = ORIGIN_TOK.get(t.strip(), t.strip())
        if t and t not in toks:
            toks.append(t)
    return ' / '.join(toks)


def load():
    data = {}
    for c in CATS:
        d = json.load(open(f'{HERE}/cat100/{FILES[c]}.json', encoding='utf8'))
        assert d['category'] == c and len(d['ranking']) == 100, c
        data[c] = d['ranking']
    return data


def main(check=False):
    data = load()
    extra = json.load(open(f'{HERE}/cat100/extra.json', encoding='utf8')) if os.path.exists(f'{HERE}/cat100/extra.json') else {}
    recs = []  # (categorie, rang cercetat, înregistrare)
    for c in CATS:
        seen = set()
        for x in data[c]:
            assert x['name'] not in seen, ('nume dublu în aceeași categorie', c, x['name'])
            seen.add(x['name'])
            x = dict(x, **FIELD_FIX.get((c, x['name']), {}))
            recs.append({'cat': c, 'r0': x['r'], 'name': RENAME_RECORD.get((c, x['name']), x['name']), 'orig_name': x['name'], 'x': x})
    byname = {(r['cat'], r['orig_name']): r for r in recs}
    removed, alias = [], {}
    # 1) MERGE
    for cc, cn, show, als in MERGE:
        canon = byname[(cc, cn)]
        if show:
            canon['name'] = show
        for a in als:
            r = byname[a]
            assert r in recs, a
            recs.remove(r); removed.append((r['cat'], r['orig_name'], 'merge -> ' + canon['name']))
            alias[key(r['orig_name'])] = key(canon['name'])
    # 2) KEEP pentru cheile identice în mai multe categorii
    groups = collections.defaultdict(list)
    for r in recs:
        groups[key(r['name'])].append(r)
    unresolved = []
    for k, g in groups.items():
        if len(g) > 1:
            if k not in KEEP:
                unresolved.append((k, [(r['cat'], r['name']) for r in g])); continue
            keepers = [r for r in g if r['cat'] == KEEP[k]]
            assert len(keepers) == 1, (k, KEEP[k], [(r['cat'], r['name']) for r in g])
            for r in g:
                if r is not keepers[0]:
                    recs.remove(r); removed.append((r['cat'], r['orig_name'], 'keep în ' + KEEP[k]))
    assert not unresolved, unresolved
    for k in KEEP:
        assert any(key(r['name']) == k for r in recs), ('KEEP fără intrare', k)
    # 3) complet cu înlocuitori (extra.json) până la 100 / categorie
    allkeys = {key(r['name']) for r in recs}
    final, need, used_extra = {}, {}, collections.Counter()
    for c in CATS:
        L = sorted([r for r in recs if r['cat'] == c], key=lambda r: r['r0'])
        need[c] = 100 - len(L)
        for e in extra.get(c, []):
            if len(L) >= 100:
                break
            k = key(e['name'])
            if k in allkeys:
                print('înlocuitor sărit (există deja):', c, e['name']); continue
            allkeys.add(k)
            x = {'r': None, 'name': e['name'], 'existing': False, 'origin': e['origin'], 'year': e.get('year'), 'ctx': [], 'own_title_ro': e.get('own_title_ro') or e['name'],
                 'public_rules': e.get('public_rules', False), 'notes_ro': e.get('notes_ro', ''), 'sources': e.get('sources', [])}
            L.append({'cat': c, 'r0': 1000 + len(L), 'name': e['name'], 'orig_name': e['name'], 'x': x, 'extra': True, 'trad': e.get('trad')}); used_extra[c] += 1
        final[c] = L
    print('locuri de completat:', need)
    short = {c: len(L) for c, L in final.items() if len(L) != 100}
    if short:
        print('INCOMPLET (lipsesc înlocuitori în extra.json):', {c: 100 - n for c, n in short.items()})
        if not check:
            sys.exit(1)
    # 4) validări globale
    names = [r['name'] for L in final.values() for r in L]
    keys = [key(n) for n in names]
    assert len(set(names)) == len(names), [n for n, c in collections.Counter(names).items() if c > 1]
    assert len(set(keys)) == len(keys), [n for n, c in collections.Counter(keys).items() if c > 1]
    # toate cheile din catalogul existent trebuie să fie fie păstrate, fie alias
    for k, (n, c) in base.items():
        assert k in keys or k in alias, ('intrare existentă pierdută', n, c)
    # 5) rezultat
    out_rank, new_rows, trad_new, override, rename = {}, [], [], {}, {}
    det = {}
    for c in CATS:
        out_rank[c] = []
        for pos, r in enumerate(final[c], 1):
            x, k = r['x'], key(r['name'])
            ex = k in base
            e = {'r': pos, 'r0': r['r0'] if r['r0'] < 1000 else None, 'name': r['name'], 'origin': norm_origin(x.get('origin')),
                 'year': x.get('year'), 'own': x.get('own_title_ro') or r['name'], 'pub': bool(x.get('public_rules')), 'notes': x.get('notes_ro', ''),
                 'src': x.get('sources', []), 'ex': ex, 'extra': bool(r.get('extra'))}
            out_rank[c].append(e)
            if ex:
                bn, bc = base[k]
                if bc != c:
                    override[k] = c
                if bn != r['name']:
                    rename[k] = r['name']
            else:
                new_rows.append((r['name'], c, e['origin']))
                y = e['year']
                if r.get('trad') is not None:
                    if r['trad']: trad_new.append(k)
                elif e['pub'] and (y is None or y <= OLD_YEAR or e['own'].strip().lower() == r['name'].strip().lower() and c not in TOOLKIT and y <= 1990):
                    trad_new.append(k)
    drop = sorted(set(key(n) for _, n, _ in removed) & set(base) - set(keys))
    assert not (set(override) & set(drop))
    # mutări de categorie pentru intrările existente: raport
    print('mutate (catalog existent):', {base[k][0]: (base[k][1], c) for k, c in override.items()})
    print('redenumiri:', {base[k][0]: v for k, v in rename.items()})
    print('alias (dubluri unificate):', {k: v for k, v in alias.items() if k in base})
    print('eliminate (în afara alias-urilor):', len(removed))
    counts = {c: (sum(1 for e in out_rank[c] if e['ex']), sum(1 for e in out_rank[c] if not e['ex']), used_extra[c]) for c in CATS}
    print('(existente, noi, din care extra):', counts, 'noi total', len(new_rows), 'trad', len(trad_new))
    if check:
        return
    body = {'RANKING': out_rank, 'NEW': new_rows, 'TRAD_NEW': sorted(trad_new), 'ALIAS': alias, 'DROP': drop, 'OVERRIDE': override, 'RENAME': rename,
            'REMOVED': [list(t) for t in removed]}
    txt = ('# @generated by tools/data/build_cat100.py din tools/data/cat100/*.json (+ extra.json). Nu se editează manual.\n'
           '# RANKING[categorie] = 100 intrări în ordinea topului (r = loc final; r0 = loc în cercetare, None pentru înlocuitori;\n'
           '#   ex = există deja în catalog; own = titlu propriu planificat; pub = reguli publice; src = surse NEVERIFICATE, nu se afișează pe site).\n'
           '# NEW = intrări noi în catalog (nume, categorie, origine); TRAD_NEW = chei Kind::Traditional; ALIAS = cheie -> cheie canonică;\n'
           '# DROP = chei eliminate din catalog (dubluri); OVERRIDE = cheie -> categorie nouă; RENAME = cheie -> nume afișat nou.\n')
    for k, v in body.items():
        txt += f'{k} = ' + pprint.pformat(v, width=200, sort_dicts=False) + '\n'
    open(f'{HERE}/data_cat100.py', 'w', encoding='utf8').write(txt)
    print('scris data_cat100.py', len(txt), 'octeți')


if __name__ == '__main__':
    main('--check' in sys.argv)
