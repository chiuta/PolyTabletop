# Coroana Siderală (4X compact, variantă originală) (`sideral`)

Coroana Siderală: 4X compact pentru doi, variantă originală: flote, planete, tehnologii, cărți de strategie.

## Scop

Fii primul la 8 puncte sau ai cele mai multe puncte după 12 runde (apoi planete; egal = remiză).

## Pregătire

Harta: 9 sisteme — Citadela în centru (legată de toate cele șase sisteme din inel), inelul 1–6 și două capete (Nodul lângă Kryn/Oasa, Abis lângă Tarsis/Ostra). J1 pornește din Aureon, J2 din Veltris (planetele-capitală nu pot fi cucerite); 3 nave, 3 jetoane de comandă.

## Desfășurarea unei ture

Runda: (1) Strategie: cărțile 1–6 (Conducere, Comert, Constructie, Tehnologie, Expeditie, Imperiu) se aleg pe rând (`alege Comert`), câte două fiecare, începând cu vorbitorul (alternativ). (2) Acțiune: în ordinea inițiativei (cartea cu numărul cel mai mic), pe rând, o acțiune: `taktic A>B n` (activezi sistemul B cu un jeton sau o activare gratuită și muți n nave din A; apoi `adauga C>B n`, `gata`), `carte Nume` (efectul principal; adversarul poate folosi efectul secundar: `sec` / `nu`) sau `pas`. (3) Status: `obiectiv N`, apoi se reîmprospătează resursele și influența (+2 jetoane), cărțile se întorc și se dezvăluie un nou obiectiv.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `alege Comert` — alegi cartea de strategie Comert
* `taktic Aureon>Kryn 2` — activezi Kryn și muți 2 nave din Aureon
* `adauga C>B n` — adaugi nave din C în B
* `gata` — termini mutările tactice
* `carte Imperiu` — joci efectul principal al cărții
* `sec / nu` — adversarul folosește / refuză efectul secundar
* `pas` — pasezi
* `obiectiv N` — punctezi obiectivul N
* `nava Sistem / tech Nume` — sub-faze
* `zar` — mutare forțată: lupta

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play sideral --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `alege Imperiu` — alegi cartea Imperiu.
2. `alege Constructie` — alegi cartea Constructie (a doua carte a ta).
3. `carte Imperiu` — joci efectul principal al Imperiului (+1 p, +1 dacă ții Citadela).

## Reguli speciale și variante alese

Lupta: mutări forțate `zar`, runde de d10 (lovește la ≥ 8; Tunuri: ≥ 6), Scuturi anulează o lovitură pe rundă când adversarul lovește de cel puțin două ori, până când o parte rămâne fără nave. Cine rămâne în sistem cu cel puțin r nave (r+1 dacă planeta e a adversarului) o ocupă. Cărți: Conducere (+3 jetoane; sec.: 2 inf → +2 jetoane), Comert (+4 resurse; sec.: 1 jeton → +2), Constructie (până la 3 nave, 2 resurse fiecare, pe planetele tale; sec.: 1 jeton → 1 navă), Tehnologie (1 tehnologie, 3 resurse), Expeditie (2 activări gratuite), Imperiu (+1 p, +1 dacă ții Citadela; sec.: 4 inf → +1 p). Tehnologii: Tunuri, Scuturi, Propulsie (rază 2), Santier (+1 navă la Constructie), Extractie (+2 resurse/rundă).

## Punctaj

Puncte din Imperiu, obiective publice etc.; afișate pentru fiecare jucător.

## Sfârșitul jocului

8 puncte sau 12 runde.

## Note despre AI și informația ascunsă

`evaluate` e determinist și public (nu există informație ascunsă); zarurile sunt mutări forțate separate; în timpul activării, planeta „proiectată” e numărată ca ocupată dacă flota ajunge.

## Diferențe față de regulile tradiționale

Joc original; nu se declară abateri.
