# Crag (`crag`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play crag --seed 1`. Sursa implementării: `polytabletop-games/src/q5_dice.rs, crag.rs`.

> Rezumat din `polytabletop list`: Joc american cu 3 zaruri și 13 categorii, 13 ture fiecare: arunci, poți rearunca orice zaruri o dată, apoi scrii o categorie liberă (unu..sase, trei-la-fel 25, treisprezece 26, scări 20, crag 50). Mutări: arunc, tine 6 6, tine -, scor crag.

## Scop

Aduni mai multe puncte după 13 ture.

## Pregătire

3 zaruri; 13 categorii (`unu`…`sase`, `trei-la-fel`, `treisprezece`, `scara-mica`, `scara-mare`, `scara-impara`, `scara-para`, `crag`).

## Desfășurarea turei

`arunc`, apoi `tine 6 6` (păstrezi zarurile și rearunci restul) sau `tine -`, apoi `scor <categorie>`; dacă zarurile nu satisfac categoria, primește 0.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `arunc`
* `tine 6 6`
* `scor crag`

## Simplificări și note

Aruncările sunt mutări separate; AI-ul nu le vede în avans.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
