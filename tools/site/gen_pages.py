#!/usr/bin/env python3 -I
"""Generează paginile individuale docs/jocuri/<id>.html (N) + docs/jocuri/index.html, docs/sitemap.xml, docs/robots.txt și docs/sw.js.
Doar biblioteca standard. Rulare din rădăcina depozitului:  python3 -I tools/site/gen_pages.py [--docs DIR]
Media (docs/media/<id>/...) se decid după existența fișierelor la rulare: regenerați paginile după ce apar capturile."""
import re, json, glob, os, sys, struct, html as H
root = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
docs = f'{root}/docs'
if '--docs' in sys.argv:
    docs = os.path.abspath(sys.argv[sys.argv.index('--docs') + 1])
sys.path.insert(0, f'{root}/tools/site')
import ui_i18n
UI, LANGS = ui_i18n.UI, ui_i18n.LANGS
SITE = 'https://polytabletop.chiuta.ro'
GH = 'https://github.com/chiuta/PolyTabletop/blob/main/docs/reguli/'
esc = H.escape

# ---------- date
src = open(f'{root}/polytabletop-games/src/lib.rs', encoding='utf8').read()
hot = set(re.findall(r'"([^"]+)"', re.search(r'pub const HOTSEAT_SAFE: &\[&str\] = &\[(.*?)\];', src, re.S).group(1)))
src = src[src.index('fn playable_base()'):] + ''.join(open(f'{root}/polytabletop-games/src/{r}.rs', encoding='utf8').read() for r in ('q6_reg', 'b1_reg'))
games = re.findall(r'id: "([^"]+)",\s*title: "([^"]*)",\s*category: "([^"]*)"', src)
assert len(games) >= 240, len(games)
au = open(f'{root}/polytabletop-games/tests/a1_viewer_audit.rs', encoding='utf8').read()
def arr(name):
    return set(re.findall(r'"([^"]+)"', re.search(r'const %s: &\[&str\] = &\[(.*?)\];' % name, au, re.S).group(1)))
god, nohid = arr('SOLO_GOD'), arr('NO_HIDDEN')
CATS = ['Abstract', 'Cărți', 'Strategie', 'Party', 'Zaruri', 'Clasic', 'Familie', 'Miniaturi', 'Cooperativ', 'TCG', 'Deducție', 'Cuvinte', 'Copii', 'Rol (RPG)']
CK = dict(zip(CATS, ['abstract', 'cards', 'strategy', 'party', 'dice', 'classic', 'family', 'minis', 'coop', 'tcg', 'deduction', 'words', 'kids', 'rpg']))
tsv = {}
for f in sorted(glob.glob(f'{root}/polytabletop-games/data/i18n/titles_*.tsv')):
    for l in open(f, encoding='utf8'):
        if l.startswith('#') or not l.strip():
            continue
        p = l.rstrip('\n').split('\t')
        if len(p) >= 4 and p[1] in LANGS:
            tsv[(p[0], p[1])] = (p[2], p[3])
en_t = dict(l.rstrip('\n').split('\t') for l in open(f'{root}/tools/site/en_titles.tsv', encoding='utf8') if l.strip())
rk = json.load(open(f'{root}/tools/site/data/rankings.json', encoding='utf8'))['lists']
LNAME = {lid: f'rk_t_{lid}' for lid in rk}
pos = {}
for lid, L in rk.items():
    for x in L:
        if x['g']:
            pos.setdefault(x['g'], []).append((lid, x['r'], x['n'], x['s']))
# topurile pe categorii: id „c-<categorie>”
for ck_, L in json.load(open(f'{root}/tools/site/data/rankings.json', encoding='utf8'))['cat_lists'].items():
    for x in L:
        if x['g']:
            pos.setdefault(x['g'], []).append(('c-' + ck_, x['r'], x['n'], x['s']))
G = {}
for gid, ro, cat in games:
    titles, rules = {}, {}
    for lg in LANGS:
        t, r = tsv.get((gid, lg), (None, None))
        if lg == 'ro': t = t or ro
        if lg == 'en': t = t or en_t.get(gid) or gid
        titles[lg] = t or en_t.get(gid) or gid
        rules[lg] = r or ''
    G[gid] = {'id': gid, 'cat': CK[cat], 'titles': titles, 'rules': rules}
