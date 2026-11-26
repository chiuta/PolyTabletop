# Piesa dăruită (4×4) (`dar4`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Quarto”. Pornire: `polytabletop play dar4 --seed 1`. Sursa implementării: `polytabletop-games/src/dar4.rs`.

> Reguli: 16 piese distincte (mărime, umplere, formă, culoare), tablă 4×4. În fiecare tură pui piesa primită de la adversar și alegi piesa pe care i-o dai. Câștigi dacă piesa pusă completează un rând, o coloană, o diagonală sau un pătrat 2×2 de patru piese cu o însușire comună. Tabla plină = remiză. Piesele sunt 0–f (hexazecimal). Mutări: `b3 dă 9`; prima tură: `dă 5`.
