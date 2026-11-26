# Inimi (evitarea punctelor, 2 jucători) (`inimi`)

Joc de levate cu evitarea punctelor (tip „Hearts”), pentru doi jucători, cu 13 cărți fiecare dintr-un pachet de 52.

## Scop

Ai mai puține puncte de penalizare decât adversarul când cineva atinge 40 de puncte.

## Pregătire

Pachet de 52; fiecare primește 13 cărți, ceilalte 26 rămân nefolosite și nevăzute. Împărțirea se cere cu mutarea forțată `imparte`. Cine nu conduce prima levată o deschide.

## Desfășurarea unei ture

Joci o carte pe rând; câștigătorul unei levate conduce următoarea. Se răspunde obligatoriu la culoare; nu există atu, ia levata cea mai mare carte din culoarea condusă. Cine ia levata ia și punctele ei.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `imparte` — împarte o mână nouă
* `7c` — joci 7 de treflă; rang + culoare (`c d h s`)
* `Qs` — Dama de pică (13 puncte penalizare)
* `10h` — 10 de cupă

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play inimi --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `Qs` — joci Dama de pică (atenție: 13 puncte penalizare pentru cine o ia).
2. `2c` — joci 2 de treflă.
3. `8d` — joci 8 de caro.

## Reguli speciale și variante alese

Nu poți conduce cu inimă până nu s-a jucat una („inimi sparte”), decât dacă ai doar inimi. Afișajul arată `inimi nesparte`/`sparte`.

## Punctaj

Fiecare inimă (h) costă 1 punct, Dama de pică (Qs) costă 13. Punctele sunt penalizări.

## Sfârșitul jocului

Când cineva atinge 40 de puncte, câștigă cel cu mai puține (egal = remiză).

## Note despre AI și informația ascunsă

Informație ascunsă: vezi doar mâna ta. AI-ul (alpha-beta adâncime 1) citește doar propria mână, levatele luate (publice) și numărul cărților adversarului. Împărțirea (`imparte`) este mutare forțată.

## Diferențe față de regulile tradiționale

Variantă pentru doi jucători: 26 de cărți nefolosite și nevăzute; nume generic fără mărci.