def compact(d):
    o = {'en': d['en']}
    for lg in LANGS[1:]:
        if d[lg] and d[lg] != d['en']: o[lg] = d[lg]
    return o

def ai_info(gid):
    f = f'{root}/docs/reguli/{gid}.md'
    if not os.path.exists(f): return None
    m = re.search(r'^Stil AI în cod: (.*)$', open(f, encoding='utf8').read(), re.M)
    if not m: return None
    s = m.group(1)
    a = re.search(r'adâncire iterativă și buget de timp \((\d+) ms, adâncime maximă (\d+)\)', s)
    if a: return ('pg_ai_abt', {'ms': int(a.group(1)), 'd': int(a.group(2))})
    a = re.search(r'alpha-beta cu adâncime fixă (\d+)', s)
    if a: return ('pg_ai_ab', {'d': int(a.group(1))})
    if 'MCTS' in s: return ('pg_ai_mcts', {})
    return None

def load_json(p):
    try: return json.load(open(p, encoding='utf8'))
    except Exception: return {}
man_cli = load_json(f'{docs}/media/manifest.json')
man_gui = load_json(f'{docs}/media/gui_manifest.json')

def png_size(p):
    try:
        with open(p, 'rb') as f:
            h = f.read(24)
        if h[:8] == b'\x89PNG\r\n\x1a\n': return struct.unpack('>II', h[16:24])
    except Exception: pass
    return None

STYLE = re.search(r'<style>\n(.*?)</style>', open(f'{root}/tools/site/template.html', encoding='utf8').read(), re.S).group(1)
_DROP = ('.hero', '.sq1', '.pc-l', '.goboard', '.st-b', '.hx1', '.wood', '.die{', '.card{', '.hero-art', '@keyframes float', '@media (prefers-reduced-motion:no-preference){.hero-art', '.stats', '.tag{', '.lead{', '.cta{', '.bd', '.rk-', '.leg', '.cards{', '.card-b', '#glist', '.btn.', '.btn{', '.btn:', '@media(min-width:880px){.hero', '.two', '.demo', '.filters', '.chip', '.count')
STYLE = '\n'.join(l for l in STYLE.split('\n') if not l.startswith(_DROP) or l.startswith('.filters') or l.startswith('.chip') or l.startswith('.count'))
for _v in ('sq1', 'sq2', 'frame', 'pcl', 'pcd', 'pcs', 'go', 'goline', 'stb', 'stw', 'hx1', 'hx2', 'hx3', 'wood', 'pit', 's0', 's1', 's2', 'die', 'pip', 'card', 'ink', 'ink2', 'bd1b', 'bd1t', 'bd2b', 'bd2t', 'bd3b', 'bd3t', 'bd4b', 'bd4t', 'bd5b', 'bd5t'):
    STYLE = re.sub(r'(?<=[;{\n])--%s:[^;}\n]*;?' % _v, '', STYLE)
