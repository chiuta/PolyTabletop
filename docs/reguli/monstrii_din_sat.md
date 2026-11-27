# Monștrii din Sat (`monstrii_din_sat`)

Categorie: Cooperativ. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului, ca echipă împotriva jocului). Pornire: `polytabletop play monstrii_din_sat --seed 1`. Sursa implementării: `polytabletop-games/src/monstrii_din_sat.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Jocul e cooperativ: succesul echipei se raportează ca victorie a Jucătorului 1, iar eșecul ca remiză (nu există adversar).

> Variantă originală cooperativă: un monstru înaintează spre voi în fiecare rundă; adunați 3 dovezi și duceți-le în turn (nodul 0). Dacă monstrul vă atinge pierdeți o viață (5 în total); cu fix n lângă el îl alungați la început. Mutări: mov n, fix n, iau, dau, fin; ev încheie runda.
