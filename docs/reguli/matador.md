# Matador (domino) (`matador`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play matador --seed 1`. Sursa implementării: `polytabletop-games/src/matador.rs`.

> Rezumat din `polytabletop list`: Domino în care jumătățile se pun cu suma 7 față de capăt (1+6, 2+5, 3+4); matadorii 0-0, 1-6, 2-5, 3-4 se pun oriunde. 7 piese fiecare, tragi dacă nu poți, câștigă cine termină primul. Mutări: 6-1@s=1, trag, pas.

## Scop

Rămâi primul fără piese (sau ai mai puține puncte la blocaj).

## Pregătire

Set dublu-șase de 28 de piese; 7 pentru fiecare, restul formează talonul.

## Desfășurarea turei

Pui o piesă la un capăt al liniei (jumătățile însumează 7 cu capătul, sau un matador), `trag` din talon dacă nu poți, `pas` când talonul e gol.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `6-1`
* `3-4@s=4`
* `trag`

## Simplificări și note

O singură mână. Informația ascunsă (mâna adversarului, talonul) nu e citită de AI; hot-seat arată doar mâna jucătorului la mutare.

## Informație ascunsă

Jocul are informație ascunsă: `render` arată doar ce are voie să vadă jucătorul care privește (în hot-seat, doar jucătorul la mutare), iar AI-ul nu citește informația ascunsă a adversarului.
