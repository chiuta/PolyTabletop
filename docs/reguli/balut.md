# Balut (`balut`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play balut --seed 1`. Sursa implementării: `polytabletop-games/src/q5_dice.rs, balut.rs`.

> Rezumat din `polytabletop list`: Joc filipinez cu zaruri: 5 zaruri, până la 3 aruncări pe tură, 7 categorii completate o dată (patru, cinci, sase, scara 50, casa, sansa, balut 30). 7 ture fiecare; câștigă cine are mai multe puncte. Mutări: arunc, tine 6 6, tine -, scor sase.

## Scop

Aduni mai multe puncte după 7 ture, completând cele 7 categorii.

## Pregătire

5 zaruri, 7 categorii: `patru`, `cinci`, `sase` (zarurile de acea valoare), `scara` 50, `casa` (trei + pereche, suma), `sansa` (suma), `balut` (cinci la fel) 30.

## Desfășurarea turei

`arunc` (de până la 3 ori), `tine 6 6` (păstrezi zaruri, rearunci restul) sau `tine -`, apoi `scor <categorie>`; dacă nu se potrivește, scrii 0.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `arunc`
* `tine 6 6`
* `scor sase`

## Simplificări și note

Fără pariul pe numărul de balut-uri. Aruncările sunt mutări separate; AI-ul decide pe baza speranței categoriilor libere.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
