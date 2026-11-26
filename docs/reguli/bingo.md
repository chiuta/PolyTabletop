# Bingo (75 de bile) (`bingo`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bingo --seed 1`. Sursa implementării: `polytabletop-games/src/bingo.rs`.

> Rezumat din `polytabletop list`: Fișă 5 × 5 (B 1–15, I 16–30, N 31–45 cu centru liber, G 46–60, O 61–75). Tragerea e o mutare (trag), marcarea o alta (marchez); cu o combinație (rând, coloană, diagonală) strigi sau continui. Cine strigă singur câștigă; dacă strigă amândoi = remiză. Fișa AI-ului e ascunsă. Mutări: trag, marchez, strig, continui.

## Scop

Ești singurul care strigă o combinație câștigătoare (rând, coloană, diagonală).

## Pregătire

Fișă 5 × 5: B 1–15, I 16–30, N 31–45 (centru liber), G 46–60, O 61–75. Numerele se trag dintr-un sac amestecat cu generatorul jocului. Fișa AI-ului este ascunsă până la final.

## Desfășurarea turei

Tragerea este o mutare separată (`trag`, făcută de Jucătorul 1 pentru ambii). După fiecare tragere, cine are numărul pe fișă trebuie să `marchez`; după marcare, cine are o combinație poate `strig` sau `continui`. Ordinea: J1 marchează, J2 marchează, J1 strigă?, J2 strigă?.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trag`, `marchez`, `strig`, `continui`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `trag`. Introduci `trag`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Motorul este comun cu Loteria (joc separat).

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine strigă singur câștigă; dacă strigă amândoi în aceeași tragere sau dacă sacul se golește fără strigăt = remiză.

## Note despre AI

Informație ascunsă: afișajul arată doar fișa Jucătorului 1. AI-ul își citește doar fișa lui și strigă când are combinație. Tragerea fiind mutare separată, AI-ul nu vede ce urmează.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
