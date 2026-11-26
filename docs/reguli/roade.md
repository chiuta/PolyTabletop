# Roade de Toamnă (legume și schimburi, masa = roboți) (`roade`)

Roade de Toamnă: plantat și schimbat legume, cu trei fermieri (tu + 2 roboți), 8 legume și mână cu ordine fixă.

## Scop

Ai cele mai multe monede la final, recoltând legume.

## Pregătire

Ai 2 ogoare (al treilea costă 3 monede) și o mână cu ordine fixă: mereu plantezi prima carte. Un ogor se poate planta doar gol sau cu aceeași legumă.

## Desfășurarea unei ture

1) Plantezi prima carte (obligatoriu) și, dacă vrei, a doua (`planteaza 1` sau `planteaza 2`); 2) se întorc două cărți din pachet: fiecare o plantezi sau o oferi mesei (`ofera`; un robot care o vrea îți dă o carte necunoscută din mâna lui, amândoi plantați imediat); 3) iei 3 cărți la coada mâinii. Poți recolta oricând (`recolta 2`): numărul de cărți din ogor → monede după tabel.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `planteaza 1` — plantezi o carte (1 sau 2 cărți)
* `recolta 2` — recoltezi ogorul 2
* `compra` — cumperi al treilea ogor (3 monede)
* `treci` — nu mai faci nimic
* `ofera` — oferi cartea mesei
* `accepta ceapa` — accepți oferta unui robot, dând cartea Ceapă
* `refuza` — refuzi oferta

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play roade --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `planteaza 2` — plantezi două cărți (Dovleac și Ceapă).
2. `treci` — treci peste opțiuni.
3. `ofera` — oferi o carte întoarsă mesei.

## Reguli speciale și variante alese

Restul cărților din ogor ies din joc ca „fără valoare”. Roboții îți pot oferi o carte: dacă accepți dai o carte din mână (la alegere) și o plantezi imediat.

## Punctaj

Monedele din recolte; la final se recoltează tot.

## Sfârșitul jocului

Jocul se termină la a doua golire a pachetului; câștigă cine are mai multe monede.

## Note despre AI și informația ascunsă

Schimbul cu masa este orb (cartea primită e necunoscută).

## Diferențe față de regulile tradiționale

Simplificări declarate: fără negociere liberă, fără cărți donate din mână în afara ofertelor.
