# Bile și inele (hex 37) (`bilinele`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „ZÈRTZ”. Pornire: `polytabletop play bilinele --seed 1`. Sursa implementării: `polytabletop-games/src/bilinele.rs`.

> Reguli: 37 de inele pe un hexagon; rezerva comună are 5 bile albe, 7 gri, 9 negre. Săritura peste o bilă vecină pe un inel gol e obligatorie (lanț cu aceeași bilă). Altfel pui o bilă la alegere pe un inel gol și scoți un inel gol „liber” (marginal); zonele izolate cu toate inelele ocupate se capturează. Câștigi cu 3 de fiecare culoare, sau 4 albe, 5 gri, 6 negre. Mutări: w d4 x a1 (bila w pe d4, scoate a1), g d4, d4xd6xd8.
