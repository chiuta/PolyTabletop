# Focul în Bloc (`focul_in_bloc`)

Categorie: Cooperativ. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului, ca echipă împotriva jocului). Pornire: `polytabletop play focul_in_bloc --seed 1`. Sursa implementării: `polytabletop-games/src/focul_in_bloc.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Jocul e cooperativ: succesul echipei se raportează ca victorie a Jucătorului 1, iar eșecul ca remiză (nu există adversar).

> Variantă originală cooperativă de salvare: 3 victime de dus afară (nodul 0) dintr-o clădire în flăcări; flăcările se întind (la prag trec la vecini), iar pe un loc la prag pierdeți o viață (6 în total). Mutări: mov n, fix n (stinge), iau, dau, fin; ev încheie runda.
