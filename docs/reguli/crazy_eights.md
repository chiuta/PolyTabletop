# Crazy Eights (Optarii) (`crazy_eights`)

Crazy Eights (optarii) pentru doi jucători.

## Scop

Primul la 100 de puncte, golind mâna.

## Pregătire

52 de cărți, 7 fiecăruia, o carte de start pe masă (dacă e 8 se întoarce alta).

## Desfășurarea unei ture

Pui o carte de aceeași culoare sau valoare; optarul (8) se pune oricând și alege culoarea (`8h:s`). Fără carte potrivită tragi (`trage`) până poți juca; cu stocul gol dai `pas`. Cine rămâne fără cărți ia punctele din mâna adversarului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `7h`
* `8h:s`
* `trage`
* `pas`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Valori: 8 = 50, K Q J 10 = 10, A = 1, restul cât scrie. Blocaj (ambii pas): ia diferența cel cu mâna mai mică. Fără reamestecarea aruncatelor.