STYLE = re.sub(r';\}', '}', STYLE)
STYLE = '\n'.join(l.strip() for l in STYLE.split('\n') if l.strip())
STYLE += """
.bc a{min-height:0;padding:0;display:inline;line-height:inherit}.bc{padding:1rem 0 .2rem;font-size:.95rem}.bc ol{list-style:none;display:flex;flex-wrap:wrap;gap:.2rem .5rem;margin:0;padding:0}
.bc li+li::before{content:"›";margin-right:.5rem;color:var(--mut)}
article{padding-bottom:2rem}article section{padding:1.6rem 0}article section:first-of-type{border-top:0}
.pg-h1{font-size:clamp(2rem,6vw,3rem);letter-spacing:-.02em;margin:.2rem 0 .6rem}
.rules{font-size:1.05rem;max-width:75ch}
.opts{display:grid;gap:.4rem;margin:.6rem 0}.opts dt{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-weight:700}.opts dd{margin:0 0 .5rem;color:var(--mut)}
@media(min-width:700px){.opts{grid-template-columns:max-content 1fr;gap:.3rem 1.2rem}.opts dd{margin:0}}
.gal{display:grid;gap:.8rem;grid-template-columns:repeat(auto-fit,minmax(min(100%,17rem),1fr));margin:.8rem 0;padding:0;list-style:none}
.gal img{width:100%;height:auto;display:block;border-radius:10px;border:1px solid var(--line);background:#111}
video{width:100%;max-width:960px;height:auto;display:block;border-radius:10px;background:#111;margin:.8rem 0}
.pn{display:flex;flex-wrap:wrap;gap:.8rem;justify-content:space-between;border-top:1px solid var(--line);padding-top:1.2rem}
.pn a{display:inline-flex;flex-direction:column;min-height:44px;padding:.5rem .9rem;border:1px solid var(--line);background:var(--surf);border-radius:10px;text-decoration:none}
.pn a:hover{border-color:var(--acc)}.pn small{color:var(--mut)}
.pg-tbl table{min-width:24rem}
.lst{list-style:none;margin:0;padding:0;display:grid;gap:.6rem;grid-template-columns:repeat(auto-fill,minmax(16rem,1fr))}
.lst li{background:var(--surf);border:1px solid var(--line);border-radius:10px;padding:.65rem .8rem;display:flex;flex-direction:column;gap:.15rem}
.lst .n{font-weight:650}.lst .k{font-size:.82rem;color:var(--mut)}.lst code{font-size:.82rem;word-break:break-all}
.chk{display:flex;gap:.5rem;align-items:center;margin:.4rem 0;min-height:44px}.chk input{width:1.2rem;height:1.2rem}
"""
STYLE += '''
html.lb-open{overflow:hidden}
.lb-t{cursor:zoom-in;display:block}
dialog.lb{border:0;padding:0;margin:0;width:100vw;height:100vh;height:100dvh;max-width:none;max-height:none;background:rgba(5,8,12,.97);color:#fff;overflow:hidden}
dialog.lb::backdrop{background:#000}
.lb-st{display:flex;flex-direction:column;height:100%;background:#05080c}
.lb-top{display:flex;gap:.5rem;align-items:center;padding:.5rem .8rem;border-bottom:1px solid #2c3745}
.lb-info{flex:1;min-width:0}.lb-cnt{font-weight:700;font-variant-numeric:tabular-nums}.lb-cap{color:#c9d3df;font-size:.92rem;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.lb-b{display:inline-flex;align-items:center;justify-content:center;min-width:48px;min-height:48px;padding:0 .6rem;background:#1d2733;color:#fff;border:1px solid #7b8b9e;border-radius:10px;font-size:1.5rem;line-height:1;cursor:pointer;text-decoration:none}
.lb-b:hover{background:#2b3a4b}
.lb-b[hidden]{display:none}
.lb :focus-visible{outline:3px solid #fff;outline-offset:2px}
.lb-main{position:relative;flex:1;min-height:0;display:flex;align-items:center;justify-content:center;padding:.5rem}
.lb-main img{max-width:100%;max-height:100%;width:auto;height:auto;object-fit:contain;display:block;margin:auto}
.lb-nav{position:absolute;top:50%;transform:translateY(-50%);z-index:1;background:rgba(29,39,51,.88);min-width:52px;min-height:64px;font-size:2rem}
.lb-p{left:.5rem}.lb-n{right:.5rem}
@media(forced-colors:active){.lb-b{border-color:ButtonText}}
'''
STYLE = re.sub(r'/\*.*?\*/', '', STYLE, flags=re.S)   # minificare CSS (paginile de joc trebuie să rămână sub 60 KB)
STYLE = re.sub(r'\s*([{};,>])\s*', r'\1', STYLE)
STYLE = re.sub(r';\}', '}', STYLE).replace('\n', ' ').strip()
shell = open(f'{root}/tools/site/page_template.html', encoding='utf8').read()

def J(o):
    return json.dumps(o, ensure_ascii=False, separators=(',', ':')).replace('</', '<\\/')
