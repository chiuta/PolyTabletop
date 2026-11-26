# Inele și semne (hex) (`inele`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „YINSH”. Pornire: `polytabletop play inele --seed 1`. Sursa implementării: `polytabletop-games/src/inele.rs`.

> Reguli: Tablă hexagonală cu 61 de puncte. Faza 1: fiecare pune 3 inele (r d5). Apoi o tură: alegi un inel propriu, lași un semn al tău în locul lui și muți inelul în linie dreaptă pe un punct gol, sărind peste șiruri de semne (care se întorc) și oprindu-te imediat după ele. Un rând de 4 semne consecutive se scoate, iar proprietarul scoate unul din inelele lui (x d5). Cine scoate 2 inele câștigă. Mutări: r d5, d5>d8, x d5.
