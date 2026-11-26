# Culori în șir (turnuri 8×8) (`culori_pas`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Kamisado”. Pornire: `polytabletop play culori_pas --seed 1`. Sursa implementării: `polytabletop-games/src/culori_pas.rs`.

> Reguli: Tablă 8×8 cu opt culori (R O Y G B P K W) pe celule; fiecare are opt turnuri, câte unul din fiecare culoare. J1 mută primul orice turn; apoi culoarea celulei pe care a aterizat ultimul turn hotărăște ce turn mută adversarul. Un turn merge doar înainte, pe verticală sau diagonală, oricât, fără să sară. Câștigi ajungând pe rândul advers. Dacă turnul cerut e blocat, treci; pierde cine trece a doua oară la rând. Mutări: a1-a5, pas.