def hreflang(path):
    o = [f'<link rel="alternate" hreflang="x-default" href="{SITE}/{path}">']
    o += [f'<link rel="alternate" hreflang="{l}" href="{SITE}/{path}?lang={l}">' for l in LANGS]
    return '\n'.join(o)
def subset(html_text, extra=()):
    keys = set(re.findall(r'data-(?:i|ia|ialt)="(\w+)"', html_text)) | set(extra) | {'pg_copy', 'pg_copied'}
    for m in re.finditer(r'data-a=\'([^\']*)\'', html_text):
        for v in json.loads(H.unescape(m.group(1))).values():
            if isinstance(v, str) and v.startswith('cat_'): keys.add(v)
    return {k: [UI[l][k] for l in LANGS] for k in sorted(keys)}
def render(main, page_js, title, desc, path, d, extra_keys=(), ld=''):
    T = subset(main + shell, extra_keys)
    out = shell
    page_js = '\n'.join(l.strip() for l in page_js.split('\n') if l.strip() and not l.strip().startswith('//'))
    page_js = re.sub(r'([;{,])\n', r'\1', page_js)
    for k, v in (('@@TITLE@@', esc(title)), ('@@DESC@@', esc(desc)), ('@@URL@@', f'{SITE}/{path}'), ('@@HREFLANG@@', hreflang(path)),
                 ('@@LD@@', ld), ('@@STYLE@@', STYLE), ('@@MAIN@@', main), ('@@T@@', J(T)), ('@@D@@', J(d)), ('@@PAGE_JS@@', page_js)):
        out = out.replace(k, v)
    # comprimare: fără linii noi între etichete în afara <script>/<style>/<pre>
    parts = re.split(r'(<script.*?</script>|<style.*?</style>|<pre.*?</pre>)', out, flags=re.S)
    out = ''.join(x if x.startswith(('<script', '<style', '<pre')) else re.sub(r'>\n+<', '><', x) for x in parts)
    return out

LB_JS = open(f'{root}/tools/site/lightbox.js', encoding='utf8').read().replace('/*TEST_HOOK*/', os.environ.get('PT_LB_TEST', ''))
PAGE_JS = r'''
window.pageApply=function(l,t,u){
  const title=u.gt(D.id),cat=t["cat_"+D.cat];
  const $=u.$;
  document.title=u.fmt(t.pg_title,{title:title});
  const desc=u.fmt(t.pg_desc,{title:title,cat:cat});
  [["meta[name=description]"],["meta[property='og:description']"]].forEach(s=>{const e=$(s[0]);if(e)e.content=desc});
  const og=$("meta[property='og:title']");if(og)og.content=document.title;
  const ld=$("#ld");if(ld){const o=JSON.parse(ld.textContent);o.name=title;o.description=desc;o.inLanguage=l;ld.textContent=JSON.stringify(o)}
};
'''

LIST_JS = r'''
let cat="all",onlyR=false;
window.pageApply=function(l,t,u){
  document.title=t.ls_title;
  const desc=u.fmt(t.ls_p,{t:D.L.length});
  const md=u.$("meta[name=description]");if(md)md.content=desc;
  const og=u.$("meta[property='og:title']");if(og)og.content=t.ls_title;
  const od=u.$("meta[property='og:description']");if(od)od.content=desc;
  u.$("#ls-p").textContent=desc;
  const chips=u.$("#chips");chips.textContent="";
  [["all",t.pg_all_cats]].concat(D.CK.map(k=>[k,t["cat_"+k]])).forEach(([k,lab])=>{
    const b=document.createElement("button");b.type="button";b.className="chip";b.textContent=lab;b.setAttribute("aria-pressed",String(k===cat));
    b.addEventListener("click",()=>{cat=k;draw()});chips.appendChild(b);
  });
  draw();
  function draw(){
    [...chips.children].forEach((b,i)=>b.setAttribute("aria-pressed",String((i?D.CK[i-1]:"all")===cat)));
    const q=u.norm(u.$("#q").value.trim()),ul=u.$("#glist");ul.textContent="";let n=0;
    for(const g of D.L){
      if(cat!=="all"&&g[1]!==cat)continue;
      if(onlyR&&!g[2])continue;
      const names=u.gtAll(g[0]);
      if(q&&!u.norm(g[0]+" "+names).includes(q))continue;
      n++;
      const li=document.createElement("li"),a=document.createElement("a");a.className="n";a.href=g[0]+".html?lang="+l;a.textContent=u.gt(g[0]);
      const k=document.createElement("span");k.className="k";k.textContent=t["cat_"+g[1]];
      const c=document.createElement("code");c.textContent="polytabletop play "+g[0];
      li.append(a,k,c);ul.appendChild(li);
    }
    if(!n){const li=document.createElement("li");li.textContent=t.pg_none;ul.appendChild(li)}
    u.$("#count").textContent=u.fmt(t.pg_count,{n:n,t:D.L.length});
  }
  window.__draw=draw;
};
'''

