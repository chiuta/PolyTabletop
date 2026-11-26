# Perechi în piramidă (duel) (`perechi_duel`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Mahjong solitaire”. Pornire: `polytabletop play perechi_duel --seed 1`. Sursa implementării: `polytabletop-games/src/perechi_duel.rs`.

> Reguli: 36 de plăcuțe (9 modele × 4) pe trei niveluri: 6×4 jos, 4×2 la mijloc, 2×2 sus. O plăcuță e liberă dacă nu are nimic deasupra și măcar o latură stânga/dreapta liberă. Pe rând, iei o pereche de plăcuțe libere cu același model și o păstrezi (1 punct). Când nu mai există perechi libere, câștigă cine are mai multe perechi. Plăcuțe: nivel + coloană + rând, ex. 0a1. Mutări: 0a1+0f4.
