# Chicken Foot (domino) (`chicken_foot`)

Chicken Foot, domino dublu-12, pentru doi jucători.

## Scop

Cel mai mic total de puncte după 13 runde.

## Pregătire

Set dublu-12 (91 de piese), 15 piese fiecăruia; runda r pornește cu dublul 12−r în mijloc.

## Desfășurarea unei ture

Așezi o piesă la un capăt liber (`5-7@2`); fără piesă tragi (`trage`) până poți, apoi `pas`. Un dublu (și motorul) trebuie satisfăcut: următoarele trei piese merg pe laturile lui libere.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `5-7@2`
* `trage`
* `pas`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Runda se încheie când cineva rămâne fără piese sau la blocaj; fiecare adaugă punctele pieselor rămase (câștigătorul rundei 0). Egal = remiză.
