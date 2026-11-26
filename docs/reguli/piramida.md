# Piramida (bile pe 4 niveluri) (`piramida`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Pylos”. Pornire: `polytabletop play piramida --seed 1`. Sursa implementării: `polytabletop-games/src/piramida.rs`.

> Reguli: Piramidă pe patru niveluri: 4×4, 3×3, 2×2 și vârful; 15 bile pe jucător. Un loc de sus se poate folosi doar când cele patru locuri de sub el sunt ocupate. O tură: pui o bilă din rezervă sau muți o bilă proprie liberă pe un loc disponibil mai sus. Dacă închizi un pătrat 2×2 propriu pe același nivel, ridici înapoi una sau două bile libere. Pierzi dacă nu poți nici pune, nici muta. Remiză după 250 de semi-mutări. Mutări: b2, b2^f1, b2-a1-c3.
