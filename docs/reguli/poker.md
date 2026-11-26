# Poker (Texas hold'em, heads-up) (`poker`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play poker --seed 1`. Sursa implementării: `polytabletop-games/src/poker.rs`.

> Rezumat din `polytabletop list`: 20 de mâini, blinduri fixe 10/20, 1000 de jetoane, raise fix (20 / 40 de la turn), maxim 3 raise-uri pe rundă. Câștigă cine are mai multe jetoane. Mutări: fold, call/check, raise; pașii de împărțire sunt forțați: flop, turn, river, showdown, decontare.

## Scop

Ai la final mai multe jetoane după 20 de mâini.

## Pregătire

Texas hold'em heads-up, fixed-limit: 52 de cărți, două cărți private fiecare, cinci cărți comune. Fiecare pornește cu 1000 de jetoane; blindurile sunt fixe, 10 (mic, butonul) și 20 (mare). Butonul alternează și începe cu Jucătorul 1.

## Desfășurarea turei

Runde de pariuri: preflop, flop (3 cărți), turn, river. Heads-up, butonul acționează primul înainte de flop și ultimul după.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `fold`, `call` sau `check`, `raise`.
* Pașii de împărțire (forțați): `flop`, `turn`, `river`, `showdown`, `decontare`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `fold`, `call`, `raise`. Introduci `fold`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `decontare`. Introduci `decontare`; răspunsul calculatorului (în ordine): `call`.
3. Mutări legale (2): `check`, `raise`. Introduci `raise`; răspunsul calculatorului (în ordine): `call`; `flop`.

## Reguli speciale și variante alese

* Raise fix: 20 înainte de flop și la flop, 40 la turn și river; cel mult 3 raise-uri pe rundă.
* Stack scurt = all-in; jetoanele neacoperite se returnează; egalitatea împarte potul.
* `showdown`/`decontare` împart și mâna următoare.

## Punctaj

Jetoanele câștigate din pot.

## Sfârșitul jocului

După 20 de mâini, sau mai devreme dacă un jucător rămâne fără jetoane. Câștigă cine are mai multe jetoane (egal = remiză).

## Note despre AI

Pachetul și ambele mâini sunt în starea jocului; afișajul arată cărțile tale și „[??] [??]” pentru adversar. Pașii de împărțire sunt mutări forțate separate, ca acțiunea AI-ului să nu dezvăluie o carte evaluării. AI-ul (adâncime 1) estimează echitatea doar din cărțile lui și masa; fără modelarea adversarului și fără bluff. Poate fi exploatat.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Heads-up, pariuri fixe (fixed-limit), 20 de mâini; fără no-limit.
