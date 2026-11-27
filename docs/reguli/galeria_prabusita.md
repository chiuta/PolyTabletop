# Galeria Prăbușită (`galeria_prabusita`)

Categorie: Cooperativ. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului; în jocurile cooperative cei doi formează o echipă contra jocului). Pornire: `polytabletop play galeria_prabusita --seed 1`. Sursa implementării: `polytabletop-games/src/c3_pande.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: cooperativ pe un graf de locuri cu pericole care cresc: doi jucători în echipă, câteva acțiuni pe tură (mers la un vecin, cur = scade pericolul, rit = ritual într-un laborator curat, pas), apoi se trag la întâmplare locuri care se agravează; peste 3 izbucnește și infectează vecinii. Echipa câștigă cu numărul cerut de ritualuri, pierde la prea multe izbucniri sau când se termină rundele (remiză). 9 galerii, 4 acțiuni pe tură, 5 ritualuri de împlinit, cel mult 4 prăbușiri în lanț. Mutări: mers v, cur, rit, pas.
