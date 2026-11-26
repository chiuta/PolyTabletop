# Animale (mini-șah 3×4) (`animale`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Dōbutsu Shōgi”. Pornire: `polytabletop play animale --seed 1`. Sursa implementării: `polytabletop-games/src/animale.rs`.

> Reguli: Tablă 3×4: leu (L, orice direcție), girafă (G, ortogonal), elefant (E, diagonal) și pui (P, înainte; ajuns pe ultimul rând devine cocoș C). Piesele luate trec în mâna ta și pot fi puse pe orice celulă goală. Câștigi luând leul adversarului sau ajungând cu leul pe ultimul rând într-o celulă neatacată. Remiză după 100 de semi-mutări. Mutări: b1-b2, G@b2 (pune din mână).
