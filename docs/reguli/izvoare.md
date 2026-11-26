# Izvoare (stive conectate 7×7) (`izvoare`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „DVONN”. Pornire: `polytabletop play izvoare --seed 1`. Sursa implementării: `polytabletop-games/src/izvoare.rs`.

> Reguli: Tablă 7×7 cu 3 izvoare neutre (b6, d4, f2). Faza 1: pe rând pui câte o piesă (23 fiecare). Faza 2: o stivă condusă de tine se mută pe linie dreaptă exact cât e înălțimea ei și aterizează pe altă stivă (se contopesc). Stivele neconectate (pe 8 direcții) de un izvor dispar. La final câștigă cine conduce mai multe piese proprii. Mutări: c3 (pune), c3-e5, pas.
