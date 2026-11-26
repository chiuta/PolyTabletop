# Epoci (variantă originală) (`epoci`)

Epoci: draft de cărți în trei epoci cu comerț între vecini, pentru Jucătorul 1, Jucătorul 2 și Orașul Oarb (automat).

## Scop

Ai cel mai mare scor după cele 3 epoci (Orașul Oarb nu intră în clasament).

## Pregătire

Trei orașe într-un inel, deci fiecare are doi vecini. Resurse: L lemn, P piatră, U lut, S sare (materii prime); N pânză, G sticlă, H hârtie (prelucrate). 3 epoci; în fiecare, fiecare oraș primește 7 cărți și joacă 6 ture. Împărțirea epocii următoare este mutare separată (`imparte`).

## Desfășurarea unei ture

Alegi o carte din mână, mâinile se pasează (epocile 1 și 3: J1→J2→Oarb→J1; epoca 2 în sens invers); ultima carte rămasă se aruncă. Ce nu produci, cumperi de la vecini: 2 monede/unitate (1 cu cartea de reducere potrivită); plătești întâi Orașul Oarb, apoi celălalt jucător.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `joc 3` — construiești cartea 3 plătind costul
* `etapa 3` — cartea 3 cu fața în jos construiește următoarea etapă a minunii
* `vinde 3` — vinzi cartea 3 (+3 monede)
* `imparte` — mutare forțată: începe epoca următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play epoci --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `joc 2` — construiești Lutăria.
2. `joc 4` — construiești Cherestea.
3. `joc 1` — construiești Cărbunăria.

## Reguli speciale și variante alese

Orașul Oarb: nu are minune; produce, are armată și vinde resurse; din mâna lui construiește gratuit cea mai prețuită carte posibilă, în ordinea: militară, știință, civică, materii prime, produse prelucrate, comerț, breaslă; altfel vinde prima carte pentru 3 monede. Mutările sunt secvențiale; resursele de cumpărat de la vecini se iau din starea de la începutul turei.

## Punctaj

Puncte civice + etape ale minunii, +1/+3/+5 la fiecare victorie militară (epoca 1/2/3) și −1 la fiecare înfrângere, 1 punct la 3 monede, știință (fiecare simbol: pătratul numărului + 7 pe set de trei), bresle.

## Sfârșitul jocului

După epoca 3.

## Note despre AI și informația ascunsă

AI-ul nu folosește cartea pe care a jucat-o adversarul în aceeași tură; împărțirea epocii următoare e mutare separată (`imparte`), ca AI-ul să nu vadă viitorul.

## Diferențe față de regulile tradiționale

Simplificare declarată: fără lanțuri de construcție; cărțile sunt ale noastre.
