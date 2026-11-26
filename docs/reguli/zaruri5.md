# Cinci Zaruri (13 categorii) (`zaruri5`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zaruri5 --seed 1`. Sursa implementării: `polytabletop-games/src/zaruri5.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 13 runde, 5 zaruri, până la 3 aruncări. Aruncarea e mutare forțată (arunc); între aruncări ții zaruri (tine 3 3 5, tine -) sau scrii o categorie (scor casa). Bonus 35 la 63 sus; fără regula joker. Câștigă totalul mai mare.

## Scop

Ai totalul cel mai mare după 13 runde.

## Pregătire

Fiecare jucător are 13 runde; 5 zaruri; 13 categorii de scor, fiecare folosită o dată.

## Desfășurarea turei

În fiecare rundă: până la 3 aruncări. Aruncarea este mutare forțată (`arunc`). După prima și a doua aruncare poți ține unele zaruri (`tine 3 3 5`, `tine -` = niciunul) și rearunci restul, sau scrii scorul într-o categorie liberă. După a treia aruncare trebuie să scrii; o categorie goală primește 0.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`.
* `tine 3 3 5`, `tine -`.
* `scor casa` (sau `scor trei-la-fel`, `scor sansa` etc.).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (28): `tine -`, `tine 2`, `tine 3`, `tine 2 3`, `tine 5`, `tine 2 5`, `tine 3 5`, `tine 2 3 5`, `tine 5 5`, `tine 2 5 5` …. Introduci `tine 3 5 5 5`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Categorii: unu…sase (suma zarurilor cu acea valoare; bonus 35 dacă suma celor 6 ≥ 63), trei-la-fel și patru-la-fel (suma tuturor zarurilor), casa (3+2: 25), scara-mica (4 consecutive: 30), scara-mare (5 consecutive: 40), cinci-la-fel (50), sansa (suma).

## Punctaj

Suma categoriilor + bonus 35 la 63 sus.

## Sfârșitul jocului

După 13 runde pe jucător; câștigă totalul mai mare.

## Note despre AI

Zarurile sunt aruncate prin mutarea forțată `arunc`, ca AI-ul să nu vadă zarurile viitoare. `evaluate` folosește doar scorurile, zarurile curente și cele ținute (speranța exactă a celei mai bune categorii după aruncarea zarurilor rămase).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Fără regula „joker” la a doua cinci-la-fel și fără bonus de 100.
