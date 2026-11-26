#!/usr/bin/env python3
"""Regenerates polytabletop-games/src/catalog.rs from the research tables in tools/data.

Run from the repository root:  python3 -I tools/gen_catalog.py
Classification (Implemented / Traditional / Branded / Toolkit) is decided here.
"""
import re, sys, pathlib
root = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(root / "tools" / "data"))
import data_world as d, data_rest as r, data_cont as cc
import data_cat100 as c100  # generat de tools/data/build_cat100.py: 12 topuri x 100 pe categorii

rows = [(n, c, o) for L in (d.LUME, d.EUROPA, r.ALTE) for n, c, o in L]
rows += [(n, c, "") for n, c, _ in r.ROMANIA]
rows += list(cc.NEW)  # jocuri tradiționale regionale (topuri pe continente), toate `plan`
rows += list(c100.NEW)  # intrări noi din topurile pe categorii (câte 100 / categorie)

def key(n):
    return re.sub(r"\(.*?\)", "", n).strip().lower()

seen, order = {}, []
for n, c, o in rows:
    k = key(n)
    if k not in seen:
        seen[k] = [n, c, o]; order.append(k)
    elif not seen[k][2] and o:
        seen[k][2] = o

TRAD = set("""șah|dame|table|go|mahjong|domino|ludo / parcheesi|ludo|nu te supăra, frate|poker|remi|bridge|mancala|xiangqi|shogi|carrom|șerpi și scări|gomoku|reversi / othello|moara|țintar|hanafuda|karuta|sugoroku|pai gow|dou dizhu|janggi|yut|hwatu|makruk|sungka|congkak|pallanguzhi|teen patti|rummy|ganjifa|chaupar|dara|fanorona|bagh-chal|surakarta|hasami shogi|morabaraba|ayo|trix|tarneeb|baloot|belote|skat|tarot|scopa|briscola|durak|santase|șase-zeci și șase|canasta|truco|lotería|bingo|cribbage|euchre|blackjack|septica|macao|tablanet|barbut|bătălia navală|battleship|mastermind|memory|connect four|mensch ärgere dich nicht|mensch ärgere dich nicht / ludo""".split("|"))
IMPL = {'blood on the clocktower': 'ceasornic', 'talisman': 'coroana', 'root': 'codru', 'twilight imperium': 'sideral', 'ark nova': 'rezervatia', 'scythe': 'hotar', '7 wonders': 'epoci', 'wingspan': 'solzi', 'everdell': 'catun', 'kingdomino': 'tinuturi', 'cascadia': 'poieni', 'dune: imperium': 'salina', 'terraforming mars': 'reverdire', 'puerto rico': 'mosia', 'agricola': 'gospodaria', 'caylus': 'cetatuia', 'terra mystica': 'taramuri', 'brass: birmingham': 'furnale', 'codenames': 'spionii', 'dixit': 'povestitor', 'cards against humanity': 'gura', 'activity': 'treicai', 'taboo': 'oprite', 'pictionary': 'schita', 'alias': 'dictionar', 'cameleonul': 'impostor', 'just one': 'cuvantunic', 'pokémon tcg': 'tcg_evo', 'yu-gi-oh! tcg': 'tcg_zone', 'flesh and blood': 'tcg_lant', 'disney lorcana': 'tcg_cerneala', 'werewolf / mafia': 'lupii', 'les loups-garous de thiercelieux': 'lupii', 'werewolf': 'lupii', 'the resistance: avalon': 'santinela', 'secret hitler': 'consiliul', 'bang!': 'valea', 'munchkin': 'hruba', 'small world': 'apus', 'dobble': 'fulger', 'bohnanza': 'roade', 'pandemic': 'focare', 'spirit island': 'strajile', 'mysterium': 'farul', 'zombicide': 'cenusii', 'gloomhaven': 'cripta', 'star wars: x-wing': 'nave', 'warhammer: kill team': 'patrula', 'infinity': 'reactii', 'bolt action': 'ordine', 'blood bowl': 'gridiron', 'canasta': 'canasta', 'bridge': 'bridge', 'truco': 'truco', 'cribbage': 'cribbage', 'euchre': 'euchre', 'hearts': 'inimi', 'spades': 'pica', 'oh hell': 'prinde', 'whist': 'whist', 'tablanet': 'tablanet', 'trix': 'trix', 'tarneeb': 'tarneeb', 'baloot': 'baloot', 'belote': 'belote', 'skat': 'skat', 'tarot': 'tarot', 'scopa': 'scopa', 'briscola': 'briscola', 'durak': 'durak', 'risk': 'arhipelag', 'game of life': 'roata', 'yahtzee': 'zaruri5', 'rummikub': 'placute', 'jenga': 'turnul', 'trivial pursuit': 'cunoastere', 'stratego': 'ceata', 'scotland yard': 'umbra', 'labirintul nebun': 'zidurile', 'candy land': 'prajituri', 'guess who?': 'portret', 'cine e?': 'portret', 'congkak': 'congkak', 'pallanguzhi': 'pallanguzhi', 'ayo': 'ayo', 'battleship': 'battleship', 'bătălia navală': 'battleship', 'mastermind': 'mastermind', 'memory': 'memory', 'bingo': 'bingo', 'lotería': 'loteria', 'ludo / parcheesi': 'pachisi', 'ludo': 'ludo', 'mensch ärgere dich nicht': 'ludo', 'mensch ärgere dich nicht / ludo': 'ludo', 'carrom': 'carrom', 'azul': 'faianta', 'dominion': 'bresla', 'splendor': 'cristale', 'love letter': 'curieri', 'exploding kittens': 'fitil', 'sushi go!': 'bufet', 'lost cities': 'expeditii', 'hanabi': 'artificii', 'machi koro': 'orasel', 'skull': 'capcana', 'monopoly': 'mosii', 'scrabble': 'litere', 'uno': 'culori', 'catan': 'insula', 'ticket to ride': 'magistrale', 'carcassonne': 'mozaic', 'cluedo': 'conac', 'mahjong': 'mahjong', 'hanafuda': 'hanafuda', 'hwatu': 'hwatu', 'karuta': 'karuta', 'dou dizhu': 'doudizhu', 'pai gow': 'paigow', 'teen patti': 'teenpatti', 'ganjifa': 'ganjifa', 'yut': 'yut', 'sugoroku': 'sugoroku', 'chaupar': 'chaupar', 'magic: the gathering': 'tcg', 'warhammer 40,000': 'miniaturi', 'warhammer: age of sigmar': 'miniaturi', "șase-zeci și șase":"66","santase":"66","blackjack":"blackjack","remi":"rummy","rummy":"rummy","poker":"poker","dara":"dara","popa prostul":"popa","treizeci și unu":"treizeci1","cassino":"casino","go fish":"pescuit","poker de zaruri":"zarpoker","puncte și căsuțe":"puncte","străpungere":"strapungere","izolare":"izolare","rotire în cinci":"rotire","konane":"konane","coridor":"coridor","drum de pietre":"drum","trei-pierde":"tripierd","fanorona":"fanorona","bagh-chal":"tigri","surakarta":"surakarta","hasami shogi":"hasami","morabaraba":"morabaraba","sungka":"sungka","makruk":"makruk","janggi":"janggi","septica":"septica","macao":"macao","domino":"domino","table":"backgammon","nu te supăra, frate":"ludo","șerpi și scări":"snakes","barbut":"barbut","șah": "chess", "dame": "draughts", "reversi / othello": "reversi", "gomoku": "gomoku", "hex": "hex", "game of the amazons": "amazoane", "lines of action": "linii", "nim": "nim",
        "moara": "morris", "țintar": "morris", "mancala": "mancala", "go": "go", "connect four": "connect4", "xiangqi": "xiangqi", "shogi": "shogi"}
