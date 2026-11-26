# Ronda (Maroc / Spania) (`ronda`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ronda --seed 1`. Sursa implementării: `polytabletop-games/src/ronda.rs`.

> Rezumat din `polytabletop list`: Pachet spaniol de 40 de cărți, 4 pe masă, 3 în mână. Joci o carte: dacă pe masă e același rang, iei toate cărțile acelui rang și apoi, în șir, rangurile următoare prezente. Puncte: ronda (pereche în mână) 1, trio 5, missa (masă golită) 1, peste 20 de cărți 1; meci până la 12. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: 5o, Rb, imparte, urmatoarea.

## Scop

Aduni 12 puncte.

## Pregătire

Pachet spaniol de 40 de cărți: culori `o` (oros), `c` (copas), `e` (espadas), `b` (bastos); ranguri 1–7, S (sota), C (caballo), R (rey). 4 cărți pe masă, 3 în mână; următoarele împărțiri de câte 3 cu mutarea forțată `imparte`.

## Desfășurarea turei

Joci o carte. Dacă pe masă sunt cărți de același rang, le iei pe toate, apoi și cărțile cu rangurile următoare prezente, în șir (după 7 urmează S). Altfel cartea rămâne pe masă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Cartea din mână: `5o`, `Rb`.
* `imparte`, `urmatoarea` (mutări forțate).

## Reguli speciale și variante alese

* Ronda (pereche în mână la împărțire) = 1 punct, trio = 5 puncte, acordate automat.
* Missa: golești masa = 1 punct (nu la ultima carte a rundei).

## Punctaj

Ronda 1, trio 5, missa 1, peste 20 de cărți la finalul rundei 1.

## Sfârșitul jocului

Meciul se joacă până la 12 puncte.

## Note despre AI

Informație ascunsă; evaluarea citește doar informația publică. Alpha-beta cu adâncime 1.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; mâna afișată este a jucătorului la mutare, cu un banner de predare a ecranului.

## Diferențe față de regulile tradiționale

* Variantă simplificată: fără anunțuri, fără bonusul pentru capturarea ultimei cărți jucate de adversar și fără alte puncte regionale; regulile Ronda variază mult.