def media_html(gid, t_title):
    """Returnează (html, chei) pentru secțiunea media sau ''. Decide după existența fișierelor."""
    md = f'{docs}/media/{gid}'
    ex = lambda n: os.path.isfile(f'{md}/{n}')
    def block(prefix, clip, poster, head_key, cap_keys, shot_key, grp):
        shots = [f'{prefix}{i}.png' for i in (1, 2, 3) if ex(f'{prefix}{i}.png')]
        has_clip = ex(clip)
        if not shots and not has_clip: return ''
        o = [f'<h3 data-i="{head_key}"></h3>']
        if has_clip:
            pst = f' poster="../media/{gid}/{poster}"' if ex(poster) else ''
            o.append(f'<video controls preload="none" playsinline{pst}><source src="../media/{gid}/{clip}" type="video/mp4"><span data-i="pg_novideo"></span></video>')
        if shots:
            o.append('<ul class="gal">')
            for i, s in enumerate(shots, 1):
                sz = png_size(f'{md}/{s}')
                wh = f' width="{sz[0]}" height="{sz[1]}"' if sz else ''
                a = json.dumps({'n': i, 't': len(shots), 'title': '@' + gid}, ensure_ascii=False)
                o.append(f'<li><a class="lb-t" data-g="{grp}" data-s="{i}" href="../media/{gid}/{s}" target="_blank" rel="noopener"><img src="../media/{gid}/{s}"{wh} loading="lazy" decoding="async" alt="" data-ialt="{shot_key}" data-a=\'{esc(a, quote=False)}\'></a></li>')
            o.append('</ul>')
        for ck in cap_keys:
            o.append(f'<p class="mut" data-i="{ck}"></p>')
        return '\n'.join(o)
    g = man_gui.get(gid, {}) if isinstance(man_gui, dict) else {}
    c = man_cli.get(gid, {}) if isinstance(man_cli, dict) else {}
    gcaps = ['pg_gui_cap']
    if g.get('graphical_board') is False: gcaps.append('pg_gui_text')
    if g.get('hidden_info_shown'): gcaps.append('pg_gui_hidden')
    if g.get('complete') is False: gcaps.append('pg_media_part')
    ccaps = ['pg_media_cap']
    if c.get('shows_hidden'): ccaps.append('pg_media_hidden')
    if c.get('complete') is False: ccaps.append('pg_media_part')
    gb = block('gui_shot', 'gui_clip.mp4', 'gui_poster.png', 'pg_gui_h', gcaps, 'pg_gui_shot_alt', 'gui')
    cb = block('shot', 'clip.mp4', 'poster.png', 'pg_cli_h', ccaps, 'pg_shot_alt', 'cli')
    if not gb and not cb: return ''
    return '<section id="media" aria-labelledby="media-h">\n<h2 id="media-h" data-i="pg_media_h">Screenshots and clip</h2>\n' + gb + ('\n' if gb and cb else '') + cb + '\n</section>'

by_cat = {}
for gid, _, cat in games:
    by_cat.setdefault(CK[cat], []).append(gid)
