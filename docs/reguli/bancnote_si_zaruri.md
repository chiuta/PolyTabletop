# Bancnote și zaruri (`bancnote_si_zaruri`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bancnote_si_zaruri --seed 1`. Sursa implementării: `polytabletop-games/src/zr_loc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de majoritate: 8 zaruri fiecare, 6 mese numerotate cu bancnote aleatoare ×1–5. La fiecare tură arunci zarurile rămase și pui TOATE zarurile cu o valoare la masa cu același număr. La final de rundă masa e câștigată de cine are mai multe zaruri (egal = nimeni). 4 runde. Mutări: tira, pon v loc, pas, ronda.

Detalii: Recompensele se extrag la începutul fiecărei runde și sunt vizibile.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira` (sau echivalentul ei), deci AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en Banknotes and Dice.
