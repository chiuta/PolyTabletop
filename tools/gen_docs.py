#!/usr/bin/env python3
"""Generează docs/JOCURI.md din binarul `polytabletop` construit.

Folosire:  python3 -I tools/gen_docs.py [--bin CALE_POLYTABLETOP] [--out docs/JOCURI.md]
Binarul implicit: $POLYTABLETOP_BIN, apoi target/release/polytabletop.
"""
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def args_get(flag, default):
    a = sys.argv
    return a[a.index(flag) + 1] if flag in a else default


BIN = args_get("--bin", os.environ.get("POLYTABLETOP_BIN", os.path.join(ROOT, "target/release/polytabletop")))
OUT = args_get("--out", os.path.join(ROOT, "docs/JOCURI.md"))
LIB = os.path.join(ROOT, "polytabletop-games/src/lib.rs")


def run(args, stdin=""):
    r = subprocess.run([BIN] + args, input=stdin, capture_output=True, text=True, timeout=120)
    return r.stdout


def parse_list():
    games, cur = [], None
    for line in run(["list"]).splitlines():
        if not line.strip():
            continue
        if not line.startswith(" "):
            id_, _, title = line.partition(" ")
            cur = {"id": id_, "title": title.strip(), "rules": ""}
            games.append(cur)
        elif cur is not None:
            cur["rules"] = line.strip()
    return games


def parse_source():
    src = open(LIB, encoding="utf-8").read()
    cats = dict(re.findall(r'id: "([^"]+)",\s*title: "[^"]*",\s*category: "([^"]+)"', src))
    m = re.search(r"HOTSEAT_SAFE: &\[&str\] = &\[(.*?)\];", src, re.S)
    safe = set(re.findall(r'"([^"]+)"', m.group(1))) if m else set()
    return cats, safe


def fresh(id_):
    out = run(["play", id_, "--seed", "1"], "?\nq\n")
    moves = []
    m = re.search(r"Jucător 1 > Mutări:([^\n]*)", out)
    if m:
        moves = m.group(1).split()
    # randare = tot ce urmează după antet (primul rând gol) până la primul prompt
    body = out.split("\n\n", 1)[1] if "\n\n" in out else out
    return moves, body.split("Jucător 1 >")[0]


def summary(rules):
    text = re.split(r"\s*Mutări?:", rules)[0].strip()
    sents = [s.strip() for s in re.split(r"(?<=[.;!?])\s+(?=[A-ZĂÂÎȘȚ„(])", text) if s.strip()]
    return sents[:6] if sents else [text]


def players(g, hidden_roles):
    r = g["rules"].lower()
    if "roboți" in r or "roboti" in r:
        return "tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți"
    if "partener" in r or "cooperativ" in r:
        return "tu + partener AI contra jocului/adversarilor"
    return "2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)"


LABELS = [
    ("implementat", "Implementat"),
    ("planificat (domeniu public)", "Tradițional"),
    ("planificat (variantă proprie)", "Variantă proprie"),
    ("toolkit generic", "Toolkit"),
]


KIND_LABEL = {"Implemented": "implementat", "Traditional": "planificat (domeniu public)", "Branded": "planificat (variantă proprie)",
              "Toolkit": "toolkit generic (RPG: vezi polytabletop-rpg; TCG/miniaturi: planificat)"}
KIND_GROUP = {"Implemented": "Implementat", "Traditional": "Tradițional", "Branded": "Variantă proprie", "Toolkit": "Toolkit"}


