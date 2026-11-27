#!/usr/bin/env python3 -I
"""Generează objectul JSON cu jocurile și îl încorporează în docs/index.html (din template.html).
Rulare: python3 -I tools/site/gen_games.py   (din rădăcina depozitului)"""
import re, json, glob, os, sys
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
src = open(f'{root}/polytabletop-games/src/lib.rs', encoding='utf8').read()
src = src[src.index('fn playable_base()'):] + ''.join(open(f'{root}/polytabletop-games/src/{r}.rs', encoding='utf8').read() for r in ('q6_reg', 'b1_reg'))
games = re.findall(r'id: "([^"]+)",\s*title: "([^"]*)",\s*category: "([^"]*)"', src)
LANGS = ['en', 'ro', 'fr', 'it', 'es', 'pt', 'de', 'hu']
tsv = {}
for f in sorted(glob.glob(f'{root}/polytabletop-games/data/i18n/titles_*.tsv')):
    for l in open(f, encoding='utf8'):
        if l.startswith('#') or not l.strip():
            continue
        p = l.rstrip('\n').split('\t')
        if len(p) >= 3 and p[1] in LANGS and p[2]:
            tsv[(p[0], p[1])] = p[2]
en = dict(l.rstrip('\n').split('\t') for l in open(f'{root}/tools/site/en_titles.tsv', encoding='utf8') if l.strip())
CATS = ['Abstract', 'Cărți', 'Strategie', 'Party', 'Zaruri', 'Clasic', 'Familie', 'Miniaturi', 'Cooperativ', 'TCG', 'Deducție', 'Cuvinte', 'Copii', 'Rol (RPG)']
CK = {c: k for c, k in zip(CATS, ['abstract', 'cards', 'strategy', 'party', 'dice', 'classic', 'family', 'minis', 'coop', 'tcg', 'deduction', 'words', 'kids', 'rpg'])}

def hero_svg():
    import math
    o=['<svg class="hero-art" viewBox="0 0 560 400" role="img" aria-labelledby="heroT" xmlns="http://www.w3.org/2000/svg"><title id="heroT">Chess, go, hex, mancala, dice and cards</title>']
    # sah 4x4
    o.append('<g transform="translate(16 16)"><rect x="-6" y="-6" width="188" height="188" rx="10" class="frame"/>')
    for r in range(4):
        for c in range(4):
            o.append(f'<rect x="{c*44}" y="{r*44}" width="44" height="44" class="{"sq1" if (r+c)%2==0 else "sq2"}"/>')
    def pawn(x,y,cl):
        return f'<g class="{cl}"><circle cx="{x}" cy="{y-9}" r="8"/><path d="M{x-6} {y-1}h12l4 14h-20z"/><rect x="{x-12}" y="{y+13}" width="24" height="5" rx="2"/></g>'
    def rook(x,y,cl):
        return f'<g class="{cl}"><path d="M{x-11} {y-14}h5v4h4v-4h4v4h4v-4h5v10l-4 4v8l4 4v4h-26v-4l4-4v-8l-4-4z" transform="translate(0 0)"/></g>'
    o.append(pawn(22,26,'pc-d')); o.append(rook(66,22,'pc-l')); o.append(pawn(110,70,'pc-l')); o.append(rook(154,110,'pc-d')); o.append(pawn(66,114,'pc-d')); o.append(pawn(22,158,'pc-l'))
    o.append('</g>')
    # go 5x5
    o.append('<g transform="translate(216 16)"><rect width="168" height="168" rx="10" class="goboard"/>')
    for i in range(5):
        o.append(f'<path d="M24 {24+i*30}h120M{24+i*30} 24v120" class="goline"/>')
    for (x,y,b) in [(1,1,1),(2,1,0),(2,2,1),(3,2,0),(1,3,0),(3,3,1),(2,3,1),(0,2,0)]:
        o.append(f'<circle cx="{24+x*30}" cy="{24+y*30}" r="12" class="{"st-b" if b else "st-w"}"/>')
    o.append('</g>')
    # hex
    cx,cy,R=478,100,26
    w=math.sqrt(3)*R
    cells=[(0,0,'hx2'),(1,0,'hx1'),(-1,0,'hx3'),(.5,1,'hx1'),(-.5,1,'hx2'),(.5,-1,'hx3'),(-.5,-1,'hx1')]
    for dx,dy,cl in cells:
        x=cx+dx*w; y=cy+dy*R*1.5
        pts=' '.join(f'{x+R*math.cos(math.radians(60*k-30)):.1f},{y+R*math.sin(math.radians(60*k-30)):.1f}' for k in range(6))
        o.append(f'<polygon points="{pts}" class="{cl}"/>')
    # mancala
    o.append('<g transform="translate(16 214)"><rect width="352" height="104" rx="52" class="wood"/>')
    o.append('<ellipse cx="34" cy="52" rx="18" ry="36" class="pit"/><ellipse cx="318" cy="52" rx="18" ry="36" class="pit"/>')
    seeds=[3,1,4,2,0,4,2,3,1,5]
    for r in range(2):
        for i in range(5):
            px=84+i*46; py=30+r*44
            o.append(f'<circle cx="{px}" cy="{py}" r="17" class="pit"/>')
            n=seeds[r*5+i]
            for k in range(n):
                a=k*2.4; rr=0 if n==1 else 7
                o.append(f'<circle cx="{px+rr*math.cos(a):.1f}" cy="{py+rr*math.sin(a):.1f}" r="3.6" class="seed{k%3}"/>')
    for k in range(7):
        a=k*2.1; o.append(f'<circle cx="{34+8*math.cos(a):.1f}" cy="{52+14*math.sin(a):.1f}" r="3.6" class="seed{k%3}"/>')
    o.append('</g>')
    # zaruri
    def die(x,y,rot,pips):
        g=f'<g transform="translate({x} {y}) rotate({rot})"><rect x="-26" y="-26" width="52" height="52" rx="10" class="die"/>'
        pos={1:[(0,0)],2:[(-12,-12),(12,12)],3:[(-12,-12),(0,0),(12,12)],4:[(-12,-12),(12,-12),(-12,12),(12,12)],5:[(-12,-12),(12,-12),(0,0),(-12,12),(12,12)],6:[(-12,-14),(12,-14),(-12,0),(12,0),(-12,14),(12,14)]}
        for px,py in pos[pips]: g+=f'<circle cx="{px}" cy="{py}" r="4.2" class="pip"/>'
        return g+'</g>'
    o.append(die(420,222,-10,5)); o.append(die(486,246,14,3))
    # carti
    for rot,x,t in [(-16,430,'a'),(0,470,'b'),(16,510,'c')]:
        sh={'a':'<path d="M0 -16l12 16-12 16-12-16z" class="ink"/>','b':'<circle r="12" class="ink"/>','c':'<path d="M0 -14l13 24h-26z" class="ink"/>'}[t]
        o.append(f'<g transform="translate({x} 345) rotate({rot} 0 40)"><rect x="-30" y="-44" width="60" height="88" rx="8" class="card"/>{sh}<circle cx="-19" cy="-33" r="3" class="ink"/><circle cx="19" cy="33" r="3" class="ink"/></g>')
    o.append('</svg>')
    return ''.join(o)

