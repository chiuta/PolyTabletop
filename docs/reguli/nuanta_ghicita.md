# Nuanța Ghicită (`nuanta_ghicita`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play nuanta_ghicita --seed 1`. Sursa implementării: `polytabletop-games/src/nuanta_ghicita.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: o grilă 8×8; ținta e cunoscută doar de cel care dă indicii despre bandă (rand b, col b, cel mult 2), celălalt pune un marcaj (ghic r c). Exact 3, vecin 2, la distanță 2 → 1 punct; indicatorul primește 1 (+1 pentru un singur indiciu) dacă ghicitul ia ≥2. 6 runde. Mutări: rand b, col b, gata, ghic r c, verific.
