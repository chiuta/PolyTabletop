# Cele șase ținte (zaruri) (`bunco`)

Joc de zaruri cu titlu propriu, inspirat de jocul tradițional american Bunco (nume comercial).

## Scop

Câștigi cele mai multe din cele 6 runde.

## Pregătire

Șase runde; în runda N ținta e fața N. Trei zaruri.

## Desfășurarea unei ture

Arunci (`arunc`): fiecare zar egal cu ținta = 1 punct, trei egale cu ținta (bunco) = 21 și încheie runda, alt triplet = 5. Dacă ai punctat, arunci din nou; fără puncte, tura trece.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `arunc`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Runda se încheie la 21 de puncte. După 6 runde câștigă cine a luat mai multe runde, la egalitate totalul de puncte. Nu există decizii: totul e șansă (cum e și în jocul tradițional).