IMPL.update({'coloane': 'coloane', 'bile pe hexagoane': 'bile', 'patru atribute': 'patru', 'cupe încuibate': 'cupe', 'poliedre': 'poliedre'})
TRAD |= {key(n) for n, _, _ in cc.NEW}
TRAD |= set(c100.TRAD_NEW)
# dubluri unificate (păstrăm o singură intrare canonică), mutări de categorie (ex. Yahtzee -> Zaruri) și nume afișate canonice
for k in c100.DROP:
    order.remove(k); del seen[k]
for k, c in c100.OVERRIDE.items():
    seen[k][1] = c
for k, n in c100.RENAME.items():
    seen[k][0] = n
TOOLKIT = {"Rol (RPG)", "Miniaturi", "TCG"}
KIND = {"impl": "Implemented", "trad": "Traditional", "brand": "Branded", "toolkit": "Toolkit"}

def esc(s):
    return s.replace("\\", "\\\\").replace('"', '\\"')

lines = []
for k in order:
    n, c, o = seen[k]
    if k in IMPL: kind, im = "impl", IMPL[k]
    elif c in TOOLKIT: kind, im = "toolkit", ""
    elif k in TRAD: kind, im = "trad", ""
    else: kind, im = "brand", ""
    lines.append(f'    E {{ name: "{esc(n)}", category: "{esc(c)}", origin: "{esc(o or "—")}", kind: Kind::{KIND[kind]}, impl_id: "{im}" }},')

out = ("// @generated by tools/gen_catalog.py from tools/data. Do not edit by hand.\n"
       "use crate::{Kind, E};\n\npub const CATALOG: &[E] = &[\n" + "\n".join(lines) + "\n];\n")
(root / "polytabletop-games" / "src" / "catalog.rs").write_text(out, encoding="utf-8")
print(len(order), "entries")