all_gt = {gid: compact(G[gid]['titles']) for gid in G}
os.makedirs(f'{docs}/jocuri', exist_ok=True)
sizes, nmedia = [], 0
for gid, _, cat in games:
    g = G[gid]; ck = g['cat']; T_en = g['titles']['en']
    cmd = f'polytabletop play {gid}'
    ex = f'polytabletop --lang ro play {gid} --you both --level hard --seed 1'
    # AI
    ai = ai_info(gid)
    ai_li = ''
    if ai:
        ai_li = f'<li data-i="{ai[0]}" data-a=\'{esc(json.dumps(ai[1]), quote=False)}\'></li>\n'
    state = 'pg_hot_safe' if gid in hot else 'pg_hot_god' if gid in god else 'pg_hot_open' if gid in nohid else 'pg_hot_unk'
    # clasamente
    P = pos.get(gid, [])
    if P:
        rows = ''.join(f'<tr><td><a href="../index.html#rankings-{lid}" data-lk' + (f' data-i="{LNAME[lid]}"></a>' if lid in LNAME else f'><span data-i="cat_l"></span>: <span data-i="cat_{lid[2:]}"></span></a>') + f'</td><td class=r>{r}</td><td>{esc(n)}</td></tr>' for lid, r, n, s in P)
        kinds = {s for *_, s in P}
        kind = ('<p class="mut" data-i="pg_kind_play"></p>' if 'play' in kinds else '') + ('<p class="mut" data-i="pg_kind_var"></p>' if 'var' in kinds else '')
        rank = f'<p data-i="pg_rank_p"></p>\n<div class="tbl-wrap pg-tbl" role="region" tabindex="0" data-ia="pg_rank_cap"><table><caption data-i="pg_rank_cap"></caption><thead><tr><th scope="col" data-i="pg_rank_list"></th><th scope="col" data-i="pg_rank_place"></th><th scope="col" data-i="pg_rank_entry"></th></tr></thead><tbody>{rows}</tbody></table></div>\n{kind}'
    else:
        rank = '<p data-i="pg_rank_none"></p>'
    # prev/next
    sib = by_cat[ck]; i = sib.index(gid)
    pn = ''
    if len(sib) > 1:
        pv, nx = sib[i - 1], sib[(i + 1) % len(sib)]
        pn = (f'<nav class="pn" aria-label="Other games in this category" data-ia="pg_nav">'
              f'<a rel="prev" href="{pv}.html" data-lk><small data-i="pg_prev"></small><span data-gt="{pv}">{esc(G[pv]["titles"]["en"])}</span></a>'
              f'<a rel="next" href="{nx}.html" data-lk><small data-i="pg_next"></small><span data-gt="{nx}">{esc(G[nx]["titles"]["en"])}</span></a></nav>')
    media = media_html(gid, T_en)
    if media: nmedia += 1
    main = f'''<nav class="bc" aria-label="Breadcrumb" data-ia="pg_bc"><ol>
<li><a href="../index.html" data-lk data-i="pg_home">Home</a></li>
<li><a href="index.html" data-lk data-i="pg_games">Games</a></li>
<li aria-current="page" data-gt="{gid}">{esc(T_en)}</li>
</ol></nav>
<article>
<header>
<p class="eyebrow"><span data-i="pg_cat">Category</span>: <span data-i="cat_{ck}"></span></p>
<h1 class="pg-h1" data-gt="{gid}">{esc(T_en)}</h1>
</header>
{media}
<section id="rules" aria-labelledby="rules-h">
<h2 id="rules-h" data-i="pg_rules_h">Rules in short</h2>
<p class="rules" data-ru>{esc(g['rules']['en'])}</p>
<p><a class="btn" href="../play/?g={gid}" data-i="pg_play_web">▶ Play in your browser</a></p>
<p><a href="{GH}{gid}.md" rel="noopener" data-i="pg_rules_full">Full rules page (in Romanian, on GitHub)</a></p>
<p class="mut" data-i="pg_rules_note"></p>
</section>
<section id="play" aria-labelledby="play-h">
<h2 id="play-h" data-i="pg_play_h">Play it</h2>
<p data-i="pg_play_p"></p>
<div class="codebox"><pre><code>{cmd}</code></pre><button type="button" class="cp" data-i="pg_copy">Copy</button></div>
<h3 data-i="pg_opts">Options</h3>
<dl class="opts">
<dt>--you one|two|both|none</dt><dd data-i="pg_o_you"></dd>
<dt>--level easy|normal|hard|expert</dt><dd data-i="pg_o_level"></dd>
<dt>--lang en|ro|fr|it|es|pt|de|hu</dt><dd data-i="pg_o_lang"></dd>
<dt>--seed N</dt><dd data-i="pg_o_seed"></dd>
</dl>
<div class="codebox"><pre><code>{ex}</code></pre><button type="button" class="cp" data-i="pg_copy">Copy</button></div>
</section>
<section id="table" aria-labelledby="table-h">
<h2 id="table-h" data-i="pg_table_h">At the table</h2>
<p data-i="{state}"></p>
<h3 data-i="pg_ai_h">Computer opponent</h3>
<ul>
{ai_li}<li data-i="pg_ai_nocheat"></li>
</ul>
</section>
<section id="rank" aria-labelledby="rank-h">
<h2 id="rank-h" data-i="pg_rank_h">Place in the rankings</h2>
{rank}
</section>
</article>
{pn}
<p style="margin:1.5rem 0 2rem"><a href="../index.html#games" data-lk data-i="pg_back">Back to the games on the home page</a> · <a href="index.html" data-lk data-i="pg_all">All game pages</a></p>'''
    ld = {'@context': 'https://schema.org', '@type': 'VideoGame', 'name': T_en, 'url': f'{SITE}/jocuri/{gid}.html', 'genre': ck,
          'applicationCategory': 'Game', 'operatingSystem': 'Linux, macOS, Windows', 'inLanguage': 'en', 'isAccessibleForFree': True,
          'description': UI['en']['pg_desc'].format(title=T_en, cat=UI['en']['cat_' + ck]),
          'author': {'@type': 'Person', 'name': 'Alexio'}, 'isPartOf': {'@type': 'WebSite', 'name': 'PolyTabletop', 'url': SITE + '/'}}
    d = {'id': gid, 'cat': ck, 'GT': {k: all_gt[k] for k in {gid} | ({pv, nx} if len(sib) > 1 else set())}, 'RU': {k: v for k, v in g['rules'].items() if v and (k == 'en' or v != g['rules']['en'])}}
    has_lb = 'class="lb-t"' in main
    pjs = PAGE_JS + (LB_JS if has_lb else '')
    lbk = ('pg_gui_h', 'pg_cli_h', 'lb_dialog', 'lb_close', 'lb_prev', 'lb_next', 'lb_full', 'lb_orig', 'lb_s1', 'lb_s2', 'lb_s3') if has_lb else ()
    html = render(main, pjs, UI['en']['pg_title'].format(title=T_en), ld['description'], f'jocuri/{gid}.html', d,
                  extra_keys=('pg_title', 'pg_desc', 'cat_' + ck) + lbk, ld=f'<script type="application/ld+json" id="ld">{J(ld)}</script>')
    open(f'{docs}/jocuri/{gid}.html', 'w', encoding='utf8').write(html)
    sizes.append(len(html.encode('utf8')))

