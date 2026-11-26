# Blackjack (21) (`blackjack`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play blackjack --seed 1`. Sursa implementării: `polytabletop-games/src/blackjack.rs`.

> Rezumat din `polytabletop list`: Tu (J1) contra dealerului (J2, trage până la 17): carte / stai / dublu; blackjack plătește 3:2. Set de 21 de mâini cu pariu de 10 din 100 de jetoane: câștigi dacă termini peste 100. Pașii dealerului sunt mutări forțate: dealer-trage, dealer-sta, decontare.

## Scop

Ai mai mulți jetoane după 21 de mâini decât la început (peste 100).

## Pregătire

Tu (J1) joci contra dealerului (J2, care urmează o regulă fixă). Pachet de 52 de cărți amestecat proaspăt la fiecare mână. Pariu fix de 10 din 100 de jetoane.

## Desfășurarea turei

Primești două cărți, dealerul două (una ascunsă). Poți cere `carte`, `stai` sau `dublu` (o singură carte, miza dublă, doar ca primă decizie). Pașii dealerului sunt mutări forțate ale lui J2.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `carte`, `stai`, `dublu`.
* Pașii dealerului (forțați): `dealer-trage`, `dealer-sta`, `decontare`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (3): `carte`, `stai`, `dublu`. Introduci `carte`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (2): `carte`, `stai`. Introduci `stai`; răspunsul calculatorului (în ordine): `dealer-trage`; `dealer-trage`; `dealer-sta`.
3. Mutări legale (3): `carte`, `stai`, `dublu`. Introduci `dublu`; răspunsul calculatorului (în ordine): `decontare`.

## Reguli speciale și variante alese

* Dealerul trage până la 17 sau mai mult și stă pe orice 17 (inclusiv soft).
* Blackjack (21 din două cărți) plătește 3:2 și pierde doar la blackjack al dealerului (egalitate).
* Fără split, fără asigurare, fără renunțare.

## Punctaj

Jetoanele din bancă (start 100, miză 10; blackjack 3:2; dublu 2×).

## Sfârșitul jocului

După a 21-a mână: câștigi dacă banca ta e peste 100, dealerul câștigă dacă e sub 100, altfel remiză.

## Note despre AI

Pachetul și cartea ascunsă a dealerului sunt în starea jocului; afișajul arată cartea vizibilă a dealerului și „[??]” până la jocul acestuia. Pașii dealerului sunt mutări forțate separate, ca o tragere să nu poată dezvălui următoarea carte evaluării AI. AI-ul (adâncime 1) folosește un model cu pachet infinit și joacă aproximativ „strategia de bază”; nu numără cărți.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără split, asigurare sau renunțare.
* Meci de 21 de mâini cu miză fixă.