out, fromtsv = [], 0
for gid, ro, cat in games:
    t = {}
    for lg in LANGS:
        if (gid, lg) in tsv:
            t[lg] = tsv[(gid, lg)]; fromtsv += 1
        elif lg == 'ro':
            t[lg] = ro
        else:
            t[lg] = en.get(gid) or tsv.get((gid, 'en')) or gid   # fallback: titlul englez până sosesc traducerile din TSV
    # compactare: păstrăm doar ce diferă de en
    d = {'i': gid, 'c': CK[cat], 'en': t['en']}
    for lg in LANGS[1:]:
        if t[lg] != t['en']:
            d[lg] = t[lg]
    out.append(d)
N_GAMES = len(out)
assert N_GAMES >= 240, N_GAMES
js = json.dumps(out, ensure_ascii=False, separators=(',', ':')).replace('</', '<\\/')
tpl = open(f'{root}/tools/site/template.html', encoding='utf8').read()
assert '/*GAMES_JSON*/[]' in tpl
sys.path.insert(0, f'{root}/tools/site')
import ui_i18n
def J(o):
    return json.dumps(o, ensure_ascii=False, separators=(',', ':')).replace('</', '<\\/')
rk = json.load(open(f'{root}/tools/site/data/rankings.json', encoding='utf8'))
ori = json.load(open(f'{root}/tools/site/data/origins.json', encoding='utf8'))
RANK = {lid: [[x['r'], x['n'], x['c'], x['x'], x['s'], x['g']] for x in L] for lid, L in rk['lists'].items()}
for L in RANK.values():
    assert len(L) == 100
# topuri pe categorii: id-ul listei este „c-<categorie>”; fiecare are exact 100 de intrări distincte
CATIDS = ['abstract', 'classic', 'family', 'strategy', 'party', 'coop', 'cards', 'rpg', 'minis', 'tcg', 'kids', 'dice']
assert set(rk['cat_lists']) == set(CATIDS)
for k in CATIDS:
    L = rk['cat_lists'][k]
    assert len(L) == 100 and len({x['n'] for x in L}) == 100
    RANK['c-' + k] = [[x['r'], x['n'], x['c'], x['x'], x['s'], x['g']] for x in L]
ctabs = '\n'.join(f'<button type="button" role="tab" id="rankings-c-{k}" aria-controls="rkp-c-{k}" aria-selected="false" tabindex="{0 if i == 0 else -1}" data-lid="c-{k}" data-i18n="cat_{k}">{k}</button>' for i, k in enumerate(CATIDS))
cpanels = '\n'.join(f'<div id="rkp-c-{k}" role="tabpanel" aria-labelledby="rankings-c-{k}" tabindex="0" hidden></div>' for k in CATIDS)
PREF = ('cat_rpg', 'cat_kids', 'nav_rank', 'rk_', 'th_', 'st_', 'stl_', 'idx_media')
TX = {l: {k: v for k, v in d.items() if k.startswith(PREF)} for l, d in ui_i18n.UI.items()}
rkjs = open(f'{root}/tools/site/rank.js', encoding='utf8').read()
has_media = bool(glob.glob(f'{root}/docs/media/*/*.png') or glob.glob(f'{root}/docs/media/*/*.mp4'))
note = '<p class="mut" data-i18n="idx_media"></p>' if has_media else ''
html = (tpl.replace('<!--RK_CAT_TABS-->', ctabs).replace('<!--RK_CAT_PANELS-->', cpanels).replace('/*GAMES_JSON*/[]', js).replace('<!--HERO_SVG-->', hero_svg())
        .replace('/*RANK_JSON*/{}', J(RANK)).replace('/*ORI_JSON*/{}', J(ori['origins'])).replace('/*OBS_JSON*/{}', J(ori['notes']))
        .replace('/*TX_JSON*/{}', J(TX)).replace('/*RANK_JS*/', rkjs).replace('<!--MEDIA_NOTE-->', note))
html = html.replace('__N__', str(N_GAMES))
open(f'{root}/docs/index.html', 'w', encoding='utf8').write(html)
print('jocuri', len(out), 'titluri din TSV', fromtsv, 'dimensiune', len(html.encode('utf8')), 'media', has_media)
