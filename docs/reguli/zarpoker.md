# Poker de zaruri (`zarpoker`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zarpoker --seed 1`. Sursa implementării: `polytabletop-games/src/zarpoker.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 7 runde, 5 zaruri, până la 3 aruncări (arunc; tine 3 3 5 sau tine -; fin). Mâini de poker: cinci la fel > careu > full > scară > trei > două perechi > pereche > nimic. Mâna terminată a primului jucător rămâne acoperită până termină al doilea; cine are mâna mai bună ia punctul rundei.

## Scop

Câștigi mai multe runde (mâna de poker mai bună din 5 zaruri).

## Pregătire

7 runde, 5 zaruri, până la 3 aruncări pe jucător și rundă. Variantă originală cu mâini de poker.

## Desfășurarea turei

Fiecare jucător, pe rând, face o mână: arunci (`arunc`, forțată), apoi ții unele zaruri (`tine 3 3 5`, `tine -`) și rearunci restul, sau te oprești (`fin`). După a treia aruncare te oprești.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`.
* `tine 3 3 5`, `tine -`.
* `fin`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (16): `fin`, `tine -`, `tine 2`, `tine 3`, `tine 2 3`, `tine 5`, `tine 2 5`, `tine 3 5`, `tine 2 3 5`, `tine 5 5` …. Introduci `tine 2 3 5`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Mâinile, de la cea mai slabă: nimic (cel mai mare zar), pereche, două perechi, trei la fel, scară (1-2-3-4-5 sau 2-3-4-5-6), full, careu, cinci la fel. Egalitățile se rup după valorile zarurilor (grupul mai mare întâi).

## Punctaj

Câștigătorul rundei ia 1 punct; remiza nu dă puncte.

## Sfârșitul jocului

După 7 runde; câștigă cine are mai multe puncte.

## Note despre AI

Mâna terminată a primului jucător rămâne acoperită până termină al doilea. Aruncarea e mutare forțată. AI-ul citește doar zarurile lui și punctele rundelor încheiate; nu se uită la mâna acoperită a adversarului.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă originală de poker cu zaruri; clasamentul standard al mâinilor.