def gen_catalog():
    """Citește direct polytabletop-games/src/catalog.rs (sursa de adevăr; nu depinde de lățimea coloanelor din `polytabletop catalog`)."""
    groups = {k: [] for _, k in LABELS}
    src = open(os.path.join(ROOT, "polytabletop-games/src/catalog.rs"), encoding="utf-8").read()
    for m in re.finditer(r'name:\s*"((?:[^"\\]|\\.)*)",\s*category:\s*"([^"]*)",\s*origin:\s*"([^"]*)",\s*kind:\s*Kind::(\w+),', src):
        name = m.group(1).replace('\\"', '"')
        groups[KIND_GROUP[m.group(4)]].append((name, m.group(2), m.group(3), KIND_LABEL[m.group(4)]))
    total = sum(len(v) for v in groups.values())
    out = ["# Catalogul (generat)\n",
           f"{total} de intrări, grupate după tip, din `polytabletop catalog`. Regenerat de `tools/gen_docs.py`; "
           "sursa e `tools/data` (via `tools/gen_catalog.py`).\n"]
    for _, k in LABELS:
        v = groups[k]
        out.append(f"## {k} ({len(v)})\n")
        out.append("| Nume | Categorie | Origine | Stare |\n|---|---|---|---|")
        for n, c, o, r in v:
            out.append(f"| {n} | {c} | {o} | {r} |")
        out.append("")
    path = os.path.join(os.path.dirname(OUT), "CATALOG.md")
    open(path, "w", encoding="utf-8").write("\n".join(out) + "\n")
    print(f"scris {path}: {total} intrări")


def main():
    gen_catalog()
    games = parse_list()
    cats, safe = parse_source()
    parts = [
        "# Jocurile jucabile\n",
        f"Generat de `tools/gen_docs.py` din `polytabletop list` și `polytabletop play ID` ({len(games)} jocuri). "
        "Nu edita manual; regenerează cu `python3 -I tools/gen_docs.py`.\n",
        "Sintaxa generală: scrii mutarea exact cum apare în lista `?` (fără diferență între majuscule/minuscule, "
        "spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.\n",
        "## Cuprins\n",
    ]
    for g in games:
        parts.append(f"- [{g['title']}](#{g['id']}) (`{g['id']}`)")
    parts.append("")
    for g in games:
        id_ = g["id"]
        cat = cats.get(id_, "?")
        moves, rendered = fresh(id_)
        a = fresh(id_)[1]
        b = rendered
        nondet = a != b
        hidden = id_ in safe or (nondet and cat not in ("Abstract", "Clasic", "Zaruri"))
        if id_ in safe:
            hs = "da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare"
        elif hidden:
            hs = "da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1"
        else:
            hs = "nu (informație completă; zarurile/amestecul nu se consideră ascunse)" if nondet else "nu"
        ex = moves[:2]
        parts.append(f"<a id=\"{id_}\"></a>\n## {g['title']}\n")
        parts.append(f"- **Id:** `{id_}`")
        parts.append(f"- **Categorie:** {cat}")
        parts.append(f"- **Jucători:** {players(g, hidden)}")
        parts.append(f"- **Informație ascunsă:** {hs}")
        parts.append(f"- **Pornire:** `polytabletop play {id_}` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)\n")
        parts.append("**Reguli pe scurt**\n")
        for s in summary(g["rules"]):
            parts.append(f"- {s}")
        parts.append("")
        if ex:
            parts.append("**Mutări** (din `?` pe un joc nou, `legal_moves`)\n")
            parts.append(f"- Sintaxă din regulile jocului: {g['rules'].split('Mutări:')[-1].strip() if 'Mutări:' in g['rules'] else 'vezi lista `?`'}")
            parts.append(f"- Exemple legale la început: `{ex[0]}`" + (f", `{ex[1]}`" if len(ex) > 1 else ""))
            parts.append(f"- La început sunt {len(moves)} cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).\n")
        else:
            parts.append("**Mutări:** `?` nu a returnat mutări la început (începe calculatorul sau jocul cere o alegere preliminară); folosește `--you two` sau `?` după prima mutare.\n")
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    open(OUT, "w", encoding="utf-8").write("\n".join(parts) + "\n")
    print(f"scris {OUT}: {len(games)} jocuri")


main()
