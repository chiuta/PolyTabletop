# Conquián (`conquian`)

Conquián (rummy mexican) cu pachet spaniol, pentru doi jucători.

## Scop

Primul la 3 mâini câștigate.

## Pregătire

40 de cărți: A 2 3 4 5 6 7 S C R în culorile `o c e b`; 10 cărți fiecăruia, una pe talon.

## Desfășurarea unei ture

Tragi (`stoc`) sau iei talonul doar ca să-l legi imediat (`ia 3c 3e` sau `ia la 2`). Apoi cobori combinații (`pune 3o 3c 3e`), lipești cărți pe orice combinație (`adauga 4o la 2`) și închei cu `descarta 5o`. Combinații: set de 3–4 sau secvență de 3+ din aceeași culoare.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `stoc`
* `pune 3o 3c 3e`
* `adauga 4o la 2`
* `descarta 5o`
* `pas`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Câștigă mâna cine rămâne fără cărți în mână. Stoc gol fără talon posibil: mână nulă (`pas`). Nu există pachet de 48 de cărți sau reguli de plată.
