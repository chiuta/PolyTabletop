# Cupe încuibate (4×4) (`cupe`)

Cupe încuibate (4×4): tablă 4×4; fiecare jucător are 12 cupe în 3 stive de câte 4 mărimi.

## Scop

Fă patru cupe vizibile pe un rând, o coloană sau o diagonală.

## Pregătire

Stivele sunt încuibate: se poate lua doar cupa de sus (cea mai mare rămasă).

## Desfășurarea unei ture

O tură: fie pui o cupă din rezervă pe o celulă goală sau peste o cupă mai mică (a oricui), fie muți o cupă proprie vizibilă de pe tablă pe o celulă goală sau peste o cupă mai mică. Cupa din rezervă poate acoperi o cupă adversă doar dacă aceasta face parte dintr-un șir de trei piese adverse vizibile pe aceeași linie.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `3@b2` — pui din rezervă o cupă de mărime 3 pe b2
* `b2-c3` — muți cupa vizibilă de pe b2 pe c3 (`+` = are cupe dedesubt)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play cupe --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `4@c3` — pui o cupă de mărime 4 pe c3.
2. `4@d1` — pui o cupă de mărime 4 pe d1.
3. `4@d3` — pui o cupă de mărime 4 pe d3.

## Reguli speciale și variante alese

Dacă ridicarea unei cupe descoperă șirul de patru al adversarului, acesta câștigă (chiar dacă și tu închei un șir).

## Punctaj

Nu există scor.

## Sfârșitul jocului

Câștigă patru cupe vizibile pe o linie; cine nu mai poate muta pierde; remiză după 50 de semi-mutări fără punere din rezervă.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Reguli publice; implementare proprie.
