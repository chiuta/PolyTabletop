# Șir de cinci (cărți și jetoane) (`sir5`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Sequence”. Pornire: `polytabletop play sir5 --seed 1`. Sursa implementării: `polytabletop-games/src/sir5.rs`.

> Reguli: Tablă 10×10 cu 4 colțuri libere (jolly) și 96 de celule cu cele 48 de cărți ale unui pachet fără valeți, fiecare de două ori. Fiecare ține 7 cărți ascunse. O tură: joci o carte și pui jetonul tău pe una din cele două celule cu acea carte. Valetul de romb sau treflă e jolly (orice celulă liberă); valetul de inimă sau pică scoate un jeton adversar (care nu e într-un șir terminat). O carte moartă se aruncă (`d 7h`); apoi tragi. Un șir = cinci jetoane proprii în linie (colțurile contează); două șiruri pot împărți cel mult o celulă. Primul cu două șiruri câștigă; după 300 de semi-mutări, cine are mai multe. Mutări: 7h@d4, Jd@d4, Js@d4, d 7h.
