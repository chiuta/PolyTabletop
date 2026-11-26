# Flipper (duel de scor) (`flipper_duel`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Flipper (Pinball)”. Pornire: `polytabletop play flipper_duel --seed 1`. Sursa implementării: `polytabletop-games/src/flipper_duel.rs`.

> Reguli: Masă de 7 coloane (0–6) și 12 rânduri, cu bumpere (b) și ținte (T, o lovitură pe bilă). Fiecare are 3 bile, alternativ. Lansezi (coloană 0–6 și putere 1–3: `3/2`); bila urcă drept cât dă puterea, apoi coboară în diagonală, ricoșând de pereți și bumpere (100 de puncte; ținta 300). Jos o poți relansa cu paleta stângă (`st`, coloanele 0–3) sau dreaptă (`dr`, coloanele 3–6), de cel mult 4 ori; `nu` o lasă să cadă. Câștigă scorul mai mare. Mutări: 3/2, st, dr, nu.
