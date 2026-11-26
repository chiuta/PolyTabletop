# Bid Whist (2 jucători) (`bid_whist`)

Adaptare pentru doi jucători a jocului Bid Whist, cu 54 de cărți și kitty.

## Scop

Primul la +7 puncte (sau adversarul la −7).

## Pregătire

54 de cărți (cu jokerii `JB` mare și `JL` mic), 12 fiecăruia, 6 în kitty, 24 deoparte.

## Desfășurarea unei ture

Licitezi `licit 3…6` sau `pas`. Câștigătorul ia kitty-ul, aruncă 6 (`dis 5h`) și declară `sus h` (Uptown), `jos h` (Downtown), `fara sus` sau `fara jos`. Licitatorul conduce; culoarea se urmează. Dacă ambii dau pas, se reîmparte (`reimparte`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `licit 4`
* `dis 5h`
* `sus h`
* `Ah`
* `reimparte`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Adaptarea la 2 jucători și limitele 3–6 sunt alegeri proprii (jocul tradițional e în patru). Scor: levate peste book (6) dacă licitatorul face contractul, altfel pierde N; dublu fără atu. Regula jokerilor la fără atu e a acestei implementări.
