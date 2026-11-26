# Trenuri de domino (2 jucători) (`trenuri`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Mexican Train”. Pornire: `polytabletop play trenuri --seed 1`. Sursa implementării: `polytabletop-games/src/trenuri.rs`.

> Reguli: Domino dublu-9: dublul 9–9 e „gara”, de unde pornesc trenurile. Fiecare are un tren propriu și există un tren public; trenul adversarului se deschide doar dacă a trebuit să treacă (semn). Pui o piesă al cărei capăt se potrivește cu capătul unui tren permis. Un dublu te obligă să mai pui o piesă pe același tren. Fără piesă potrivită: `trag` din stoc și poți pune piesa, altfel `pas` (semn pe trenul tău). Câștigă cine rămâne primul fără piese, sau cine are mai puține puncte când amândoi trec cu stocul gol. Vezi doar mâna ta. Mutări: 6-4 m (m = trenul meu, p = public, a = al adversarului), trag, pas.
