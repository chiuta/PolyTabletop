# Pescuit (Go Fish) (`pescuit`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pescuit --seed 1`. Sursa implementării: `polytabletop-games/src/pescuit.rs`.

> Rezumat din `polytabletop list`: 52 de cărți, 7 în mână. Ceri o valoare pe care o ai (cer 7); adversarul răspunde forțat da (îți dă toate cărțile) sau pescuieste (tragi din stoc; dacă prinzi valoarea cerută mai ceri). Patru la fel formează un set; cele mai multe seturi câștigă. Informație ascunsă: vezi doar mâna ta. Mutări: cer K, da, pescuieste.

## Scop

Ai cele mai multe seturi (patru cărți de aceeași valoare) când se termină cele 13.

## Pregătire

52 de cărți, 7 în mână fiecăruia, restul stoc.

## Desfășurarea turei

Ceri o valoare pe care o ai în mână (`cer 7`). Adversarul răspunde printr-o mutare forțată: `da` (îți dă toate cărțile acelei valori) sau `pescuieste` (nu are: tragi o carte din stoc). După ce primești cărți, sau pescuiești exact valoarea cerută, mai ceri o dată; altfel rândul trece. Mână goală: tragi o carte din stoc.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `cer K` (valoare: 2–10, J, Q, K, A).
* `da`, `pescuieste` (răspunsuri forțate).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (6): `cer 2`, `cer 3`, `cer 6`, `cer 8`, `cer Q`, `cer K`. Introduci `cer 2`; răspunsul calculatorului (în ordine): `da`.
2. Mutări legale (6): `cer 2`, `cer 3`, `cer 6`, `cer 8`, `cer Q`, `cer K`. Introduci `cer 8`; răspunsul calculatorului (în ordine): `pescuieste`; `cer 6`.
3. Mutări legale (1): `da`. Introduci `da`; răspunsul calculatorului (în ordine): `cer 6`.

## Reguli speciale și variante alese

* Patru cărți de aceeași valoare formează un „set” și se pun jos.

## Punctaj

Numărul de seturi.

## Sfârșitul jocului

Când toate cele 13 seturi sunt jos; câștigă cine are mai multe.

## Note despre AI

Informație ascunsă: vezi doar mâna ta. Răspunsul e mutare separată, ca AI-ul să nu vadă mâna adversarului prin simulare; AI-ul cere o valoare după cărțile lui și după ce a cerut adversarul (informație publică).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Go Fish tradițional, două jucători.
