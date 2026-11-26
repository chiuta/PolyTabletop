# Gin Rummy (`gin_rummy`)

Gin Rummy standard pentru doi jucători.

## Scop

Primul la 100 de puncte, închizând mâinile cu cât mai puțin deadwood.

## Pregătire

52 de cărți, 10 fiecăruia, următoarea carte întoarsă formează talonul. A=1, figurile=10. Combinații: seturi (3–4 de aceeași valoare) și secvențe (3+ din aceeași culoare, A doar jos). Aranjarea mâinii e automată (deadwood minim).

## Desfășurarea unei ture

Tragi (`stoc` sau `talon`) și arunci (`dis 7h`). Cu cel mult 10 puncte de deadwood poți bate (`bat 7h`), cu 0 dai gin (`gin 7h`). După bătaie adversarul poate lipi cărți (`adauga 5h la 1`) și încheie cu `gata`. Prima tură: `talon` sau `pas`. Mâna se încheie cu `arata`/`urmatoarea`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `talon`
* `dis 9h`
* `bat 7h`
* `adauga 5h la 1`
* `gata`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Punctaj: diferența de deadwood; undercut +25; gin 25 + deadwood-ul adversarului. Mână remiză când rămân două cărți în stoc. Omise: big gin și bonusurile de final de joc. Cel mult 20 de mâini (apoi scorul mai mare).
