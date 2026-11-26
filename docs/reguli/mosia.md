# Moșia (roluri, plantații și clădiri) (`mosia`)

Moșia: joc de alegere de roluri, plantații și clădiri pentru doi jucători (mecanică generică; informație completă).

## Scop

Ai cele mai multe puncte (expediere + clădiri + monede/4).

## Pregătire

Culturi: porumb `p`, indigo `i`, zahăr `z`, tutun `t`, cafea `c` (prețuri 0, 1, 2, 3, 4). Cel mult 8 plantații și 6 mărfuri din fiecare fel. Șapte roluri: cultivator, constructor, producator, negustor, capitan, vistier, zidar.

## Desfășurarea unei ture

Într-o rundă fiecare jucător alege 3 roluri pe rând (începe alternativ); rolul rămas primește +1 monedă, luată de cine îl alege data viitoare. Toți execută acțiunea rolului, cine l-a ales are un privilegiu.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `rol cultivator` — alegi rolul
* `planta 2` — cultivator: iei plantația vizibilă 2; `cariera` = ia o carieră (doar cine a ales rolul); `sari`
* `construieste targ` — constructor: cumperi o clădire
* `vinde z` — negustor: vinzi o marfă
* `expediaza z 1` — capitan: expediezi marfa z pe nava 1
* `sari` — nu faci nimic

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play mosia --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `rol constructor` — alegi rolul constructor.
2. `construieste targ` — construiești Târgul.
3. `rol vistier` — alegi rolul vistier.

## Reguli speciale și variante alese

`constructor`: costul scade cu numărul de cariere (cel mult 3), cu 1 pentru cine a ales rolul și cu 1 pentru `hala`. Clădirile sunt unice. `producator`: toate plantațiile dau mărfuri (+1 pentru fabrica culturii). `negustor`: +1 la `targ`, +1 pentru cine a ales rolul; în piață încap 4 mărfuri. `capitan`: două nave (mărimi 4 și 6), un singur fel de marfă pe navă; fiecare marfă expediată = 1 punct (+1 cu `debarcader`); cine poate expedia trebuie. `vistier`: 2 monede pentru cine alege, 1 pentru celălalt. `zidar`: o carieră și o monedă.

## Punctaj

Puncte de expediere + puncte de clădiri + monede/4 (rotunjit în jos).

## Sfârșitul jocului

După runda în care cineva are 5 clădiri, s-au terminat clădirile, punctele de expediere (40) sau runda 14.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri după pregătire; AI-ul vede toată starea.

## Diferențe față de regulile tradiționale

Variantă compactă „fără colonii, ci doar plantații care produc direct”; mecanica rolurilor este generică.
