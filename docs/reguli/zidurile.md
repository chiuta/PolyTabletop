# Zidurile Mișcătoare (labirint cu plăci culisante) (`zidurile`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zidurile --seed 1`. Sursa implementării: `polytabletop-games/src/zidurile.rs`.

> Rezumat din `polytabletop list`: Variantă originală: tablă 7×7 cu plăci mobile; introduci placa de rezervă (mete N2 r1) apoi muți pionul pe un drum deschis (mergi c3) ca să iei comorile în ordine, apoi acasă. Nu poți anula inserția adversarului. După 100 de ture, cine are mai multe comori.

## Scop

Ia comorile tale în ordine fixă și întoarce-te primul la colțul de start.

## Pregătire

Tablă 7×7: 16 plăci fixe (rânduri și coloane pare, numărând de la 0) și 33 de plăci mobile + 1 de rezervă (12 drepte, 16 colțare, 6 în T). 12 comori (A–L); fiecare jucător are 6 de găsit, într-o ordine fixă și publică. Colțurile de start: J1 a1, J2 g7.

## Desfășurarea turei

Două părți: (1) `mete N2 r1` introduce placa de rezervă, rotită de `r` ori 90° în sensul acelor de ceasornic, pe marginea N, S, V sau E la poziția 2, 4 sau 6; placa scoasă din partea opusă devine noua rezervă. (2) `mergi c3` mută pionul pe orice placă legată de el printr-un drum deschis (pe locul curent = rămâi).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `mete N2 r1` (margine N/S/V/E + poziție 2/4/6 + rotație r0–r3).
* `mergi c3` (coloana a–g, rândul 1–7 de sus).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (24): `mete N2 r0`, `mete N2 r1`, `mete N4 r0`, `mete N4 r1`, `mete N6 r0`, `mete N6 r1`, `mete S2 r0`, `mete S2 r1`, `mete S4 r0`, `mete S4 r1` …. Introduci `mete N2 r0`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (3): `mergi a1`, `mergi a2`, `mergi b2`. Introduci `mergi a2`; răspunsul calculatorului (în ordine): `mete E4 r1`; `mergi c4`.
3. Mutări legale (22): `mete N2 r0`, `mete N2 r1`, `mete N4 r0`, `mete N4 r1`, `mete N6 r0`, `mete N6 r1`, `mete S2 r0`, `mete S2 r1`, `mete S4 r0`, `mete S4 r1` …. Introduci `mete E6 r1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Nu poți anula direct inserția adversarului (inserția opusă celei precedente).
* Pionii de pe rândul/coloana împinsă se mută odată cu plăcile; cel scos de pe tablă reapare pe placa introdusă.

## Punctaj

Comorile strânse.

## Sfârșitul jocului

Cine ajunge primul acasă cu toate comorile câștigă. După 100 de ture (50 fiecare) câștigă cine a strâns mai multe comori; egal = remiză.

## Note despre AI

Informație completă. AI-ul face căutare la 2 plies (inserție + mers) cu euristică de distanță până la țintă (BFS pe drumuri deschise).

Stil AI în cod: alpha-beta cu adâncime fixă 2.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Variantă originală cu plăci culisante; reguli proprii.