# ---------- pagina listă
inrank = {g for g in pos}
lis = ''.join(f'<li><a class="n" href="{gid}.html" data-lk>{esc(G[gid]["titles"]["en"])}</a><span class="k" data-i="cat_{CK[c]}"></span><code>polytabletop play {gid}</code></li>' for gid, _, c in games)
main = f'''<nav class="bc" aria-label="Breadcrumb" data-ia="pg_bc"><ol>
<li><a href="../index.html" data-lk data-i="pg_home">Home</a></li>
<li aria-current="page" data-i="pg_games">Games</li>
</ol></nav>
<section class="pg-top" style="border-top:0">
<h1 class="pg-h1" data-i="ls_h">All game pages</h1>
<p id="ls-p">{UI['en']['ls_p'].format(t=len(games))}</p>
<div class="filters"><label for="q" data-i="pg_search">Search games</label><input type="search" id="q" autocomplete="off"></div>
<div class="chips" id="chips" role="group" aria-label="Category" data-ia="pg_cats_nav"></div>
<p class="count" id="count" role="status" aria-live="polite"></p>
<ul class="lst" id="glist">{lis}</ul>
</section>
<p style="margin:1rem 0 2rem"><a href="../index.html#rankings-world" data-lk data-i="nav_rank">Rankings</a></p>'''
dl = {'L': [[gid, CK[c], 1 if gid in inrank else 0] for gid, _, c in games], 'CK': ['classic', 'abstract', 'family', 'strategy', 'party', 'coop', 'cards', 'minis', 'tcg', 'dice', 'deduction', 'words', 'kids', 'rpg'], 'GT': all_gt}
used = set(CK.values())
LJS = LIST_JS + '''
document.querySelector("#q").addEventListener("input",()=>{if(window.__draw)window.__draw()});
'''
# gtAll: titluri în toate limbile pentru căutare
LJS = LJS.replace('window.pageApply=function(l,t,u){', 'window.pageApply=function(l,t,u){\n  u.gtAll=id=>{const g=D.GT[id];return LANGS.map(x=>g[x]||g.en).join(" ")};', 1)
html = render(main, LJS, UI['en']['ls_title'], UI['en']['ls_p'].format(t=len(games)), 'jocuri/index.html', dl, extra_keys=[f'cat_{k}' for k in dl['CK']] + ['ls_title', 'ls_p', 'pg_all_cats', 'pg_none', 'pg_count'])
open(f'{docs}/jocuri/index.html', 'w', encoding='utf8').write(html)

