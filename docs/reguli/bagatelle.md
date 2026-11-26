# Bagatelle (`bagatelle`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bagatelle --seed 1`. Sursa implementării: `polytabletop-games/src/bagatelle.rs`.

> Rezumat din `polytabletop list`: Masă înclinată cu cuie și 9 căsuțe: alegi culoarul de lansare (1–5) și puterea (1–3), apoi lași bila să curgă; loviturile tari merg mai drept, dar pot sări peste masă. Nouă bile fiecare; câștigă cine are mai multe puncte. Mutări: tir 3 2, lansez.

## Scop

Aduni mai multe puncte din cele 9 bile ale tale.

## Pregătire

Masă cu 7 rânduri de cuie și 9 căsuțe (0, 5, 1, 3, 10, 3, 1, 5, 0 puncte); căsuțele de 10 și 5 au capacitate limitată.

## Desfășurarea turei

`tir C P` (culoar 1–5, putere 1–3), apoi `lansez` (mutare separată: bila curge probabilistic). Probabilitățile sunt alegerea noastră.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `tir 3 2`
* `lansez`

## Simplificări și note

AI: evaluează speranța exactă a loviturii alese, nu rezultatul ei.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
