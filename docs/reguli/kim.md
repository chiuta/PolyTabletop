# Jocul lui Kim (Kim's Game) (`kim`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play kim --seed 1`. Sursa implementării: `polytabletop-games/src/kim.rs`.

> Rezumat din `polytabletop list`: Joc de memorie, 5 runde cu tăvi de 6 până la 14 obiecte: privești tava (acopera), un obiect e scos în secret (scot), apoi ghicești după nume care lipsește; ghicit = 1 punct, greșit = rândul trece. Mutări: acopera, scot, cheie, verific.

## Scop

Aduni mai multe puncte ghicind obiectul care lipsește, în 5 runde.

## Pregătire

O tavă cu 6, 8, 10, 12, 14 obiecte din 30; în faza „privești” toate se văd; rundele încep alternativ.

## Desfășurarea turei

`acopera`, apoi `scot` (mutare separată, obiectul e ales în secret), apoi ghicești numele (`cheie`); `verific` (mutare separată) arată dacă e corect. Un nume greșit se elimină și rândul trece.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `acopera`
* `scot`
* `cheie`
* `verific`

## Simplificări și note

AI-ul nu vede obiectul scos: deduce din tava inițială (publică) pe care o ține minte doar parțial (primele 8 obiecte). Hot-seat: obiectul scos nu apare niciodată pe ecran.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