# ---------- sitemap
lm = '2026-11-26'
def url(path):
    loc = f'{SITE}/{path}'
    alt = '\n'.join(f'    <xhtml:link rel="alternate" hreflang="{l}" href="{loc}?lang={l}"/>' for l in LANGS) + f'\n    <xhtml:link rel="alternate" hreflang="x-default" href="{loc}"/>'
    return f'  <url>\n    <loc>{loc}</loc>\n    <lastmod>{lm}</lastmod>\n{alt}\n  </url>'
paths = ['', 'jocuri/index.html'] + [f'jocuri/{gid}.html' for gid, _, _ in games]
open(f'{docs}/sitemap.xml', 'w', encoding='utf8').write('<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">\n' + '\n'.join(url(p) for p in paths) + '\n</urlset>\n')
open(f'{docs}/robots.txt', 'w', encoding='utf8').write('User-agent: *\nAllow: /\n\n# Paginile jocurilor: /jocuri/<id>.html; capturile și clipurile sunt în /media/<id>/ (nu sunt puse în cache de service worker).\nSitemap: https://polytabletop.chiuta.ro/sitemap.xml\n')
sw = open(f'{docs}/sw.js', encoding='utf8').read() if os.path.exists(f'{docs}/sw.js') else ''
if sw:
    sw = re.sub(r'const CACHE = "polytabletop-\d+";', 'const CACHE = "polytabletop-2";', sw)
    sw = sw.replace('const FILES = ["./", "index.html",', 'const FILES = ["./", "index.html", "jocuri/index.html",', 1) if '"jocuri/index.html"' not in sw else sw
    if '/media/' not in sw:
        sw = sw.replace('const key = new URL(r.url).pathname;', 'const key = new URL(r.url).pathname;\n  // capturile și clipurile (docs/media) nu se pun în cache: browserul le cere direct\n  if (key.includes("/media/")) return;', 1)
        sw = sw.replace('(await caches.match("index.html")) ||', '(r.mode === "navigate" && key.includes("/jocuri/") ? await caches.match("jocuri/index.html") : null) || (await caches.match("index.html")) ||', 1)
    open(f'{docs}/sw.js', 'w', encoding='utf8').write(sw)
print('pagini', len(sizes), 'max', max(sizes), 'medie', sum(sizes) // len(sizes), 'cu media', nmedia, 'lista', len(html.encode('utf8')))
