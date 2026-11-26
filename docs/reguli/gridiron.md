# Gridiron (sport fantastic pe ture) (`gridiron`)

Gridiron: sport fantastic pe ture cu pase și tăvăleli, echipe de 5 pe un teren 12×7.

## Scop

Ai mai multe puncte după 2 reprize, ducând mingea în zona de punct adversă.

## Pregătire

Două echipe de 5 (Pasator, Prinzător, Alergător, Zid, Trântor), zone de punct pe coloanele `a` și `l`; 6 ture pe repriză, 2 reprize. La începutul fiecărui punct `lovitura` pune mingea.

## Desfășurarea unei ture

Pe tura ta fiecare jucător poate face O acțiune; tura se termină cu `fin` sau la pierderea turei (turnover). Zarurile se aruncă la mutarea forțată `arunca`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `A>d4` — mută (și se ridică dacă e la pământ: costă 3 din mv)
* `A>d4 xV` — asalt: mută și tăvălește (o dată pe tură)
* `A xV` — tăvăleală cu un inamic vecin
* `A pass B` — pasă a purtătorului către B (o dată pe tură)
* `A hand B` — predare la vecin
* `A sus` — ridică-te
* `fin` — încheie tura
* `lovitura` — lovitura de începere (forțat)
* `arunca` — aruncă zarurile

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play gridiron --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `lovitura` — lovitura pune mingea la începutul punctului.
2. `E>g6 xW` — Trântorul E asaltează Prinzătorul W la g6.
3. `arunca` — `arunca` rezolvă tăvăleala (W a fost doborât și rănit).

## Reguli speciale și variante alese

Părăsirea unui pătrat vecin cu un inamic cere zar de ESCAPARE; intrarea pe mingea liberă cere zar de PRINDERE; zarul d6: 1 eșuează mereu, 6 reușește mereu; un eșec = căzut (zar de armură) și pierderea turei. Tăvăleală: forță proprie față de forță țintă determină 2–3 zaruri; împins în afara terenului = scos. Doborât: zar de armură 2d6 > armura = rănit și scos. Pasă: zar 3+ la ≤ 3, 4+ la ≤ 6, 5+ la ≤ 8.

## Punctaj

Purtătorul în picioare ajunge în zona de punct adversă = 1 punct; după punct, echipa adversă primește.

## Sfârșitul jocului

După 2 reprize, mai multe puncte câștigă; egal = remiză.

## Note despre AI și informația ascunsă

Informație completă; zarurile se aruncă la `arunca`.

## Diferențe față de regulile tradiționale

Simplificări declarate: fără interceptări, fără urmărire după împingere, împingerea în lanț = doborâre pe loc.
