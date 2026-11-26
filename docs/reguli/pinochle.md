# Pinochle (`pinochle`)

Pinochle pentru doi jucători, cu 48 de cărți și combinații anunțate.

## Scop

Primul care ajunge la 1000 de puncte (din combinații și levate).

## Pregătire

48 de cărți (9 10 J Q K A, câte două exemplare pe culoare), 12 fiecăruia; următoarea carte se întoarce (atu) și rămâne la fundul stocului. Non-dealerul conduce.

## Desfășurarea unei ture

Câștigătorul unei levate poate anunța o combinație (`anunt nunta h`, `anunt regala`, `anunt secventa`, `anunt dix`, `anunt pinochle`, `anunt asi`) sau spune `fara`; apoi se trage din stoc. O combinație nouă trebuie să conțină cel puțin o carte neanunțată. După golirea stocului se urmează culoarea și se bate dacă se poate; nu se mai anunță. La final: A, 10, K din levate și ultima levată valorează 10.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Exemple de mutări folosite de această implementare:

* `Ah`
* `anunt nunta h`
* `fara`
* `urmatoarea`

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Nu există schimbul lui 9 de atu cu cartea întoarsă. Valori: dix 10, căsătorie 20, regală 40, secvență 150, pinochle 40 (dublu 300), patru ași 100, patru popi 80, patru dame 60, patru valeți 40 (dublate 1000/800/600/400). Cine atinge 1000 prin combinații câștigă pe loc.
