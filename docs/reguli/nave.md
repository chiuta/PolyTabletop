# Escadrile din Rift (manevre secrete) (`nave`)

Escadrile din Rift: luptă spațială cu manevre planificate simultan și secret, pe o hartă 12×12 cu asteroizi.

## Scop

Ai mai mult (coca + scut + 3 pe navă) după 10 runde sau elimini toate navele adversarului.

## Pregătire

Fiecare parte are 3 nave (Bivol, Șoim, Vrabie) pe harta 12×12 cu 6 asteroizi simetrici și 4 orientări (N/E/S/V). Zarurile se aruncă la mutarea forțată `arunca`.

## Desfășurarea unei ture

Fiecare navă primește în SECRET o manevră (cadran): `dN` drept (1–4), `blN`/`brN` deviere, `tlN`/`trN` viraj, `u` întoarcere, `s` oprire. Apoi `revela`: manevrele se dezvăluie și se execută în ordinea îndemânării pilotului. Verzi: drept 1–2, deviere 1, viraj 1 (șterg stresul). Roșii: viraj 3, `u`, `s` (dau stres; o navă stresată nu poate planifica roșu). Bivolul nu are `u`, nici viraj 3, nici drept 4. După mutare se trage (`A>X`) sau `A pas`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `A d2` — nava A: drept 2
* `A bl1` — deviere stânga 1
* `A tr2` — viraj dreapta 2
* `A u` — întoarcere (roșu)
* `A s` — oprire (roșu)
* `revela` — dezvăluie manevrele
* `A>X` — focul navei A asupra lui X
* `A pas` — nu trage
* `arunca` — aruncă zarurile

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play nave --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `A br3` — planifici nava A: deviere dreapta 3 (planul rămâne secret până la `revela`).
2. `B d4` — planifici nava B: drept 4.
3. `C d4` — planifici nava C: drept 4; după aceasta rămâne mutarea `revela`.

## Reguli speciale și variante alese

Ciocnire: dacă pătratul final e ocupat, nava se oprește pe ultimul pătrat liber și nu trage; pe asteroid: 1 daună și nu trage. Foc: țintă în arcul din față (lateral ≤ înainte; Bivolul are turelă 360°) la distanță 1–3. Zaruri de atac = atac (+1 la distanță 1; Bivolul trage cu 2): 4–5 lovitură, 6 critic (2 daune); apărare = agilitate (+1 la distanță 3, +1 dacă un asteroid e pe linie): 5–6 evaziune; evaziunile anulează întâi loviturile, apoi criticele. Dauna scade întâi scutul, apoi coca.

## Punctaj

Coca + scut + 3 pe navă, la final de 10 runde.

## Sfârșitul jocului

O parte fără nave pierde; după 10 runde câștigă cine are mai mult; egal = remiză.

## Note despre AI și informația ascunsă

Informație ascunsă: planurile. `render` arată doar planurile Jucătorului 1; `evaluate` citește doar planurile PROPRII și pozițiile publice, deci AI-ul nu trișează.

## Diferențe față de regulile tradiționale

Motor original („scrise de noi”), nu replică un joc existent; nu se declară abateri față de reguli tradiționale.
