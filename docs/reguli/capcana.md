# Capcana (variantă originală) (`capcana`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play capcana --seed 1`. Sursa implementării: `polytabletop-games/src/capcana.rs`.

> Rezumat din `polytabletop list`: Bluf și licitație cu discuri ascunse: 3 flori + 1 capcană. Pui discuri (pun floare / pun capcana), apoi licitezi (licitez N) câte discuri poți întoarce fără capcană; celălalt ridică sau dă pas. Provocatorul întoarce întâi ale lui, apoi ale adversarului (intorc). 2 reușite câștigă; capcana te costă un disc.

## Scop

Câștigi 2 reușite (provocări fără capcană) înaintea adversarului.

## Pregătire

Fiecare jucător are 4 discuri: 3 flori și 1 capcană.

## Desfășurarea turei

Faza de așezare: pui pe rând câte un disc cu fața în jos pe teancul tău. După ce ambii au pus cel puțin unul, poți în loc de așezare să deschizi licitația: `licitez N` = pretinzi că poți întoarce N discuri fără să dai de o capcană. Celălalt ridică sau dă pas; cine rămâne cu licitația cea mai mare e provocatorul. Provocatorul întoarce (`intorc`, forțată) întâi discurile lui, de sus în jos, apoi ale adversarului.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `pun floare`, `pun capcana`.
* `licitez N` (și ridicarea `licitez M`), `pas`.
* `intorc` (forțată).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `pun floare`, `pun capcana`. Introduci `pun floare`; răspunsul calculatorului (în ordine): `pun floare`.
2. Mutări legale (4): `pun floare`, `pun capcana`, `licitez 1`, `licitez 2`. Introduci `licitez 1`; răspunsul calculatorului (în ordine): `licitez 2`.
3. Mutări legale (1): `pas`. Introduci `pas`; răspunsul calculatorului (în ordine): `intorc`; `intorc`; `pun floare`.

## Reguli speciale și variante alese

* Fără capcană = un punct (2 puncte câștigă meciul). Cu capcană = pierzi un disc: dacă a fost capcana lui, alege el ce disc pierde; dacă a fost a adversarului, pierde unul la întâmplare. Fără discuri = ești eliminat.
* Runda următoare o începe provocatorul.

## Punctaj

Un punct pe provocare reușită; 2 puncte câștigă.

## Sfârșitul jocului

Când cineva ajunge la 2 puncte sau adversarul e eliminat.

## Note despre AI

Informație ascunsă: vezi propriul teanc, nu și pe al adversarului. AI-ul nu citește teancul adversarului și alege licitarea după probabilități și după propriul teanc.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Variantă originală de bluf și licitație.
