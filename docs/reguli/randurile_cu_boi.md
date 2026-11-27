# Rândurile cu Boi (`randurile_cu_boi`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului; în jocurile cooperative cei doi formează o echipă contra jocului). Pornire: `polytabletop play randurile_cu_boi --seed 1`. Sursa implementării: `polytabletop-games/src/c3_nimmt.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: joc de cărți cu rânduri, 2 jucători: pachet 1..104 cu penalizări („boi”); patru rânduri, mâini deschise de 10 cărți, joacă pe rând (joc i): cartea merge la rândul al cărui capăt e cel mai mare număr mai mic decât ea; a 6-a carte dintr-un rând îți aduce cele 5 cărți (boii lor); dacă e mai mică decât toate, alegi rândul de luat (rand r). După 20 de cărți câștigă cine are mai puțini boi. Mutări: joc i, rand r.
