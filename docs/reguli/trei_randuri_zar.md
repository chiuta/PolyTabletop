# Trei Rânduri cu Zaruri (`trei_randuri_zar`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play trei_randuri_zar --seed 1`. Sursa implementării: `polytabletop-games/src/trei_randuri_zar.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_C5.tsv`.

> Variantă originală: o foaie cu 3 rânduri de 8 căsuțe. Cel la rând aruncă trei zaruri și alege care se adună (dados m); apoi amândoi scriu suma într-o căsuță (pun r c): crescător pe rând, fără număr repetat pe coloană. Cine nu poate sau nu vrea să scrie (pas) primește o penalizare dacă e la rând. Rând complet +6, coloană completă +3. Mutări: arunc, dados m, pun r c, pas.
