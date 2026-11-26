# Castelul Capricios (constructorul fixează prețurile) (`castelul_capricios`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play castelul_capricios --seed 1`. Sursa implementării: `polytabletop-games/src/castelul_capricios.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: Constructorul-șef (alternează) fixează prețurile 1/2/3 ale celor 3 camere (prec a,b,c); celălalt alege primul (ia l / nu) și plătește constructorului; apoi constructorul poate lua una din rămase plătind banca. Scor: pătratul numărului de camere pe fel plus jumătate din valori. Mutări: ofer n, pas, prec a,b,c, ia l, nu, trag.
