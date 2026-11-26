# Whist (2 jucători) (`whist`)

Whist clasic adaptat pentru doi jucători, fără licitație.

## Scop

Iei peste 6 din cele 13 levate; ajungi primul la 5 puncte.

## Pregătire

52 de cărți, 13 fiecăruia, 26 nefolosite și nevăzute. Ultima carte împărțită dealerului se arată: culoarea ei e atu (cartea rămâne în mâna lui). `imparte` pornește mâna.

## Desfășurarea unei ture

Primul conduce non-dealerul; culoare obligatorie, atu bate tot, câștigătorul conduce următoarea levată.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `imparte` — împarte o mână
* `Ah` — joci Asul de cupă
* `9c` — 9 de treflă

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play whist --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `Ac` — joci Asul de treflă (culoarea atu în seed 1).
2. `Kc` — joci Regele de treflă.
3. `9c` — joci 9 de treflă.

## Reguli speciale și variante alese

Fără „onoruri” (simplificare declarată în sursă).

## Punctaj

Cine ia mai mult de 6 levate primește câte un punct pentru fiecare levată peste a șasea.

## Sfârșitul jocului

Jocul se încheie la 5 puncte.

## Note despre AI și informația ascunsă

Informație ascunsă: vezi doar mâna ta. AI-ul (alpha-beta adâncime 1) citește doar propria mână, atuul arătat, levatele luate și numărul cărților adversarului.

## Diferențe față de regulile tradiționale

Fără onoruri; doi jucători în loc de patru, 26 de cărți nefolosite.
