# Prinde (licitație exactă, 2 jucători) (`prinde`)

Levate cu licitație exactă (tip „Oh Hell”), pentru doi jucători, în 13 runde cu mâini de 1…7…1 cărți.

## Scop

Ia exact atâtea levate câte ai licitat; ai mai multe puncte după a 13-a rundă.

## Pregătire

Pachet de 52; mâinile cresc de la 1 la 7 cărți și scad înapoi (13 runde). După împărțire se întoarce o carte din restul pachetului: culoarea ei e atu în runda respectivă. Mâna nouă se pornește cu `imparte`.

## Desfășurarea unei ture

Fiecare spune câte levate va lua (`licit N`); dealerul nu are voie să aducă suma licitațiilor la numărul de levate al rundei. Primul conduce non-dealerul; culoare obligatorie, atu bate tot; cine ia levata conduce.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `imparte` — începe runda
* `licit 2` — licitezi 2 levate
* `Kd` — joci Regele de caro
* `4s` — 4 de pică

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play prinde --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `licit 1` — în runda 1 (o carte) licitezi 1 levată.
2. `Qs` — joci singura carte, Dama de pică.
3. `licit 0` — în runda 2 licitezi 0 levate.

## Reguli speciale și variante alese

Restricția dealerului („cineva trebuie să piardă”): ultimul care licitează nu poate completa suma exactă. Cartea întoarsă este publică.

## Punctaj

Exact cât ai licitat = 10 + licitația; altfel 0.

## Sfârșitul jocului

După a 13-a rundă câștigă cine are mai multe puncte (egal = remiză).

## Note despre AI și informația ascunsă

Informație ascunsă: vezi doar mâna ta. AI-ul (alpha-beta adâncime 1) citește doar propria mână, atuul întors, licitațiile, levatele luate și numărul cărților adversarului.

## Diferențe față de regulile tradiționale

Doi jucători; scor fix 10 + licitația; nume generic.
