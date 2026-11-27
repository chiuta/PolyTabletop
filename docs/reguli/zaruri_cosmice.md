# Zaruri Cosmice (`zaruri_cosmice`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zaruri_cosmice --seed 1`. Sursa implementării: `polytabletop-games/src/zaruri_cosmice.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_C5.tsv`.

> Variantă originală: fiecare are zaruri-muncitori (3 la început) cu fețele explorare, așezare, producție, livrare, dezvoltare și joker. După arunc, jokerii se fixează (fixez d f), iar un zar poate fi schimbat contra unui credit (ajust d f); gata rezolvă fazele pentru ambii: explorare +1 carte, așezare (2 cărți → lume, +1 glorie), producție (marfă pe lume), livrare (marfă → credit și glorie), dezvoltare (3 cărți → 3 glorie). Apoi recrut (zar nou contra 2 credite) sau fin. 8 runde. Mutări: arunc, fixez d f, ajust d f, gata, recrut, fin.
