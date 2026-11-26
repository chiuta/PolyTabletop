# Pitch (Setback) (`pitch`)

Pitch (Setback) pentru doi jucători, varianta de bază fără jokeri.

## Scop

Primul la 11 puncte.

## Pregătire

52 de cărți, 6 fiecăruia. Non-dealerul licitează primul.

## Desfășurarea unei ture

`pas` sau `licit 2/3/4` (puncte angajate); dealerul poate licita cel puțin la fel (la egalitate îl ia) sau da pas. Licitatorul conduce; culoarea primei cărți e atu. Răspunzi la culoare sau tai cu atu. Patru puncte: High, Low, Jack, Game.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `licit 3`
* `Ah`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Licitatorul care nu atinge contractul pierde cât a licitat (scorul poate coborî sub 0). Dacă ambii ajung la 11, câștigă licitatorul. Cel mult 30 de mâini (apoi scorul mai mare). Game: A=4, K=3, Q=2, J=1, 10=10.
