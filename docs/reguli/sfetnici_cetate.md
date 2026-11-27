# Sfetnicii Cetății (`sfetnici_cetate`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sfetnici_cetate --seed 1`. Sursa implementării: `polytabletop-games/src/sfetnici_cetate.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_C5.tsv`.

> Variantă originală: în fiecare rundă ambii aruncă două zaruri (arunc) și, alternativ, ocupă sfetnici cu numerele 1–12 (pun n d: zarul d valorează n, sau d = 3 pentru suma celor două); un sfetnic poate fi ocupat o singură dată pe rundă și dă lemn, piatră, aur, glorie sau soldați. Apoi se construiește (constr b). După rundele 2, 4 și 6 vine o invazie: cine are prea puțini soldați pierde 2 glorie. 6 runde. Mutări: arunc, pun n d, pas, constr b, gata.
