# Vinovatul Tăinuit (`vinovat_tainuit`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vinovat_tainuit --seed 1`. Sursa implementării: `polytabletop-games/src/vinovat_tainuit.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_C5.tsv`.

> Variantă originală: un vinovat ascuns dintre 18 suspecți (pălărie, haină, încălțăminte). Ținta nu o știe nimeni; pe rând puneți întrebări da/nu despre o trăsătură (intreb X=v, cel mult 4 pe jucător, răspunsurile sunt publice) sau ghiciți (ghic k): corect +2, greșit +1 pentru celălalt. 6 runde. Mutări: intreb X=v, ghic k, raspuns, verific.

Informație ascunsă: da (HOTSEAT_SAFE); fiecare jucător vede doar ce îi aparține.
