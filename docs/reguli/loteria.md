# Lotería (imagini) (`loteria`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play loteria --seed 1`. Sursa implementării: `polytabletop-games/src/bingo.rs`.

> Rezumat din `polytabletop list`: Tablă 4 × 4 cu 16 din 54 de imagini (set propriu, nu cel comercial). Tragi o imagine (trag), o marchezi dacă o ai (marchez); combinații: rând, coloană, diagonală, cele patru colțuri sau pătratul din centru; strigi sau continui. Fișa AI-ului e ascunsă. Mutări: trag, marchez, strig, continui.

## Scop

Ești singurul care strigă o combinație câștigătoare pe tabla ta.

## Pregătire

Tablă 4 × 4 cu 16 din cele 54 de imagini proprii (nu setul comercial). Imaginile se trag dintr-un sac amestecat cu generatorul jocului. Fișa AI-ului este ascunsă până la final.

## Desfășurarea turei

Tragerea este o mutare separată (`trag`, făcută de Jucătorul 1 pentru ambii). Cine are imaginea pe tablă trebuie să `marchez`; apoi cine are o combinație poate `strig` sau `continui`. Ordinea: J1 marchează, J2 marchează, J1 strigă?, J2 strigă?.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trag`, `marchez`, `strig`, `continui`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `marchez`. Introduci `marchez`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Combinații: rând, coloană, diagonală, cele patru colțuri sau pătratul din centru 2 × 2.
* Același motor ca Bingo.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine strigă singur câștigă; dacă strigă amândoi în aceeași tragere sau dacă sacul se golește fără strigăt = remiză.

## Note despre AI

Informație ascunsă: afișajul arată doar tabla Jucătorului 1 (a AI-ului apare la final). AI-ul își citește doar tabla lui și strigă când are combinație; nu vede ce urmează fiindcă `trag` este mutare separată.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Set de 54 de imagini propriu, cu nume proprii.
