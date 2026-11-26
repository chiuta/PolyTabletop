# Roiul (insecte pe masă) (`roiul`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Hive”. Pornire: `polytabletop play roiul --seed 1`. Sursa implementării: `polytabletop-games/src/roiul.rs`.

> Reguli: Fără tablă: piesele formează un roi pe hexagoane. Fiecare are regină (R), 2 gândaci (G), 2 păianjeni (P), 2 lăcuste (L), 2 furnici (F). Piesele noi ating doar piese proprii; regina se pune cel târziu la tura a 4-a; roiul nu se rupe niciodată; piesele alunecă pe margine. Câștigi înconjurând regina adversă. Mutări: +R@0,0 (pune), 0,0>1,-1 (mută), pas.
