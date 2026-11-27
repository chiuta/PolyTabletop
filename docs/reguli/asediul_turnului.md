# Asediul Turnului (`asediul_turnului`)

Categorie: Cooperativ. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului, ca echipă împotriva jocului). Pornire: `polytabletop play asediul_turnului --seed 1`. Sursa implementării: `polytabletop-games/src/asediul_turnului.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Jocul e cooperativ: succesul echipei se raportează ca victorie a Jucătorului 1, iar eșecul ca remiză (nu există adversar).

> Variantă originală cooperativă de apărare: monștri apar pe inelul exterior și înaintează spre turn (nodul 0) câte un pas pe rundă; fix n ucide un monstru din locul curent sau vecin. Ajunși la turn ei îi cresc destinul (maxim 6). Ucideți 8 monștri în 16 runde. Mutări: mov n, fix n, fin; ev încheie runda.
