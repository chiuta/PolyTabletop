# Forme și culori (plăcuțe) (`forme6`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Qwirkle”. Pornire: `polytabletop play forme6 --seed 1`. Sursa implementării: `polytabletop-games/src/forme6.rs`.

> Reguli: 108 de plăcuțe (6 culori × 6 forme × 3). Fiecare ține 6 plăcuțe ascunse; punga e ascunsă. O tură: pui o plăcuță lipită de cele de pe masă astfel încât fiecare rând și coloană (cel mult 6 plăcuțe) să aibă aceeași culoare sau aceeași formă, fără repetiții în linie. Pentru fiecare linie de cel puțin 2 plăcuțe care o conține primești atâtea puncte cât lungimea ei (+6 bonus la 6). Apoi tragi. Sau `schimb R3`: dai o plăcuță în pungă și tragi alta. Jocul se termină când punga e goală și cineva rămâne fără plăcuțe (+6) sau amândoi trec. Câștigă scorul mai mare. Culori R O Y G B P, forme 1–6. Mutări: R3@2,-1, schimb R3, pas.
