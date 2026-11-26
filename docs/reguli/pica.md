# Pică (licitație și atu pică, 2 jucători) (`pica`)

Joc de levate cu licitație și pică atu permanent (tip „Spades”), pentru doi jucători.

## Scop

Ajungi primul la 150 de puncte (sau scorul mai mare după 12 mâini) licitând corect câte levate iei.

## Pregătire

52 de cărți, 13 fiecăruia, 26 nefolosite și nevăzute. Atu permanent: pica (`s`). `imparte` pornește mâna.

## Desfășurarea unei ture

Licitație într-o singură rundă (fără a vedea cartea celuilalt): `licit 1`…`licit 13` sau `licit 0` = nil. Apoi 13 levate: culoare obligatorie, atu bate tot; câștigătorul conduce.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `imparte` — împarte o mână
* `licit 4` — licitezi 4 levate; `licit 0` = nil
* `As` — joci Asul de pică
* `7h` — 7 de cupă

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play pica --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `licit 7` — licitezi 7 levate.
2. `Ac` — joci Asul de treflă.
3. `Kc` — joci Regele de treflă.

## Reguli speciale și variante alese

Nu poți conduce cu pică până nu s-a jucat una („pică spartă”), decât dacă ai doar pică.

## Punctaj

Bid ≥ 1 reușit: 10×bid + 1 pe fiecare levată în plus („bag”); picat: −10×bid. Nil reușit +50, picat −50. La 10 bag-uri adunate: −100.

## Sfârșitul jocului

Până la 150 (sau −150) de puncte sau maximum 12 mâini; atunci câștigă scorul mai mare.

## Note despre AI și informația ascunsă

Informație ascunsă: vezi doar mâna ta. AI-ul (alpha-beta adâncime 1) citește doar propria mână, licitațiile, levatele (publice) și numărul cărților adversarului.

## Diferențe față de regulile tradiționale

Doi jucători (26 de cărți nefolosite și nevăzute); licitație dintr-o singură rundă; nume generic.
